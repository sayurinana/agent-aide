use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::Router;
use axum::http::{Method, header};
use axum::routing::{get, post};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use crate::core::config::{ConfigManager, get_config_int_or, get_config_string_or};
use crate::core::output;
use crate::decide::handlers::*;
use crate::decide::storage::DecideStorage;

pub struct DecideServer {
    pub root: PathBuf,
    pub storage: DecideStorage,
    pub port: u16,
    pub timeout: u64,
    pub bind: String,
    pub url: String,
    pub web_dir: PathBuf,
}

impl DecideServer {
    pub fn new(root: &Path, web_dir: Option<&Path>) -> Self {
        let default_web_dir = std::env::current_exe()
            .ok()
            .and_then(|e| e.parent().map(|p| p.join("web")))
            .unwrap_or_else(|| PathBuf::from("web"));

        Self {
            root: root.to_path_buf(),
            storage: DecideStorage::new(root),
            port: 3721,
            timeout: 0,
            bind: "127.0.0.1".into(),
            url: String::new(),
            web_dir: web_dir.map(|p| p.to_path_buf()).unwrap_or(default_web_dir),
        }
    }

    pub fn load_config(&mut self) {
        let cfg = ConfigManager::new(&self.root);
        let config = cfg.load_config();
        self.port = get_config_int_or(&config, "decide.port", 3721) as u16;
        self.timeout = get_config_int_or(&config, "decide.timeout", 0) as u64;
        self.bind = get_config_string_or(&config, "decide.bind", "127.0.0.1");
        self.url = get_config_string_or(&config, "decide.url", "");
    }

    fn find_available_port(&self) -> Option<u16> {
        for offset in 0..10 {
            let port = self.port + offset;
            if TcpListener::bind((self.bind.as_str(), port)).is_ok() {
                return Some(port);
            }
        }
        None
    }

    pub async fn start_daemon(&mut self, pid: u32) -> bool {
        self.load_config();

        let available = match self.find_available_port() {
            Some(p) => p,
            None => return false,
        };
        self.port = available;

        let access_url = if self.url.is_empty() {
            format!("http://localhost:{}", self.port)
        } else {
            self.url.clone()
        };

        if let Err(e) = self.storage.save_server_info(pid, self.port, &access_url) {
            eprintln!("保存服务信息失败: {e}");
            return false;
        }

        let result = self.serve().await;

        self.storage.clear_server_info();
        result
    }

    #[allow(dead_code)]
    pub async fn start_foreground(&mut self) -> bool {
        self.load_config();

        let available = match self.find_available_port() {
            Some(p) => p,
            None => {
                let end_port = self.port + 9;
                output::err(&format!(
                    "无法启动服务: 端口 {}-{end_port} 均被占用",
                    self.port
                ));
                output::info("建议: 关闭占用端口的程序，或在配置中指定其他端口");
                return false;
            }
        };
        self.port = available;

        let access_url = if self.url.is_empty() {
            format!("http://localhost:{}", self.port)
        } else {
            self.url.clone()
        };

        output::info("Web 服务已启动");
        output::info(&format!("请访问: {access_url}"));
        output::info("等待用户完成决策...");

        self.serve().await
    }

    async fn serve(&self) -> bool {
        let state = Arc::new(Mutex::new(AppState {
            storage: DecideStorage::new(&self.root),
            web_dir: self.web_dir.clone(),
            should_close: false,
            project_root: self.root.clone(),
        }));

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([header::CONTENT_TYPE]);

        let app = Router::new()
            .route("/api/items", get(handle_get_items))
            .route("/api/submit", post(handle_submit))
            .fallback(handle_static_file)
            .with_state(state.clone())
            .layer(cors);

        let addr = format!("{}:{}", self.bind, self.port);
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("绑定端口失败: {e}");
                return false;
            }
        };

        let timeout = self.timeout;
        let state_clone = state.clone();

        tokio::spawn(async move {
            let start = Instant::now();
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let s = state_clone.lock().await;
                if s.should_close {
                    break;
                }
                if timeout > 0 && start.elapsed().as_secs() >= timeout {
                    break;
                }
            }
            // Signal shutdown - the server will stop after this
            std::process::exit(0);
        });

        let _ = axum::serve(listener, app).await;
        true
    }
}
