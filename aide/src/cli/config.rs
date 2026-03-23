use crate::core::config::ConfigManager;
use crate::core::output;
use crate::core::project::find_project_root;
use std::fs;
use std::io::{self, Write};

/// 根据 global 标志获取对应的 ConfigManager
/// 当 global=true 但 $HOME 不可用时返回 None 并输出错误
fn get_config_manager(global: bool) -> Option<ConfigManager> {
    if global {
        match ConfigManager::new_global() {
            Some(cfg) => Some(cfg),
            None => {
                output::err("无法获取用户主目录，请确保 $HOME 环境变量已设置");
                None
            }
        }
    } else {
        let root = find_project_root(None);
        Some(ConfigManager::new(&root))
    }
}

/// 检查全局配置文件是否存在，不存在时输出错误
fn ensure_global_config_exists(cfg: &ConfigManager) -> bool {
    if !cfg.config_path.exists() {
        output::err(&format!(
            "全局配置文件不存在：{}，请先执行 aide init --global",
            cfg.config_path.display()
        ));
        return false;
    }
    true
}

pub fn handle_config_get(key: &str, global: bool) -> bool {
    let cfg = match get_config_manager(global) {
        Some(cfg) => cfg,
        None => return false,
    };

    if global && !ensure_global_config_exists(&cfg) {
        return false;
    }

    match cfg.get_value(key) {
        Some(value) => {
            output::info(&format!("{key} = {value}"));
            true
        }
        None => {
            output::warn(&format!("未找到配置项 {key}"));
            false
        }
    }
}

pub fn handle_config_set(key: &str, value: &str, global: bool) -> bool {
    let cfg = match get_config_manager(global) {
        Some(cfg) => cfg,
        None => return false,
    };

    if global && !ensure_global_config_exists(&cfg) {
        return false;
    }

    cfg.set_value(key, value);
    true
}

pub fn handle_config_reset(force: bool, global: bool) -> bool {
    let cfg = match get_config_manager(global) {
        Some(cfg) => cfg,
        None => return false,
    };

    if cfg.config_path.exists() && !force {
        output::warn("此操作将重置配置到默认值，现有配置将备份。是否继续？[y/N]");
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return false;
        }

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            output::info("已取消");
            return false;
        }
    }

    if cfg.config_path.exists() {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_path = cfg.backups_dir.join(format!("config.toml.{}", timestamp));
        let _ = cfg.ensure_base_dirs();

        if let Err(e) = fs::copy(&cfg.config_path, &backup_path) {
            output::err(&format!("备份失败: {}", e));
            return false;
        }

        let backup_display = if global {
            format!("~/.aide/backups/config.toml.{}", timestamp)
        } else {
            format!("aide-memory/backups/config.toml.{}", timestamp)
        };
        output::ok(&format!("已备份配置到 {}", backup_display));
    }

    let _ = cfg.ensure_base_dirs();

    if global {
        // --global：从程序默认配置重新生成
        let _ = fs::write(&cfg.config_path, crate::core::config::DEFAULT_CONFIG);
        cfg.generate_config_md();
        output::ok("全局配置已重置为默认值");
    } else {
        // 项目配置：从全局配置复制
        match ConfigManager::new_global() {
            Some(global_cfg) if global_cfg.config_path.exists() => {
                let _ = fs::copy(&global_cfg.config_path, &cfg.config_path);
                cfg.generate_config_md();
                output::ok("项目配置已从全局配置重置");
            }
            _ => {
                // 全局配置不可用，回退到程序默认配置
                let _ = fs::write(&cfg.config_path, crate::core::config::DEFAULT_CONFIG);
                cfg.generate_config_md();
                output::warn("全局配置不可用，已使用程序默认配置重置");
            }
        }
    }

    true
}

pub fn handle_config_update(global: bool) -> bool {
    let cfg = match get_config_manager(global) {
        Some(cfg) => cfg,
        None => return false,
    };

    if !cfg.config_path.exists() {
        if global {
            output::err(&format!(
                "全局配置文件不存在：{}，请先执行 aide init --global",
                cfg.config_path.display()
            ));
        } else {
            output::err("项目配置文件不存在，请先执行 aide init");
        }
        return false;
    }

    let config = cfg.load_config();
    let current_schema = crate::core::config::walk_get(&config, "meta.schema_version")
        .and_then(|v| v.as_integer())
        .unwrap_or(0);

    if current_schema >= crate::core::config::CURRENT_SCHEMA_VERSION {
        output::ok(&format!("配置已是最新版本（schema v{}）", current_schema));
        return true;
    }

    output::info(&format!(
        "检测到配置版本差异：当前 schema v{}，最新 schema v{}",
        current_schema,
        crate::core::config::CURRENT_SCHEMA_VERSION
    ));

    // 使用 toml_edit 保留用户自定义值
    let content = fs::read_to_string(&cfg.config_path).unwrap_or_default();
    let mut doc = match content.parse::<toml_edit::DocumentMut>() {
        Ok(d) => d,
        Err(_) => {
            output::warn("配置文件解析失败，将使用默认配置覆盖");
            let _ = fs::write(&cfg.config_path, crate::core::config::DEFAULT_CONFIG);
            cfg.generate_config_md();
            output::ok(&format!(
                "配置已更新到 schema v{}",
                crate::core::config::CURRENT_SCHEMA_VERSION
            ));
            return true;
        }
    };

    // 执行逐版本迁移
    let mut schema = current_schema;
    while schema < crate::core::config::CURRENT_SCHEMA_VERSION {
        match schema {
            0 | 1 => migrate_v1_to_v2(&mut doc),
            2 => migrate_v2_to_v3(&mut doc),
            _ => {}
        }
        schema += 1;
    }

    // 更新 meta 版本信息
    if let Some(meta) = doc.get_mut("meta").and_then(|v| v.as_table_mut()) {
        meta.insert(
            "schema_version",
            toml_edit::Item::Value(toml_edit::Value::from(
                crate::core::config::CURRENT_SCHEMA_VERSION,
            )),
        );
        meta.insert(
            "aide_version",
            toml_edit::Item::Value(toml_edit::Value::from(
                crate::core::config::CURRENT_AIDE_VERSION,
            )),
        );
    } else {
        // meta 节不存在，创建
        let mut meta = toml_edit::Table::new();
        meta.insert(
            "aide_version",
            toml_edit::Item::Value(toml_edit::Value::from(
                crate::core::config::CURRENT_AIDE_VERSION,
            )),
        );
        meta.insert(
            "schema_version",
            toml_edit::Item::Value(toml_edit::Value::from(
                crate::core::config::CURRENT_SCHEMA_VERSION,
            )),
        );
        doc.insert("meta", toml_edit::Item::Table(meta));
    }

    let _ = fs::write(&cfg.config_path, doc.to_string());
    cfg.generate_config_md();

    output::ok(&format!(
        "配置已更新到 schema v{}",
        crate::core::config::CURRENT_SCHEMA_VERSION
    ));
    let md_display = if global {
        "~/.aide/config.md"
    } else {
        "aide-memory/config.md"
    };
    output::ok(&format!("已更新配置说明 {}", md_display));
    true
}

/// Schema v1 → v2 迁移：移除 jar_path，添加 PlantUML 工具管理配置
fn migrate_v1_to_v2(doc: &mut toml_edit::DocumentMut) {
    // 确保 [plantuml] 节存在
    if doc.get("plantuml").is_none() {
        doc.insert("plantuml", toml_edit::Item::Table(toml_edit::Table::new()));
    }

    if let Some(plantuml) = doc.get_mut("plantuml").and_then(|v| v.as_table_mut()) {
        // 移除 jar_path
        plantuml.remove("jar_path");

        // 添加新配置项（仅在不存在时添加）
        if plantuml.get("download_cache_path").is_none() {
            plantuml.insert(
                "download_cache_path",
                toml_edit::Item::Value(toml_edit::Value::from("download-buffer")),
            );
        }
        if plantuml.get("clean_cache_after_install").is_none() {
            plantuml.insert(
                "clean_cache_after_install",
                toml_edit::Item::Value(toml_edit::Value::from(true)),
            );
        }
        if plantuml.get("install_path").is_none() {
            plantuml.insert(
                "install_path",
                toml_edit::Item::Value(toml_edit::Value::from("utils")),
            );
        }
        if plantuml.get("download_url").is_none() {
            plantuml.insert(
                "download_url",
                toml_edit::Item::Value(toml_edit::Value::from(
                    "https://github.com/sayurinana/agent-aide/releases/download/resource-001/plantuml-1.2025.4-linux-x64.tar.gz",
                )),
            );
        }
    }
}

/// Schema v2 → v3 迁移：重构配置结构以适应 aide-memory
fn migrate_v2_to_v3(doc: &mut toml_edit::DocumentMut) {
    // 移除 [general] 节
    doc.remove("general");

    // 移除 [docs] 节
    doc.remove("docs");

    // 重构 [task] 节
    if doc.get("task").is_none() {
        doc.insert("task", toml_edit::Item::Table(toml_edit::Table::new()));
    }

    if let Some(task) = doc.get_mut("task").and_then(|v| v.as_table_mut()) {
        // 移除旧字段
        task.remove("spec");
        task.remove("plans_path");

        // 重命名 source → description_file
        if let Some(source) = task.remove("source") {
            task.insert("description_file", source);
        } else if task.get("description_file").is_none() {
            task.insert(
                "description_file",
                toml_edit::Item::Value(toml_edit::Value::from("task-now.md")),
            );
        }

        // 添加新字段
        if task.get("template").is_none() {
            task.insert(
                "template",
                toml_edit::Item::Value(toml_edit::Value::from("任务口述模板.md")),
            );
        }
        if task.get("parse_guide").is_none() {
            task.insert(
                "parse_guide",
                toml_edit::Item::Value(toml_edit::Value::from("任务解析指导.md")),
            );
        }
    }

    // 添加 [branch] 节
    if doc.get("branch").is_none() {
        let mut branch = toml_edit::Table::new();
        branch.insert("prefix", toml_edit::Item::Value(toml_edit::Value::from("")));
        branch.insert("format", toml_edit::Item::Value(toml_edit::Value::from("task-{n}")));
        branch.insert("resident", toml_edit::Item::Value(toml_edit::Value::from("dev")));
        doc.insert("branch", toml_edit::Item::Table(branch));
    }

    // 添加 [git] 节
    if doc.get("git").is_none() {
        let mut git = toml_edit::Table::new();
        git.insert("auto_commit_on_switch", toml_edit::Item::Value(toml_edit::Value::from(true)));
        git.insert("auto_commit_message", toml_edit::Item::Value(toml_edit::Value::from("暂存：清理仓库状态以切换分支")));
        git.insert("bye_commit_message", toml_edit::Item::Value(toml_edit::Value::from("暂存：清理仓库状态")));
        doc.insert("git", toml_edit::Item::Table(git));
    }

    // 更新 [flow] 节的 diagram_path
    if let Some(flow) = doc.get_mut("flow").and_then(|v| v.as_table_mut()) {
        flow.insert(
            "diagram_path",
            toml_edit::Item::Value(toml_edit::Value::from("aide-memory/memory/diagram")),
        );
    }
}
