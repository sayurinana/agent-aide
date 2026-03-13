## ADDED Requirements

### Requirement: 全局配置管理

系统 SHALL 支持全局配置文件 `$HOME/.aide/config.toml`，与项目配置格式完全一致。

全局配置目录路径通过 `std::env::var("HOME")` 获取主目录后拼接 `.aide/` 得到。当 `$HOME` 环境变量不可用时，SHALL 输出错误 `✗ 无法获取用户主目录，请确保 $HOME 环境变量已设置` 并返回失败。

`ConfigManager::new_global()` SHALL 创建以 `$HOME` 为根目录的实例，使 `aide_dir` 指向 `$HOME/.aide/`，复用现有的 `ensure_config()`、`load_config()`、`get_value()`、`set_value()` 等方法。

#### Scenario: 获取全局配置目录
- **WHEN** `$HOME` 环境变量为 `/home/user`
- **THEN** 全局配置目录为 `/home/user/.aide/`
- **AND** 全局配置文件路径为 `/home/user/.aide/config.toml`

#### Scenario: HOME 环境变量不可用
- **WHEN** `$HOME` 环境变量未设置
- **AND** 执行任何需要全局配置的操作
- **THEN** 输出 `✗ 无法获取用户主目录，请确保 $HOME 环境变量已设置`
- **AND** 返回失败

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
- **AND** 输出 `✓ 全局配置初始化完成`
- **AND** 不修改当前工作目录

#### Scenario: 全局初始化但已存在（--global）
- **WHEN** 运行 `aide init --global`
- **AND** 全局配置已存在
- **THEN** 输出 `→ 全局配置已存在：{path}`
- **AND** 不修改任何文件

### Requirement: 配置读取

`aide config get <key>` SHALL 使用点分隔键值表示法读取 TOML 配置值。

当指定 `--global` 标志时，SHALL 从全局配置 `$HOME/.aide/config.toml` 中读取。未指定时从项目配置读取。

支持的键示例：`flow.phases`、`decide.port`、`plantuml.jar_path`。

读取成功时输出 `→ key = value`。键不存在时输出警告。全局配置文件不存在时输出错误。

#### Scenario: 读取简单值
- **WHEN** 运行 `aide config get decide.port`
- **AND** 配置中 `[decide]` 段的 `port = 3721`
- **THEN** 输出 `→ decide.port = 3721`

#### Scenario: 读取数组值
- **WHEN** 运行 `aide config get flow.phases`
- **AND** 配置中 `flow.phases = ["task-optimize", "flow-design", "impl"]`
- **THEN** 输出 `→ flow.phases = ["task-optimize", "flow-design", "impl"]`

#### Scenario: 读取不存在的键
- **WHEN** 运行 `aide config get nonexistent.key`
- **THEN** 输出 `⚠` 前缀的警告信息

#### Scenario: 读取全局配置值
- **WHEN** 运行 `aide config get --global decide.port`
- **AND** 全局配置中 `[decide]` 段的 `port = 4000`
- **THEN** 输出 `→ decide.port = 4000`

#### Scenario: 全局配置文件不存在时读取
- **WHEN** 运行 `aide config get --global decide.port`
- **AND** 全局配置文件不存在
- **THEN** 输出 `✗` 前缀的错误信息

### Requirement: 配置写入

`aide config set <key> <value>` SHALL 更新 TOML 配置值，保留文件中的已有注释。

当指定 `--global` 标志时，SHALL 修改全局配置 `$HOME/.aide/config.toml`。未指定时修改项目配置。

值的类型自动推断：
- `true` / `false` → 布尔值
- 纯数字 → 整数
- 含小数点的数字 → 浮点数
- 其他 → 字符串

写入时 SHALL 使用 `toml_edit` crate 保留注释和格式。

写入成功时输出 `✓ 已更新 key = value`。全局配置文件不存在时输出错误。

#### Scenario: 设置布尔值
- **WHEN** 运行 `aide config set general.gitignore_aide true`
- **THEN** 配置文件中 `general.gitignore_aide = true`
- **AND** 原有注释保留
- **AND** 输出 `✓ 已更新 general.gitignore_aide = true`

#### Scenario: 设置字符串值
- **WHEN** 运行 `aide config set task.source my-task.md`
- **THEN** 配置文件中 `task.source = "my-task.md"`

#### Scenario: 设置嵌套键
- **WHEN** 运行 `aide config set env.venv.path .venv-custom`
- **THEN** 配置文件中 `[env.venv]` 段下 `path = ".venv-custom"`

#### Scenario: 设置全局配置值
- **WHEN** 运行 `aide config set --global decide.port 4000`
- **AND** 全局配置文件存在
- **THEN** 全局配置文件中 `decide.port = 4000`
- **AND** 输出 `✓ 已更新 decide.port = 4000`

#### Scenario: 全局配置文件不存在时写入
- **WHEN** 运行 `aide config set --global decide.port 4000`
- **AND** 全局配置文件不存在
- **THEN** 输出 `✗` 前缀的错误信息

### Requirement: 配置重置命令

`aide config reset` SHALL 将配置文件重置为默认值，并自动备份现有配置。

当指定 `--global` 标志时，SHALL 仅对全局配置 `$HOME/.aide/config.toml` 进行重置，备份到 `$HOME/.aide/backups/`。未指定时仅对项目配置操作。

执行流程：
1. 检查目标配置文件是否存在
2. 如存在，备份到对应的 `backups/` 目录（`config.toml.{timestamp}`）
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

#### Scenario: 重置全局配置
- **WHEN** 运行 `aide config reset --global`
- **AND** 全局配置文件存在
- **THEN** 备份到 `$HOME/.aide/backups/config.toml.{timestamp}`
- **AND** 重置全局配置为默认值
- **AND** 不影响当前工作目录下的配置文件

#### Scenario: 全局配置重置（配置不存在）
- **WHEN** 运行 `aide config reset --global`
- **AND** 全局配置文件不存在
- **THEN** 在 `$HOME/.aide/` 下创建默认配置
- **AND** 输出 `✓ 已创建默认配置`

### Requirement: 配置更新命令

`aide config update` SHALL 检测配置版本差异，并更新配置文件以匹配当前 aide 版本。

当指定 `--global` 标志时，SHALL 仅对全局配置 `$HOME/.aide/config.toml` 进行更新。未指定时仅对项目配置操作。

执行流程：
1. 读取目标配置文件中的 `meta.aide_version` 和 `meta.schema_version`
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

#### Scenario: 更新全局配置
- **WHEN** 运行 `aide config update --global`
- **AND** 全局配置中 `meta.schema_version` 低于当前版本
- **THEN** 更新全局配置到最新 schema 版本
- **AND** 不影响当前工作目录下的配置文件
