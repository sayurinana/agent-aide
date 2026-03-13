# Change: 集成 PlantUML 自包含可执行程序管理

## Why

当前 aide 的 PlantUML 支持依赖外部 Java 环境和 jar 文件，用户需要自行安装 Java 和 PlantUML 才能使用流程图功能。这增加了环境配置的复杂度，降低了开箱即用体验。通过集成自包含的 PlantUML 可执行程序（内嵌 JRE），可以消除对外部 Java 环境的依赖，实现一键安装。

## What Changes

- 废弃 `plantuml.jar_path` 配置项，替换为基于自包含可执行程序的新配置方案
- 在 `[plantuml]` 配置节新增 `download_cache_path`、`clean_cache_after_install`、`install_path`、`download_url` 四个配置项
- 新增 PlantUML 工具管理模块，实现检测、下载、解压和验证功能
- 增强 `aide init --global` 命令，在全局初始化时检测 PlantUML 并提示自动安装
- 自定义 `aide -V` 版本输出，显示 PlantUML 的版本、路径和可用状态
- 更新流程图 hook 中的 PlantUML 命令解析逻辑，优先使用新的可执行程序路径
- **BREAKING**：移除 `plantuml.jar_path` 配置项，`schema_version` 升级至 2

## Impact

- Affected specs: `config`、`cli`、`flow`、新增 `plantuml-management`
- Affected code:
  - `src/core/config.rs` — DEFAULT_CONFIG、DEFAULT_CONFIG_MD、schema_version
  - `src/cli/init.rs` — handle_init_global() 增加 PlantUML 检测和安装
  - `src/main.rs` — 自定义版本输出逻辑
  - `src/flow/hooks.rs` — get_plantuml_command() 适配新路径
  - 新增 `src/core/plantuml.rs` — PlantUML 工具管理模块
  - `Cargo.toml` — 新增 reqwest、flate2、tar 依赖
  - `docs/` — 更新相关文档
