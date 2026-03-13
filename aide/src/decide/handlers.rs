use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::{header, Request, Response, StatusCode};
use axum::response::IntoResponse;
use tokio::sync::Mutex;

use crate::decide::storage::DecideStorage;
use crate::decide::types::*;

pub struct AppState {
    pub storage: DecideStorage,
    pub web_dir: PathBuf,
    pub should_close: bool,
    pub project_root: PathBuf,
}

pub type SharedState = Arc<Mutex<AppState>>;

pub async fn handle_get_items(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.lock().await;

    let pending = match state.storage.load_pending() {
        Ok(Some(p)) => p,
        Ok(None) => {
            return server_error("无法读取待定项数据", "文件不存在或格式错误");
        }
        Err(e) => {
            return server_error("无法读取待定项数据", &e);
        }
    };

    // 转换为 JSON 并为有 location 的 item 添加 source_content
    let mut data: serde_json::Value = match serde_json::to_value(&pending) {
        Ok(v) => v,
        Err(e) => return server_error("序列化失败", &e.to_string()),
    };

    // 移除 _meta
    if let Some(obj) = data.as_object_mut() {
        obj.remove("_meta");
    }

    // 为有 location 的 item 读取 source_content
    if let Some(items) = data.get_mut("items").and_then(|v| v.as_array_mut()) {
        for item in items {
            if let Some(location) = item.get("location") {
                if let Some(file) = location.get("file").and_then(|f| f.as_str()) {
                    let start = location
                        .get("start")
                        .and_then(|s| s.as_i64())
                        .unwrap_or(1);
                    let end = location.get("end").and_then(|e| e.as_i64()).unwrap_or(1);
                    if let Some(content) =
                        read_source_lines(&state.project_root, file, start, end)
                    {
                        item.as_object_mut()
                            .unwrap()
                            .insert("source_content".into(), serde_json::Value::String(content));
                    }
                }
            }
        }
    }

    json_response(StatusCode::OK, &data)
}

pub async fn handle_submit(
    State(state): State<SharedState>,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    // Check body size
    if body.len() > 1024 * 1024 {
        return json_error_response(
            StatusCode::PAYLOAD_TOO_LARGE,
            "请求体过大",
            "单次提交限制 1MB",
        );
    }

    let mut state = state.lock().await;

    let pending = match state.storage.load_pending() {
        Ok(Some(p)) => p,
        Ok(None) => return json_error_response(StatusCode::INTERNAL_SERVER_ERROR, "保存失败", "未找到待定项数据"),
        Err(e) => return json_error_response(StatusCode::INTERNAL_SERVER_ERROR, "保存失败", &e),
    };

    let payload: DecideOutput = match serde_json::from_slice(&body) {
        Ok(p) => p,
        Err(e) => {
            return json_error_response(
                StatusCode::BAD_REQUEST,
                "决策数据无效",
                &e.to_string(),
            );
        }
    };

    if let Err(e) = validate_output(&payload, &pending) {
        return json_error_response(StatusCode::BAD_REQUEST, "决策数据无效", &e);
    }

    if let Err(e) = state.storage.save_result(&payload) {
        return json_error_response(StatusCode::INTERNAL_SERVER_ERROR, "保存失败", &e);
    }

    // 触发关闭
    state.should_close = true;

    let resp = serde_json::json!({
        "success": true,
        "message": "决策已保存"
    });
    json_response(StatusCode::OK, &resp)
}

pub async fn handle_static_file(
    State(state): State<SharedState>,
    req: Request<Body>,
) -> impl IntoResponse {
    let path = req.uri().path();
    let state = state.lock().await;

    let (filename, content_type) = match path {
        "/" | "/index.html" => ("index.html", "text/html; charset=utf-8"),
        "/style.css" => ("style.css", "text/css; charset=utf-8"),
        "/app.js" => ("app.js", "application/javascript; charset=utf-8"),
        _ => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .body(Body::from("Not Found"))
                .unwrap();
        }
    };

    let file_path = state.web_dir.join(filename);
    match fs::read(&file_path) {
        Ok(data) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(Body::from(data))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .body(Body::from(format!("{filename} 不存在")))
            .unwrap(),
    }
}

fn read_source_lines(root: &Path, file_path: &str, start: i64, end: i64) -> Option<String> {
    let full_path = root.join(file_path);
    if !full_path.exists() {
        return None;
    }
    let content = fs::read_to_string(&full_path).ok()?;
    let lines: Vec<&str> = content.lines().collect();
    let start_idx = (start - 1).max(0) as usize;
    let end_idx = (end as usize).min(lines.len());
    if start_idx >= lines.len() {
        return None;
    }
    Some(lines[start_idx..end_idx].join("\n"))
}

fn json_response(status: StatusCode, data: &serde_json::Value) -> Response<Body> {
    let body = serde_json::to_string(data).unwrap_or_default();
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS")
        .header(header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type")
        .body(Body::from(body))
        .unwrap()
}

fn json_error_response(status: StatusCode, error: &str, detail: &str) -> Response<Body> {
    let data = serde_json::json!({
        "error": error,
        "detail": detail
    });
    json_response(status, &data)
}

fn server_error(message: &str, detail: &str) -> Response<Body> {
    json_error_response(StatusCode::INTERNAL_SERVER_ERROR, message, detail)
}
