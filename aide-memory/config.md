# Aide 配置说明

本文档详细说明 `aide-memory/config.toml` 中的所有配置项。

## 配置操作

- **读取配置**：`aide config get <key>`（如 `aide config get flow.phases`）
- **设置配置**：`aide config set <key> <value>`（如 `aide config set task.description_file "my-task.md"`）
- **重置配置**：`aide config reset`（重置为默认值，自动备份）
- **更新配置**：`aide config update`（版本升级时更新配置）

支持点号分隔的嵌套键，如 `task.description_file`、`flow.phases`。

## [meta] - 元数据

配置文件的版本信息，用于版本管理和迁移。

- **aide_version**（字符串）：生成此配置的 aide 版本号
- **schema_version**（整数）：配置结构的 schema 版本

## [task] - 任务配置

### description_file
- 类型：String
- 默认值：task-now.md
- 说明：任务描述文档路径（相对于项目根目录）

### template
- 类型：String
- 默认值：任务口述模板.md
- 说明：任务模板路径（相对于 templates/）

### parse_guide
- 类型：String
- 默认值：任务解析指导.md
- 说明：任务解析指导文档路径（相对于 templates/）

## [branch] - 分支配置

### prefix
- 类型：String
- 默认值：空
- 说明：任务分支名前缀

### format
- 类型：String
- 默认值：task-{n}
- 说明：分支名格式，{n} 为任务编号

### resident
- 类型：String
- 默认值：dev
- 说明：常驻工作分支名（不建议使用 master/main）

## [git] - Git 配置

### auto_commit_on_switch
- 类型：Boolean
- 默认值：true
- 说明：切换分支时自动暂存并提交

### auto_commit_message
- 类型：String
- 默认值：暂存：清理仓库状态以切换分支
- 说明：自动提交的默认消息

### bye_commit_message
- 类型：String
- 默认值：暂存：清理仓库状态
- 说明：bye 操作的默认提交消息

## [flow] - 流程追踪配置

控制任务流程追踪行为。

- **phases**（数组，默认 `["task-optimize", "flow-design", "impl", "verify", "docs", "confirm", "finish"]`）
  - 任务流程的环节名称列表（有序）
  - 可自定义环节名称和顺序
- **diagram_path**（字符串，默认 `"aide-memory/memory/diagram"`）：流程图输出目录

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

## [plugin] - 插件配置

插件仓库同步相关配置，用于自动同步 agent-aide 的 commands 和 skills。

- **repo_url**（字符串，默认 `https://github.com/sayurinana/agent-aide.git`）：agent-aide 仓库 Git 地址
  - 支持 SSH 和 HTTPS 格式
  - 可通过 `aide config set plugin.repo_url <url>` 修改
- **sync_on_init**（布尔值，默认 `true`）：项目初始化时是否同步插件
  - 设为 `false` 可禁用自动同步

## [template] - 模板配置

模板文件同步相关配置，控制 aide init 时模板文件的处理方式。

- **sync_strategy**（字符串，默认 `backup`）：模板同步策略
  - `backup`（默认）：下载新模板为 `.bak` 文件，保留原文件不变
  - `skip`：跳过已存在的文件，仅复制新文件
  - `overwrite`：直接覆盖已存在的文件
  - `backup-and-replace`：将原文件备份为 `.bak` 文件后，用新模板替换
