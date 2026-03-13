## MODIFIED Requirements

### Requirement: CLI 命令结构

系统 SHALL 提供以下顶层命令结构：
- `aide init [--global]` — 初始化项目配置，`--global` 时仅初始化全局配置
- `aide config get [--global] <key>` — 读取配置值，`--global` 时读取全局配置
- `aide config set [--global] <key> <value>` — 设置配置值，`--global` 时修改全局配置
- `aide config reset [--global] [--force]` — 重置配置到默认值，`--global` 时重置全局配置
- `aide config update [--global]` — 更新配置到最新版本，`--global` 时更新全局配置
- `aide flow <subcommand>` — 流程追踪
- `aide decide <subcommand>` — 待定项确认

系统 SHALL 使用 `clap` (derive API) 解析命令行参数，并自动生成帮助信息（`--help`）和版本信息（`--version`）。

`--global` 标志 SHALL 在 `init` 和所有 `config` 子命令上统一支持，使用 `#[arg(long)]` 声明。

#### Scenario: 无参数运行显示帮助
- **WHEN** 用户运行 `aide` 不带任何参数
- **THEN** 显示帮助信息，列出所有可用命令

#### Scenario: 无效命令显示错误
- **WHEN** 用户运行 `aide unknown`
- **THEN** 显示错误信息并提示可用命令

#### Scenario: init --global 帮助信息
- **WHEN** 用户运行 `aide init --help`
- **THEN** 帮助信息中包含 `--global` 选项说明
