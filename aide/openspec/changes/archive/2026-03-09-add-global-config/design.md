## Context

aide 当前的配置管理完全基于项目目录。`ConfigManager` 通过 `new(root)` 接收项目根路径，所有路径（`aide_dir`、`config_path`、`backups_dir` 等）均基于该根路径构建。需要扩展此架构以支持全局配置目录 `~/.aide/`。

## Goals / Non-Goals

- Goals:
  - 复用 `ConfigManager` 现有逻辑处理全局配置
  - 全局配置和项目配置格式完全一致
  - `--global` 标志在 `init` 和所有 `config` 子命令上统一可用
  - `aide init` 时自动将全局配置复制到项目目录

- Non-Goals:
  - 不实现分层配置合并（全局 + 项目自动合并）
  - 不实现 `aide config list` 或配置项枚举功能
  - 不修改 `flow` 或 `decide` 模块

## Decisions

### 决策 1：复用 ConfigManager

通过 `ConfigManager::new_global()` 构造函数创建以 `~/.aide/` 为根目录的实例，复用 `ensure_config()`、`load_config()`、`get_value()`、`set_value()` 等现有方法。

- 替代方案：创建独立的 `GlobalConfigManager` 结构体
- 选择理由：全局配置和项目配置的文件格式、操作逻辑完全一致，无需新建结构体。`ConfigManager` 的所有方法都基于 `self.root` 路径，只需改变根路径即可复用。

### 决策 2：获取主目录的方式

使用 `std::env::var("HOME")` 获取主目录路径，不引入额外的 `dirs` crate。

- 替代方案：引入 `dirs` crate 的 `home_dir()` 函数
- 选择理由：项目当前不依赖 `dirs` crate，且目标平台为 Unix（已通过 `cfg(unix)` 依赖 `libc`），`$HOME` 在 Unix 环境下可靠。在 `$HOME` 不可用时返回明确错误，由调用方处理。

### 决策 3：init 命令中全局配置的 schema 版本检测

当全局配置存在但 `meta.schema_version` 低于当前版本时，`aide init` 仅输出提示信息，不自动升级。

- 替代方案 A：自动升级全局配置后再复制
- 替代方案 B：忽略版本差异直接复制
- 选择理由：自动升级可能影响用户其他项目；直接复制则可能传播过期配置。提示用户手动执行 `aide config update --global` 是最安全的做法。

### 决策 4：全局目录结构

全局目录 `~/.aide/` 采用与项目目录 `.aide/` 相同的子目录结构（`backups/`、`logs/` 等），即使部分子目录在全局上下文中暂不使用。

- 选择理由：`ensure_base_dirs()` 会创建所有子目录，保持一致性避免特殊处理。未来若需在全局级别使用这些目录（如全局操作日志），无需额外修改。

## Risks / Trade-offs

- `$HOME` 未设置时（如某些容器环境）`aide init` 和 `--global` 相关操作会失败
  - 缓解：返回明确错误信息 `✗ 无法获取用户主目录，请确保 $HOME 环境变量已设置`
  - 不影响已存在项目配置的正常使用（`aide config get/set` 无 `--global` 时仍走项目路径）

- `aide init` 执行路径变长（先处理全局再处理项目）
  - 缓解：仅增加一次文件存在性检查和一次文件复制，性能影响可忽略

## Open Questions

无。所有关键决策已在用户澄清中确认。
