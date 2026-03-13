# config Specification Delta

## MODIFIED Requirements

### Requirement: 初始化命令

`aide init` SHALL 执行以下操作：
1. 创建 `.aide/` 目录
2. 创建 `.aide/decisions/` 子目录
3. 创建 `.aide/logs/` 子目录
4. 创建 `.aide/backups/` 子目录（用于配置备份）
5. 生成简洁的 `config.toml`（如不存在）
6. 生成详细的 `config.md` 配置说明文档
7. 根据 `general.gitignore_aide` 配置更新 `.gitignore`

初始化完成后输出 `✓ 初始化完成，.aide/ 与默认配置已准备就绪`。
首次创建配置时额外输出 `✓ 已创建默认配置 .aide/config.toml` 和 `✓ 已创建配置说明 .aide/config.md`。

#### Scenario: 首次初始化
- **WHEN** 项目目录中不存在 `.aide/` 目录
- **THEN** 创建 `.aide/`、`.aide/decisions/`、`.aide/logs/`、`.aide/backups/` 目录
- **AND** 生成简洁的 `config.toml`（仅包含配置项和简短注释）
- **AND** 生成详细的 `config.md` 说明文档
- **AND** 输出 `✓ 已创建默认配置 .aide/config.toml`
- **AND** 输出 `✓ 已创建配置说明 .aide/config.md`
- **AND** 输出 `✓ 初始化完成，.aide/ 与默认配置已准备就绪`

#### Scenario: 重复初始化
- **WHEN** `.aide/` 目录和 `config.toml` 已存在
- **THEN** 不覆盖现有配置
- **AND** 确保子目录存在
- **AND** 输出 `✓ 初始化完成，.aide/ 与默认配置已准备就绪`

### Requirement: 配置文件格式

配置文件 `.aide/config.toml` SHALL 采用简洁格式，包含以下段落和默认值：

```toml
[meta]
aide_version = "0.1.0"
schema_version = 1

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
jar_path = ""
font_name = "Arial"
dpi = 300
scale = 0.5

[decide]
port = 3721
bind = "127.0.0.1"
url = ""
timeout = 0
```

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
- **AND** 配置文件总行数不超过 50 行
- **AND** 仅包含简短的行内注释

#### Scenario: 配置文档生成
- **WHEN** 运行 `aide init` 首次初始化
- **THEN** 生成的 `config.md` 包含所有配置项的详细说明
- **AND** 按配置节分组组织内容
- **AND** 每个配置项包含完整的说明和示例

#### Scenario: 版本元数据
- **WHEN** 生成新配置文件
- **THEN** `[meta]` 节包含当前 aide 版本号
- **AND** 包含当前配置 schema 版本号

## ADDED Requirements

### Requirement: 配置重置命令

`aide config reset` SHALL 将配置文件重置为默认值，并自动备份现有配置。

执行流程：
1. 检查 `.aide/config.toml` 是否存在
2. 如存在，备份到 `.aide/backups/config.toml.{timestamp}`
3. 显示确认提示（除非使用 `--force` 标志）
4. 生成新的默认 `config.toml`
5. 重新生成 `config.md`
6. 输出备份位置和重置成功信息

#### Scenario: 重置配置（有确认）
- **WHEN** 运行 `aide config reset`
- **AND** 配置文件存在
- **THEN** 显示确认提示 `⚠ 此操作将重置配置到默认值，现有配置将备份。是否继续？[y/N]`
- **WHEN** 用户输入 `y`
- **THEN** 备份现有配置到 `.aide/backups/config.toml.{timestamp}`
- **AND** 生成新的默认配置
- **AND** 重新生成 `config.md`
- **AND** 输出 `✓ 已备份配置到 .aide/backups/config.toml.{timestamp}`
- **AND** 输出 `✓ 配置已重置为默认值`

#### Scenario: 强制重置（跳过确认）
- **WHEN** 运行 `aide config reset --force`
- **THEN** 不显示确认提示
- **AND** 直接执行备份和重置操作

#### Scenario: 配置文件不存在
- **WHEN** 运行 `aide config reset`
- **AND** 配置文件不存在
- **THEN** 直接生成默认配置
- **AND** 输出 `✓ 已创建默认配置`

### Requirement: 配置更新命令

`aide config update` SHALL 检测配置版本差异，并更新配置文件以匹配当前 aide 版本。

执行流程：
1. 读取配置文件中的 `meta.aide_version` 和 `meta.schema_version`
2. 比较与当前 aide 版本和 schema 版本
3. 如版本相同，输出 `✓ 配置已是最新版本`
4. 如版本不同，执行迁移：
   - 添加新引入的配置项（使用默认值）
   - 注释掉废弃的配置项（添加废弃说明）
   - 保留用户自定义的配置值
5. 更新 `meta` 节的版本信息
6. 重新生成 `config.md`
7. 输出更新摘要

#### Scenario: 配置需要更新
- **WHEN** 运行 `aide config update`
- **AND** 配置中 `meta.schema_version = 1`
- **AND** 当前 aide 的 schema 版本为 2
- **THEN** 输出 `→ 检测到配置版本差异：当前 schema v1，最新 schema v2`
- **AND** 添加新配置项到相应的配置节
- **AND** 注释掉废弃配置项，添加 `# [已废弃] 说明`
- **AND** 保留用户修改过的配置值
- **AND** 更新 `meta.schema_version = 2`
- **AND** 更新 `meta.aide_version` 为当前版本
- **AND** 重新生成 `config.md`
- **AND** 输出 `✓ 配置已更新到 schema v2`
- **AND** 输出 `✓ 已更新配置说明 .aide/config.md`

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

### Requirement: 配置版本管理

配置系统 SHALL 维护两个版本标识：
- `aide_version`：生成配置的 aide 程序版本（如 "0.1.0"）
- `schema_version`：配置结构的 schema 版本（整数，从 1 开始）

Schema 版本变更规则：
- 添加新配置项：schema 版本递增
- 移除配置项：schema 版本递增
- 修改配置项语义：schema 版本递增
- 仅修改默认值：schema 版本不变

配置迁移逻辑 SHALL 基于 schema 版本差异执行。

#### Scenario: 版本信息记录
- **WHEN** 生成新配置文件
- **THEN** `[meta]` 节包含当前 aide 版本
- **AND** 包含当前 schema 版本

#### Scenario: 版本检测
- **WHEN** 执行 `aide config update`
- **THEN** 读取配置中的 `meta.schema_version`
- **AND** 与当前 aide 的 schema 版本比较
- **AND** 确定是否需要迁移
