## ADDED Requirements

### Requirement: CLI 命令结构

系统 SHALL 提供以下顶层命令结构：
- `aide init` — 初始化项目
- `aide config get <key>` — 读取配置值
- `aide config set <key> <value>` — 设置配置值
- `aide flow <subcommand>` — 流程追踪
- `aide decide <subcommand>` — 待定项确认

系统 SHALL 使用 `clap` (derive API) 解析命令行参数，并自动生成帮助信息（`--help`）和版本信息（`--version`）。

#### Scenario: 无参数运行显示帮助
- **WHEN** 用户运行 `aide` 不带任何参数
- **THEN** 显示帮助信息，列出所有可用命令

#### Scenario: 无效命令显示错误
- **WHEN** 用户运行 `aide unknown`
- **THEN** 显示错误信息并提示可用命令

### Requirement: 项目根目录发现

系统 SHALL 通过三阶段算法定位项目根目录：
1. 如果当前目录存在 `.aide/` 目录，使用当前目录
2. 第一轮：向上搜索包含 `.aide/flow-status.json` 的目录（活跃任务）
3. 第二轮：向上搜索包含 `.aide/config.toml` 的目录（有配置）
4. 兜底：返回启动时的当前目录

#### Scenario: 当前目录有 .aide
- **WHEN** 当前目录存在 `.aide/` 目录
- **THEN** 使用当前目录作为项目根目录

#### Scenario: 子目录中运行
- **WHEN** 在 `src/components/` 子目录中运行 aide
- **AND** 上级目录 `../../` 存在 `.aide/config.toml`
- **THEN** 使用包含 `.aide/config.toml` 的目录作为项目根目录

#### Scenario: 优先活跃任务
- **WHEN** 父目录 A 有 `.aide/config.toml`
- **AND** 更上级目录 B 有 `.aide/flow-status.json`
- **THEN** 使用目录 B（活跃任务优先）

### Requirement: 输出格式化

系统 SHALL 使用统一的输出前缀符号：
- `✓` — 成功
- `⚠` — 警告（非阻塞）
- `✗` — 错误
- `→` — 信息/进度
- `[n/m]` — 步骤计数

系统 SHALL 遵循静默成功原则：操作成功时默认不产生输出，仅在需要用户关注时显示信息。

所有输出 SHALL 使用 UTF-8 编码。

#### Scenario: 成功操作静默
- **WHEN** 一个常规操作（如 next-step）成功执行
- **THEN** 不产生任何标准输出

#### Scenario: 错误输出带前缀
- **WHEN** 操作发生错误
- **THEN** 输出以 `✗ ` 前缀的错误信息到标准错误

### Requirement: 错误处理

系统 SHALL 捕获所有预期异常，以格式化错误信息输出，不显示 Rust panic 或堆栈跟踪。

- 操作失败时返回非零退出码
- 用户中断（Ctrl+C）时输出 `✗ 操作已取消` 并返回退出码 1

#### Scenario: 键盘中断
- **WHEN** 用户按下 Ctrl+C
- **THEN** 输出 `✗ 操作已取消`
- **AND** 进程以退出码 1 退出

#### Scenario: 文件 IO 错误
- **WHEN** 读取配置文件失败（如权限不足）
- **THEN** 输出 `✗ ` 前缀的描述性错误信息
- **AND** 进程以非零退出码退出
