use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::core::output;
use crate::core::project::find_project_root;
use crate::decide::storage::DecideStorage;
use crate::decide::types::*;

pub fn handle_decide_submit(file_path: &str, web_dir: Option<&str>) -> bool {
    let root = find_project_root(None);
    let storage = DecideStorage::new(&root);

    // 1. 读取 JSON 文件
    let json_file = PathBuf::from(file_path);
    let json_file = if json_file.is_absolute() {
        json_file
    } else {
        root.join(&json_file)
    };

    if !json_file.exists() {
        print_error(&format!("文件不存在: {file_path}"), None);
        return false;
    }

    let raw = match std::fs::read_to_string(&json_file) {
        Ok(r) => r,
        Err(e) => {
            print_error(&format!("读取文件失败: {e}"), None);
            return false;
        }
    };

    // 2. 验证数据格式
    let decide_input: DecideInput = match serde_json::from_str(&raw) {
        Ok(d) => d,
        Err(e) => {
            print_error(
                &format!("JSON 解析失败: {e}"),
                Some("检查 JSON 格式是否正确"),
            );
            return false;
        }
    };

    if let Err(e) = validate_input(&decide_input) {
        print_error(&format!("数据验证失败: {e}"), Some("检查必填字段是否完整"));
        return false;
    }

    // 3. 保存到 pending.json
    match storage.save_pending(&decide_input) {
        Ok(_) => {}
        Err(e) => {
            print_error(&e, None);
            return false;
        }
    }

    // 4. 启动后台服务
    start_daemon(&root, &storage, web_dir)
}

fn start_daemon(root: &Path, storage: &DecideStorage, web_dir: Option<&str>) -> bool {
    // 启动自身作为后台服务进程
    let exe = match std::env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            print_error(&format!("获取可执行文件路径失败: {e}"), None);
            return false;
        }
    };

    let mut cmd = Command::new(&exe);
    cmd.arg("decide")
        .arg("serve")
        .arg("--root")
        .arg(root)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());

    if let Some(wd) = web_dir {
        cmd.arg("--web-dir").arg(wd);
    }

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }

    match cmd.spawn() {
        Ok(_) => {}
        Err(e) => {
            print_error(&format!("启动后台服务失败: {e}"), None);
            return false;
        }
    }

    // 等待服务启动
    for _ in 0..50 {
        thread::sleep(Duration::from_millis(100));
        if let Some(info) = storage.load_server_info() {
            output::info("Web 服务已启动");
            output::info(&format!("请访问: {}", info.url));
            output::info("用户完成决策后执行 aide decide result 获取结果");
            return true;
        }
    }

    print_error("服务启动超时", Some("请检查端口是否被占用"));
    false
}

pub fn handle_decide_result() -> bool {
    let root = find_project_root(None);
    let storage = DecideStorage::new(&root);

    // 检查 pending
    let pending = match storage.load_pending() {
        Ok(Some(p)) => p,
        Ok(None) => {
            print_error("未找到待定项数据", Some("请先执行 aide decide submit <file>"));
            return false;
        }
        Err(e) => {
            print_error(&e, None);
            return false;
        }
    };

    match &pending.meta {
        Some(_) => {}
        None => {
            print_error(
                "数据异常",
                Some("pending.json 缺少 session_id，请重新执行 aide decide submit"),
            );
            return false;
        }
    };

    // 检查结果
    match storage.load_result() {
        Ok(Some(result)) => {
            let payload = serde_json::to_string(&result).unwrap_or_default();
            println!("{payload}");
            storage.clear_server_info();
            true
        }
        Ok(None) => {
            if storage.is_server_running() {
                print_error("尚无决策结果", Some("请等待用户在 Web 界面完成操作"));
            } else {
                print_error(
                    "尚无决策结果",
                    Some("服务可能已超时关闭，请重新执行 aide decide submit"),
                );
            }
            false
        }
        Err(e) => {
            print_error(&e, None);
            false
        }
    }
}

pub async fn handle_decide_serve(root: &str, web_dir: Option<&str>) -> bool {
    let root = PathBuf::from(root);
    if !root.exists() {
        eprintln!("目录不存在: {}", root.display());
        return false;
    }

    let web_dir_path = web_dir.map(PathBuf::from);
    let mut server = crate::decide::server::DecideServer::new(
        &root,
        web_dir_path.as_deref(),
    );
    let pid = std::process::id();
    server.start_daemon(pid).await
}

fn print_error(message: &str, suggestion: Option<&str>) {
    let _ = writeln!(io::stderr(), "\u{2717} {message}");
    if let Some(s) = suggestion {
        let _ = writeln!(io::stderr(), "  建议: {s}");
    }
}
