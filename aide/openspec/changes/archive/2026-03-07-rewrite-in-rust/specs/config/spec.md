## ADDED Requirements

### Requirement: 初始化命令

`aide init` SHALL 执行以下操作：
1. 创建 `.aide/` 目录
2. 创建 `.aide/decisions/` 子目录
3. 创建 `.aide/logs/` 子目录
4. 生成默认 `config.toml`（如不存在）
5. 根据 `general.gitignore_aide` 配置更新 `.gitignore`

初始化完成后输出 `✓ 初始化完成，.aide/ 与默认配置已准备就绪`。
首次创建配置时额外输出 `✓ 已创建默认配置 .aide/config.toml`。

#### Scenario: 首次初始化
- **WHEN** 项目目录中不存在 `.aide/` 目录
- **THEN** 创建 `.aide/`、`.aide/decisions/`、`.aide/logs/` 目录
- **AND** 生成带有详细注释的默认 `config.toml`
- **AND** 输出 `✓ 已创建默认配置 .aide/config.toml`
- **AND** 输出 `✓ 初始化完成，.aide/ 与默认配置已准备就绪`

#### Scenario: 重复初始化
- **WHEN** `.aide/` 目录和 `config.toml` 已存在
- **THEN** 不覆盖现有配置
- **AND** 确保子目录存在
- **AND** 输出 `✓ 初始化完成，.aide/ 与默认配置已准备就绪`

### Requirement: 配置读取

`aide config get <key>` SHALL 使用点分隔键值表示法读取 TOML 配置值。

支持的键示例：`flow.phases`、`decide.port`、`plantuml.jar_path`。

读取成功时输出 `→ key = value`。键不存在时输出警告。

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

### Requirement: 配置写入

`aide config set <key> <value>` SHALL 更新 TOML 配置值，保留文件中的已有注释。

值的类型自动推断：
- `true` / `false` → 布尔值
- 纯数字 → 整数
- 含小数点的数字 → 浮点数
- 其他 → 字符串

写入时 SHALL 使用 `toml_edit` crate 保留注释和格式。

写入成功时输出 `✓ 已更新 key = value`。

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

### Requirement: 配置文件格式

配置文件 `.aide/config.toml` SHALL 包含以下段落和默认值：

```toml
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

默认配置 SHALL 包含详细的中文注释说明每个配置项的用途。

#### Scenario: 默认配置生成
- **WHEN** 运行 `aide init` 首次初始化
- **THEN** 生成的 `config.toml` 包含所有上述段落和默认值
- **AND** 包含中文注释

#### Scenario: gitignore 管理
- **WHEN** `general.gitignore_aide = false`
- **THEN** 不修改 `.gitignore`
- **WHEN** `general.gitignore_aide = true`（默认为 false）
- **THEN** 确保 `.gitignore` 包含 `.aide/` 条目
