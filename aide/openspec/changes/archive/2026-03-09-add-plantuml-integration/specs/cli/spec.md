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

系统 SHALL 使用 `clap` (derive API) 解析命令行参数，并自动生成帮助信息（`--help`）。

`-V` / `--version` 标志 SHALL 自定义处理，输出 aide 版本信息及 PlantUML 状态信息：
- aide 版本号
- PlantUML 可用时：显示版本号、可执行文件路径、状态为"可用"
- PlantUML 不可用时：显示状态为"未安装"，提示运行 `aide init --global` 安装

输出格式：
```
aide {version}

PlantUML:
  版本: {plantuml_version}
  路径: {plantuml_path}
  状态: 可用
```

或不可用时：
```
aide {version}

PlantUML:
  状态: 未安装
  提示: 运行 aide init --global 安装 PlantUML
```

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

#### Scenario: 版本输出（PlantUML 可用）
- **WHEN** 用户运行 `aide -V` 或 `aide --version`
- **AND** PlantUML 已安装且可用
- **THEN** 输出 aide 版本号
- **AND** 输出 PlantUML 版本号、路径和"可用"状态

#### Scenario: 版本输出（PlantUML 不可用）
- **WHEN** 用户运行 `aide -V` 或 `aide --version`
- **AND** PlantUML 未安装
- **THEN** 输出 aide 版本号
- **AND** 输出 PlantUML 状态为"未安装"
- **AND** 提示运行 `aide init --global` 安装
