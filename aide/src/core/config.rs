use std::fs;
use std::path::{Path, PathBuf};

use crate::core::output;

pub const CURRENT_AIDE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CURRENT_SCHEMA_VERSION: i64 = 2;

/// 获取全局配置目录路径 `$HOME/.aide`
/// 当 `$HOME` 环境变量不可用时返回 None
pub fn global_aide_dir() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(|home| PathBuf::from(home).join(".aide"))
}

pub const DEFAULT_CONFIG: &str = r#"[meta]
aide_version = "0.1.0"
schema_version = 2

[general]
gitignore_aide = false

[task]
source = "task-now.md"
spec = "task-spec.md"
plans_path = ".aide/task-plans/"

[docs]
path = ".aide/project-docs"

[flow]
phases = ["task-optimize", "flow-design", "impl", "verify", "docs", "confirm", "finish"]
diagram_path = ".aide/diagrams"

[plantuml]
download_cache_path = "download-buffer"
clean_cache_after_install = true
install_path = "utils"
download_url = "https://github.com/sayurinana/agent-aide/releases/download/resource-001/plantuml-1.2025.4-linux-x64.tar.gz"
font_name = "Arial"
dpi = 300
scale = 0.5

[decide]
port = 3721
bind = "127.0.0.1"
url = ""
timeout = 0
"#;

pub const DEFAULT_CONFIG_MD: &str = r#"# Aide 配置说明

本文档详细说明 `.aide/config.toml` 中的所有配置项。

## 配置操作

- **读取配置**：`aide config get <key>`（如 `aide config get flow.phases`）
- **设置配置**：`aide config set <key> <value>`（如 `aide config set task.source "my-task.md"`）
- **重置配置**：`aide config reset`（重置为默认值，自动备份）
- **更新配置**：`aide config update`（版本升级时更新配置）

支持点号分隔的嵌套键，如 `task.source`、`flow.phases`。

## [meta] - 元数据

配置文件的版本信息，用于版本管理和迁移。

- **aide_version**（字符串）：生成此配置的 aide 版本号
- **schema_version**（整数）：配置结构的 schema 版本

## [general] - 通用配置

控制 Aide 的全局行为。

- **gitignore_aide**（布尔值，默认 `false`）
  - `true`：自动添加 `.aide/` 到 `.gitignore`，不跟踪 aide 状态
  - `false`：不修改 `.gitignore`，允许 git 跟踪 `.aide` 目录（推荐，便于多设备同步）

## [task] - 任务文档配置

定义任务相关文档的路径。

- **source**（字符串，默认 `"task-now.md"`）：用户提供的原始任务描述文档
- **spec**（字符串，默认 `"task-spec.md"`）：aide 生成的可执行任务细则文档
- **plans_path**（字符串，默认 `".aide/task-plans/"`）：复杂任务计划文档目录

## [docs] - 项目文档配置

- **path**（字符串，默认 `".aide/project-docs"`）：项目文档目录路径

## [flow] - 流程追踪配置

控制任务流程追踪行为。

- **phases**（数组，默认 `["task-optimize", "flow-design", "impl", "verify", "docs", "confirm", "finish"]`）
  - 任务流程的环节名称列表（有序）
  - 可自定义环节名称和顺序
- **diagram_path**（字符串，默认 `".aide/diagrams"`）：流程图输出目录

## [plantuml] - PlantUML 配置

PlantUML 图表生成及工具管理相关配置。路径配置均为相对于 `~/.aide/` 全局配置目录的相对路径。

- **download_cache_path**（字符串，默认 `"download-buffer"`）：下载缓存目录
  - 相对于 `~/.aide/`，即默认路径为 `~/.aide/download-buffer/`
- **clean_cache_after_install**（布尔值，默认 `true`）：安装完成后是否删除下载的压缩包
- **install_path**（字符串，默认 `"utils"`）：工具程序安装目录
  - 相对于 `~/.aide/`，即默认路径为 `~/.aide/utils/`
  - PlantUML 可执行文件路径为 `~/.aide/{install_path}/plantuml/bin/plantuml`
- **download_url**（字符串）：PlantUML 程序包下载链接
  - 默认指向 GitHub Releases 上的 Linux x64 自包含程序包
- **font_name**（字符串，默认 `"Arial"`）：图表默认字体
- **dpi**（整数，默认 `300`）：图表 DPI 值
- **scale**（浮点数，默认 `0.5`）：图表缩放系数

## [decide] - 待定项确认配置

待定项确认 Web 服务配置。

- **port**（整数，默认 `3721`）：HTTP 服务起始端口
- **bind**（字符串，默认 `"127.0.0.1"`）：监听地址
- **url**（字符串，默认 `""`）：自定义访问地址（可选）
- **timeout**（整数，默认 `0`）：超时时间（秒），0 表示不超时
"#;

pub struct ConfigManager {
    pub root: PathBuf,
    pub aide_dir: PathBuf,
    pub config_path: PathBuf,
    pub config_md_path: PathBuf,
    pub decisions_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub backups_dir: PathBuf,
}

impl ConfigManager {
    pub fn new(root: &Path) -> Self {
        let aide_dir = root.join(".aide");
        Self {
            root: root.to_path_buf(),
            config_path: aide_dir.join("config.toml"),
            config_md_path: aide_dir.join("config.md"),
            decisions_dir: aide_dir.join("decisions"),
            logs_dir: aide_dir.join("logs"),
            backups_dir: aide_dir.join("backups"),
            aide_dir,
        }
    }

    /// 创建以 `$HOME` 为根目录的全局配置管理器
    /// 全局配置目录为 `$HOME/.aide/`
    pub fn new_global() -> Option<Self> {
        global_aide_dir().map(|aide_dir| {
            let root = aide_dir.parent().unwrap_or(Path::new("/")).to_path_buf();
            Self {
                root: root.clone(),
                config_path: aide_dir.join("config.toml"),
                config_md_path: aide_dir.join("config.md"),
                decisions_dir: aide_dir.join("decisions"),
                logs_dir: aide_dir.join("logs"),
                backups_dir: aide_dir.join("backups"),
                aide_dir,
            }
        })
    }

    pub fn ensure_base_dirs(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.aide_dir)?;
        fs::create_dir_all(&self.decisions_dir)?;
        fs::create_dir_all(&self.logs_dir)?;
        fs::create_dir_all(&self.backups_dir)?;
        Ok(())
    }

    pub fn ensure_gitignore(&self) {
        let config = self.load_config();
        let gitignore_aide = walk_get(&config, "general.gitignore_aide")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if !gitignore_aide {
            return;
        }

        let gitignore_path = self.root.join(".gitignore");
        let marker = ".aide/";

        if gitignore_path.exists() {
            let content = fs::read_to_string(&gitignore_path).unwrap_or_default();
            if content.lines().any(|line| line.trim() == marker) {
                return;
            }
            let mut new_content = content;
            if !new_content.ends_with('\n') {
                new_content.push('\n');
            }
            new_content.push_str(marker);
            new_content.push('\n');
            let _ = fs::write(&gitignore_path, new_content);
        } else {
            let _ = fs::write(&gitignore_path, format!("{marker}\n"));
        }
    }

    pub fn ensure_config(&self) -> toml::Value {
        let _ = self.ensure_base_dirs();
        let mut created_config = false;
        let mut created_md = false;

        if !self.config_path.exists() {
            let _ = fs::write(&self.config_path, DEFAULT_CONFIG);
            output::ok("已创建默认配置 .aide/config.toml");
            created_config = true;
        }

        if !self.config_md_path.exists() {
            let _ = fs::write(&self.config_md_path, DEFAULT_CONFIG_MD);
            output::ok("已创建配置说明 .aide/config.md");
            created_md = true;
        }

        if !created_config && !created_md {
            // 仅在两者都已存在时不输出
        }

        self.load_config()
    }

    pub fn generate_config_md(&self) {
        let _ = fs::write(&self.config_md_path, DEFAULT_CONFIG_MD);
    }

    pub fn load_config(&self) -> toml::Value {
        if !self.config_path.exists() {
            return toml::Value::Table(toml::map::Map::new());
        }
        match fs::read_to_string(&self.config_path) {
            Ok(content) => match content.parse::<toml::Value>() {
                Ok(val) => val,
                Err(e) => {
                    output::err(&format!("读取配置失败: {e}"));
                    toml::Value::Table(toml::map::Map::new())
                }
            },
            Err(e) => {
                output::err(&format!("读取配置失败: {e}"));
                toml::Value::Table(toml::map::Map::new())
            }
        }
    }

    pub fn get_value(&self, key: &str) -> Option<toml::Value> {
        let data = self.load_config();
        walk_get(&data, key).cloned()
    }

    pub fn set_value(&self, key: &str, value: &str) {
        let _ = self.ensure_config();
        let parsed = parse_value(value);
        self.update_config_value(key, &parsed);
        output::ok(&format!("已更新 {key} = {}", format_toml_value(&parsed)));
    }

    fn update_config_value(&self, key: &str, value: &toml_edit::Value) {
        let content = fs::read_to_string(&self.config_path).unwrap_or_default();
        let mut doc = match content.parse::<toml_edit::DocumentMut>() {
            Ok(d) => d,
            Err(_) => {
                output::warn("配置文件解析失败，将重写");
                return;
            }
        };

        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() == 1 {
            doc[parts[0]] = toml_edit::Item::Value(value.clone());
        } else {
            // Navigate to the parent table, creating sections as needed
            let mut current = doc.as_table_mut() as &mut dyn toml_edit::TableLike;
            for &section in &parts[..parts.len() - 1] {
                if !current.contains_key(section) {
                    current.insert(section, toml_edit::Item::Table(toml_edit::Table::new()));
                }
                current = match current.get_mut(section) {
                    Some(toml_edit::Item::Table(t)) => t as &mut dyn toml_edit::TableLike,
                    _ => return,
                };
            }
            let last_key = parts[parts.len() - 1];
            current.insert(last_key, toml_edit::Item::Value(value.clone()));
        }

        let _ = fs::write(&self.config_path, doc.to_string());
    }
}

fn parse_value(raw: &str) -> toml_edit::Value {
    let lowered = raw.to_lowercase();
    if lowered == "true" {
        return toml_edit::Value::from(true);
    }
    if lowered == "false" {
        return toml_edit::Value::from(false);
    }
    if let Ok(i) = raw.parse::<i64>() {
        if !raw.contains('.') {
            return toml_edit::Value::from(i);
        }
    }
    if let Ok(f) = raw.parse::<f64>() {
        if raw.contains('.') {
            return toml_edit::Value::from(f);
        }
    }
    toml_edit::Value::from(raw)
}

fn format_toml_value(value: &toml_edit::Value) -> String {
    match value {
        toml_edit::Value::String(s) => format!("\"{}\"", s.value()),
        toml_edit::Value::Integer(i) => i.value().to_string(),
        toml_edit::Value::Float(f) => f.value().to_string(),
        toml_edit::Value::Boolean(b) => b.value().to_string(),
        other => other.to_string(),
    }
}

pub fn walk_get<'a>(data: &'a toml::Value, dotted_key: &str) -> Option<&'a toml::Value> {
    let mut current = data;
    for part in dotted_key.split('.') {
        current = current.as_table()?.get(part)?;
    }
    Some(current)
}

pub fn get_config_string(config: &toml::Value, key: &str) -> Option<String> {
    walk_get(config, key).and_then(|v| v.as_str()).map(|s| s.to_string())
}

pub fn get_config_int(config: &toml::Value, key: &str) -> Option<i64> {
    walk_get(config, key).and_then(|v| v.as_integer())
}

pub fn get_config_string_or(config: &toml::Value, key: &str, default: &str) -> String {
    get_config_string(config, key).unwrap_or_else(|| default.to_string())
}

pub fn get_config_int_or(config: &toml::Value, key: &str, default: i64) -> i64 {
    get_config_int(config, key).unwrap_or(default)
}

pub fn get_phases(config: &toml::Value) -> Vec<String> {
    let default = vec![
        "task-optimize".into(),
        "flow-design".into(),
        "impl".into(),
        "verify".into(),
        "docs".into(),
        "confirm".into(),
        "finish".into(),
    ];

    walk_get(config, "flow.phases")
        .and_then(|v| v.as_array())
        .map(|arr| {
            let phases: Vec<String> = arr
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
            if phases.is_empty() {
                default.clone()
            } else {
                phases
            }
        })
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // === walk_get 测试 ===

    #[test]
    fn test_walk_get_top_level_key() {
        let config: toml::Value = toml::from_str(r#"name = "aide""#).unwrap();
        let val = walk_get(&config, "name");
        assert_eq!(val.unwrap().as_str().unwrap(), "aide");
    }

    #[test]
    fn test_walk_get_nested_key() {
        let config: toml::Value = toml::from_str(
            r#"
            [task]
            source = "task-now.md"
            "#,
        )
        .unwrap();
        let val = walk_get(&config, "task.source");
        assert_eq!(val.unwrap().as_str().unwrap(), "task-now.md");
    }

    #[test]
    fn test_walk_get_deeply_nested() {
        let config: toml::Value = toml::from_str(
            r#"
            [a.b]
            c = 42
            "#,
        )
        .unwrap();
        let val = walk_get(&config, "a.b.c");
        assert_eq!(val.unwrap().as_integer().unwrap(), 42);
    }

    #[test]
    fn test_walk_get_missing_key_returns_none() {
        let config: toml::Value = toml::from_str(r#"name = "aide""#).unwrap();
        assert!(walk_get(&config, "nonexistent").is_none());
        assert!(walk_get(&config, "a.b.c").is_none());
    }

    // === parse_value 测试 ===

    #[test]
    fn test_parse_value_bool() {
        assert_eq!(parse_value("true").as_bool().unwrap(), true);
        assert_eq!(parse_value("True").as_bool().unwrap(), true);
        assert_eq!(parse_value("TRUE").as_bool().unwrap(), true);
        assert_eq!(parse_value("false").as_bool().unwrap(), false);
    }

    #[test]
    fn test_parse_value_integer() {
        assert_eq!(parse_value("42").as_integer().unwrap(), 42);
        assert_eq!(parse_value("0").as_integer().unwrap(), 0);
        assert_eq!(parse_value("-5").as_integer().unwrap(), -5);
    }

    #[test]
    fn test_parse_value_float() {
        let val = parse_value("3.14");
        assert!((val.as_float().unwrap() - 3.14).abs() < 0.001);
    }

    #[test]
    fn test_parse_value_string() {
        let val = parse_value("hello world");
        assert_eq!(val.as_str().unwrap(), "hello world");
    }

    // === format_toml_value 测试 ===

    #[test]
    fn test_format_toml_value_string() {
        let val = toml_edit::Value::from("hello");
        assert_eq!(format_toml_value(&val), "\"hello\"");
    }

    #[test]
    fn test_format_toml_value_integer() {
        let val = toml_edit::Value::from(42);
        assert_eq!(format_toml_value(&val), "42");
    }

    #[test]
    fn test_format_toml_value_bool() {
        let val = toml_edit::Value::from(true);
        assert_eq!(format_toml_value(&val), "true");
    }

    // === ConfigManager 测试 ===

    #[test]
    fn test_config_manager_new() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        assert_eq!(cm.root, tmp.path());
        assert_eq!(cm.aide_dir, tmp.path().join(".aide"));
        assert_eq!(cm.config_path, tmp.path().join(".aide").join("config.toml"));
    }

    #[test]
    fn test_ensure_base_dirs() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        cm.ensure_base_dirs().unwrap();
        assert!(cm.aide_dir.exists());
        assert!(cm.decisions_dir.exists());
        assert!(cm.logs_dir.exists());
    }

    #[test]
    fn test_ensure_config_creates_default() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        let config = cm.ensure_config();
        assert!(cm.config_path.exists());
        // 验证默认配置包含预期字段
        assert!(walk_get(&config, "general.gitignore_aide").is_some());
        assert!(walk_get(&config, "task.source").is_some());
        assert!(walk_get(&config, "flow.phases").is_some());
    }

    #[test]
    fn test_load_config_empty_when_no_file() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        let config = cm.load_config();
        assert!(config.as_table().unwrap().is_empty());
    }

    #[test]
    fn test_get_value() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        cm.ensure_config();
        let val = cm.get_value("task.source");
        assert_eq!(val.unwrap().as_str().unwrap(), "task-now.md");
    }

    #[test]
    fn test_set_value_and_get() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        cm.ensure_config();
        cm.set_value("task.source", "new-task.md");
        let val = cm.get_value("task.source");
        assert_eq!(val.unwrap().as_str().unwrap(), "new-task.md");
    }

    #[test]
    fn test_set_value_creates_nested_key() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        cm.ensure_config();
        cm.set_value("custom.key", "value123");
        let val = cm.get_value("custom.key");
        assert_eq!(val.unwrap().as_str().unwrap(), "value123");
    }

    #[test]
    fn test_set_value_preserves_structure() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        cm.ensure_config();
        let original = std::fs::read_to_string(&cm.config_path).unwrap();
        assert!(original.contains("[flow]"));
        assert!(original.contains("[meta]"));
        // 修改 task.source 不应影响其他配置节
        cm.set_value("task.source", "changed.md");
        let updated = std::fs::read_to_string(&cm.config_path).unwrap();
        assert!(updated.contains("[flow]"));
        assert!(updated.contains("[meta]"));
    }

    // === ensure_gitignore 测试 ===

    #[test]
    fn test_ensure_gitignore_when_disabled() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        cm.ensure_config();
        // 默认 gitignore_aide = false，不应创建 .gitignore
        cm.ensure_gitignore();
        assert!(!tmp.path().join(".gitignore").exists());
    }

    #[test]
    fn test_ensure_gitignore_when_enabled() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        cm.ensure_config();
        cm.set_value("general.gitignore_aide", "true");
        cm.ensure_gitignore();
        let content = std::fs::read_to_string(tmp.path().join(".gitignore")).unwrap();
        assert!(content.contains(".aide/"));
    }

    #[test]
    fn test_ensure_gitignore_no_duplicate() {
        let tmp = TempDir::new().unwrap();
        let cm = ConfigManager::new(tmp.path());
        cm.ensure_config();
        cm.set_value("general.gitignore_aide", "true");
        std::fs::write(tmp.path().join(".gitignore"), ".aide/\n").unwrap();
        cm.ensure_gitignore();
        let content = std::fs::read_to_string(tmp.path().join(".gitignore")).unwrap();
        assert_eq!(content.matches(".aide/").count(), 1);
    }

    // === get_config_* 辅助函数测试 ===

    #[test]
    fn test_get_config_string() {
        let config: toml::Value =
            toml::from_str(r#"[task]\nsource = "foo.md""#.replace("\\n", "\n").as_str()).unwrap();
        assert_eq!(get_config_string(&config, "task.source").unwrap(), "foo.md");
    }

    #[test]
    fn test_get_config_int() {
        let config: toml::Value =
            toml::from_str(r#"[decide]\nport = 3721"#.replace("\\n", "\n").as_str()).unwrap();
        assert_eq!(get_config_int(&config, "decide.port").unwrap(), 3721);
    }

    #[test]
    fn test_get_config_string_or_default() {
        let config = toml::Value::Table(toml::map::Map::new());
        assert_eq!(get_config_string_or(&config, "x.y", "default"), "default");
    }

    #[test]
    fn test_get_config_int_or_default() {
        let config = toml::Value::Table(toml::map::Map::new());
        assert_eq!(get_config_int_or(&config, "x.y", 99), 99);
    }

    // === get_phases 测试 ===

    #[test]
    fn test_get_phases_default() {
        let config = toml::Value::Table(toml::map::Map::new());
        let phases = get_phases(&config);
        assert_eq!(phases.len(), 7);
        assert_eq!(phases[0], "task-optimize");
        assert_eq!(phases[6], "finish");
    }

    #[test]
    fn test_get_phases_custom() {
        let config: toml::Value = toml::from_str(
            r#"
            [flow]
            phases = ["a", "b", "c"]
            "#,
        )
        .unwrap();
        let phases = get_phases(&config);
        assert_eq!(phases, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_get_phases_empty_returns_default() {
        let config: toml::Value = toml::from_str(
            r#"
            [flow]
            phases = []
            "#,
        )
        .unwrap();
        let phases = get_phases(&config);
        assert_eq!(phases.len(), 7);
    }

    // === global_aide_dir / new_global 测试 ===

    #[test]
    fn test_global_aide_dir_returns_path() {
        // 测试环境中 $HOME 通常已设置
        if std::env::var("HOME").is_ok() {
            let dir = global_aide_dir();
            assert!(dir.is_some());
            let dir = dir.unwrap();
            assert!(dir.ends_with(".aide"));
        }
    }

    #[test]
    fn test_new_global_creates_correct_paths() {
        if std::env::var("HOME").is_ok() {
            let home = PathBuf::from(std::env::var("HOME").unwrap());
            let cm = ConfigManager::new_global();
            assert!(cm.is_some());
            let cm = cm.unwrap();
            assert_eq!(cm.aide_dir, home.join(".aide"));
            assert_eq!(cm.config_path, home.join(".aide").join("config.toml"));
            assert_eq!(cm.backups_dir, home.join(".aide").join("backups"));
        }
    }
}
