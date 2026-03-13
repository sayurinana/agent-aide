## MODIFIED Requirements

### Requirement: 初始化命令

`aide init` SHALL 执行以下操作：
1. 检查全局配置 `$HOME/.aide/config.toml` 是否存在
   - 若不存在：创建 `$HOME/.aide/` 目录及子目录，写入默认配置和配置说明文档
   - 若已存在：不做任何操作
2. 检查全局配置的 `meta.schema_version` 是否低于当前 aide 版本的 `CURRENT_SCHEMA_VERSION`
   - 若版本较低：输出 `⚠ 全局配置 schema 版本较低（v{current}），建议执行 aide config update --global 升级`
3. 检查项目配置 `.aide/config.toml` 是否存在
   - 若不存在：创建 `.aide/` 目录及子目录，从全局配置**复制**到 `.aide/config.toml`，生成 `config.md`
   - 若已存在：确保子目录存在，不覆盖现有配置
4. 根据 `general.gitignore_aide` 配置更新 `.gitignore`

`aide init --global` SHALL 仅在用户主目录下操作：
1. 检查 `$HOME/.aide/config.toml` 是否存在
   - 若不存在：创建 `$HOME/.aide/` 目录及子目录，写入默认配置和配置说明文档
   - 若已存在：输出 `→ 全局配置已存在：$HOME/.aide/config.toml`，不做任何修改
2. 不修改当前工作目录下的任何文件
3. 不在当前工作目录创建 `.aide/` 目录
4. 检测 PlantUML 可执行程序是否可用：
   - 若可用：输出 PlantUML 版本信息
   - 若不可用：提示用户 `PlantUML 未安装，是否现在自动下载并安装？[Y/n]`
     - 用户确认（输入 Y 或直接回车）：执行自动下载和安装
     - 用户拒绝（输入 n）：跳过安装，输出提示信息

初始化完成后输出 `✓ 初始化完成，.aide/ 与默认配置已准备就绪`（无 `--global`）或 `✓ 全局配置初始化完成`（`--global`）。

#### Scenario: 首次初始化（含全局配置）
- **WHEN** 全局配置 `$HOME/.aide/config.toml` 不存在
- **AND** 项目目录中不存在 `.aide/` 目录
- **THEN** 创建 `$HOME/.aide/` 目录及子目录，写入默认配置
- **AND** 创建项目 `.aide/` 目录及子目录
- **AND** 从全局配置复制到项目 `.aide/config.toml`
- **AND** 生成项目 `.aide/config.md`
- **AND** 输出 `✓ 初始化完成，.aide/ 与默认配置已准备就绪`

#### Scenario: 全局配置已存在，项目未初始化
- **WHEN** 全局配置 `$HOME/.aide/config.toml` 已存在
- **AND** 项目目录中不存在 `.aide/` 目录
- **THEN** 不修改全局配置
- **AND** 从全局配置复制到项目 `.aide/config.toml`
- **AND** 输出 `✓ 初始化完成，.aide/ 与默认配置已准备就绪`

#### Scenario: 全局配置 schema 版本较低
- **WHEN** 全局配置 `meta.schema_version` 低于 `CURRENT_SCHEMA_VERSION`
- **THEN** 输出 `⚠ 全局配置 schema 版本较低（v{current}），建议执行 aide config update --global 升级`
- **AND** 继续执行后续初始化步骤

#### Scenario: 重复初始化
- **WHEN** `.aide/` 目录和 `config.toml` 已存在
- **THEN** 不覆盖现有项目配置
- **AND** 确保子目录存在
- **AND** 输出 `✓ 初始化完成，.aide/ 与默认配置已准备就绪`

#### Scenario: 全局初始化（--global）
- **WHEN** 运行 `aide init --global`
- **AND** 全局配置不存在
- **THEN** 创建 `$HOME/.aide/` 目录及子目录
- **AND** 写入默认配置和配置说明文档
- **AND** 检测 PlantUML 可用性并提示安装
- **AND** 输出 `✓ 全局配置初始化完成`
- **AND** 不修改当前工作目录

#### Scenario: 全局初始化但已存在（--global）
- **WHEN** 运行 `aide init --global`
- **AND** 全局配置已存在
- **THEN** 输出 `→ 全局配置已存在：{path}`
- **AND** 检测 PlantUML 可用性并提示安装
- **AND** 不修改配置文件

#### Scenario: 全局初始化时 PlantUML 已安装
- **WHEN** 运行 `aide init --global`
- **AND** PlantUML 可执行文件存在且可用
- **THEN** 输出 PlantUML 版本信息（如 `→ PlantUML 已安装: 1.2025.4`）

#### Scenario: 全局初始化时 PlantUML 未安装且用户确认安装
- **WHEN** 运行 `aide init --global`
- **AND** PlantUML 可执行文件不存在
- **AND** 用户在提示后输入 Y 或直接回车
- **THEN** 自动下载 PlantUML 程序包
- **AND** 解压到安装目录
- **AND** 验证安装成功
- **AND** 输出 `✓ PlantUML 安装成功`

#### Scenario: 全局初始化时 PlantUML 未安装且用户拒绝安装
- **WHEN** 运行 `aide init --global`
- **AND** PlantUML 可执行文件不存在
- **AND** 用户在提示后输入 n
- **THEN** 跳过安装
- **AND** 输出 `→ 已跳过 PlantUML 安装，可稍后运行 aide init --global 重新安装`

### Requirement: 配置文件格式

配置文件 `.aide/config.toml` SHALL 采用简洁格式，包含以下段落和默认值：

```toml
[meta]
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
```

`plantuml.download_cache_path`、`plantuml.install_path` 中的相对路径均相对于 `~/.aide/` 全局配置目录。

配置文件 SHALL 仅包含简短的行内注释，不包含详细说明。

配置说明文档 `.aide/config.md` SHALL 包含所有配置项的详细说明，包括：
- 配置项名称和类型
- 默认值
- 用途说明
- 使用示例
- 最佳实践建议

#### Scenario: 简洁配置生成
- **WHEN** 运行 `aide init` 首次初始化
- **THEN** 生成的 `config.toml` 包含 `[meta]` 节和所有配置段落
- **AND** `[plantuml]` 节包含 `download_cache_path`、`clean_cache_after_install`、`install_path`、`download_url`
- **AND** `[plantuml]` 节不包含 `jar_path`
- **AND** 配置文件总行数不超过 50 行

#### Scenario: 配置文档生成
- **WHEN** 运行 `aide init` 首次初始化
- **THEN** 生成的 `config.md` 包含所有配置项的详细说明
- **AND** `[plantuml]` 节说明包含新增配置项的详细解释
- **AND** 说明中包含路径相对于 `~/.aide/` 的规则

#### Scenario: 版本元数据
- **WHEN** 生成新配置文件
- **THEN** `[meta]` 节包含当前 aide 版本号
- **AND** `schema_version = 2`

### Requirement: 配置更新命令

`aide config update` SHALL 检测配置版本差异，并更新配置文件以匹配当前 aide 版本。

当指定 `--global` 标志时，SHALL 仅对全局配置 `$HOME/.aide/config.toml` 进行更新。未指定时仅对项目配置操作。

执行流程：
1. 读取目标配置文件中的 `meta.aide_version` 和 `meta.schema_version`
2. 比较与当前 aide 版本和 schema 版本
3. 如版本相同，输出 `✓ 配置已是最新版本`
4. 如版本不同，执行迁移：
   - 添加新引入的配置项（使用默认值）
   - 移除废弃的配置项
   - 保留用户自定义的配置值
5. 更新 `meta` 节的版本信息
6. 重新生成 `config.md`
7. 输出更新摘要

Schema v1 → v2 迁移：
- 移除 `plantuml.jar_path`
- 添加 `plantuml.download_cache_path`（默认 `"download-buffer"`）
- 添加 `plantuml.clean_cache_after_install`（默认 `true`）
- 添加 `plantuml.install_path`（默认 `"utils"`）
- 添加 `plantuml.download_url`（默认为 GitHub 下载链接）
- 保留 `plantuml.font_name`、`plantuml.dpi`、`plantuml.scale`

#### Scenario: 配置需要更新（v1 → v2）
- **WHEN** 运行 `aide config update`
- **AND** 配置中 `meta.schema_version = 1`
- **AND** 当前 aide 的 schema 版本为 2
- **THEN** 输出 `→ 检测到配置版本差异：当前 schema v1，最新 schema v2`
- **AND** 移除 `plantuml.jar_path`
- **AND** 添加 `plantuml.download_cache_path`、`clean_cache_after_install`、`install_path`、`download_url`
- **AND** 保留用户修改过的 `plantuml.font_name`、`dpi`、`scale`
- **AND** 更新 `meta.schema_version = 2`
- **AND** 重新生成 `config.md`
- **AND** 输出 `✓ 配置已更新到 schema v2`

#### Scenario: 配置已是最新
- **WHEN** 运行 `aide config update`
- **AND** 配置版本与当前 aide 版本匹配
- **THEN** 输出 `✓ 配置已是最新版本（schema v{version}）`
- **AND** 不修改配置文件

#### Scenario: 配置缺少版本信息
- **WHEN** 运行 `aide config update`
- **AND** 配置文件中不存在 `[meta]` 节
- **THEN** 视为旧版本配置（schema v0）
- **AND** 执行完整迁移流程
- **AND** 添加 `[meta]` 节

#### Scenario: 更新全局配置
- **WHEN** 运行 `aide config update --global`
- **AND** 全局配置中 `meta.schema_version` 低于当前版本
- **THEN** 更新全局配置到最新 schema 版本
- **AND** 不影响当前工作目录下的配置文件
