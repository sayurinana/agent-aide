# Aide 配置说明

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
