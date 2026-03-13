use crate::core::config::{self, ConfigManager};
use crate::core::output;
use crate::core::plantuml;
use std::fs;

pub fn handle_init(global: bool) -> bool {
    if global {
        return handle_init_global();
    }

    // 默认分支：先确保全局配置 → 复制到项目 → 处理项目初始化
    let root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

    // 步骤 1：确保全局配置存在
    match ConfigManager::new_global() {
        Some(global_cfg) => {
            let _ = global_cfg.ensure_config();

            // 步骤 2：检查全局配置 schema 版本
            let global_config = global_cfg.load_config();
            let global_schema = config::walk_get(&global_config, "meta.schema_version")
                .and_then(|v| v.as_integer())
                .unwrap_or(0);
            if global_schema < config::CURRENT_SCHEMA_VERSION {
                output::warn(&format!(
                    "全局配置 schema 版本较低（v{}），建议执行 aide config update --global 升级",
                    global_schema
                ));
            }

            // 步骤 3：项目初始化
            let project_cfg = ConfigManager::new(&root);
            let _ = project_cfg.ensure_base_dirs();

            if !project_cfg.config_path.exists() {
                // 从全局配置复制到项目
                let _ = fs::copy(&global_cfg.config_path, &project_cfg.config_path);
                output::ok("已从全局配置复制到项目 .aide/config.toml");
            }

            if !project_cfg.config_md_path.exists() {
                project_cfg.generate_config_md();
                output::ok("已创建配置说明 .aide/config.md");
            }

            project_cfg.ensure_gitignore();
        }
        None => {
            // $HOME 不可用，回退到原有逻辑
            output::warn("无法获取用户主目录，跳过全局配置初始化");
            let cfg = ConfigManager::new(&root);
            let _ = cfg.ensure_config();
            cfg.ensure_gitignore();
        }
    }

    output::ok("初始化完成，.aide/ 与默认配置已准备就绪");
    true
}

fn handle_init_global() -> bool {
    let global_cfg = match ConfigManager::new_global() {
        Some(cfg) => cfg,
        None => {
            output::err("无法获取用户主目录，请确保 $HOME 环境变量已设置");
            return false;
        }
    };

    if global_cfg.config_path.exists() {
        output::info(&format!(
            "全局配置已存在：{}",
            global_cfg.config_path.display()
        ));
    } else {
        let _ = global_cfg.ensure_config();
    }

    // 检测 PlantUML 可用性
    let global_config = global_cfg.load_config();
    plantuml::ensure_plantuml(&global_config);

    output::ok("全局配置初始化完成");
    true
}
