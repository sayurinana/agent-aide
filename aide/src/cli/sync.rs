use crate::core::config::{self, ConfigManager, DEFAULT_PLUGIN_REPO_URL};
use crate::core::git;
use crate::core::output;

/// 处理 aide sync 命令
pub fn handle_sync() -> bool {
    // 获取全局配置管理器
    let global_cfg = match ConfigManager::new_global() {
        Some(cfg) => cfg,
        None => {
            output::err("无法获取用户主目录，请确保 $HOME 环境变量已设置");
            return false;
        }
    };

    // 检测 Git 可用性
    if !git::is_git_available() {
        output::warn("Git 未安装，无法同步仓库");
        return false;
    }

    // 读取配置中的仓库地址
    let config = global_cfg.load_config();
    let repo_url = config::get_config_string(&config, "plugin.repo_url")
        .unwrap_or_else(|| DEFAULT_PLUGIN_REPO_URL.to_string());

    // 目标目录：~/.aide/agent-aide/
    let target_dir = global_cfg.aide_dir.join("agent-aide");

    match git::clone_or_update_repo(&repo_url, &target_dir) {
        Ok(()) => true, // 静默成功
        Err(e) => {
            output::err(&format!("仓库同步失败：{}", e));
            false
        }
    }
}
