# Change: 添加 Codex 支持到 aide init

## Why

当前 `aide init` 仅支持 Claude Code（.claude 目录），但用户可能同时使用多个 AI 编程助手工具（如 Codex）。为了让 aide-plugin 的 commands 和 skills 能够被 Codex 使用，需要在初始化时同步文件到 Codex 的目录结构。

这样可以：
- 让 Codex 用户也能使用 aide-plugin 提供的工作流指导
- 实现多 AI 助手的统一插件管理
- 减少用户手动复制文件的工作量

## What Changes

- 在 `aide init` 流程中添加 Codex 插件同步步骤
- 将 commands 文件复制到 `~/.codex/prompts/` 目录（全局）
- 将 skills 文件复制到项目的 `.agents/skills/` 目录（项目级）
- 在 `aide init --global` 时也同步 commands 到 `~/.codex/prompts/`
- 默认启用 Codex 同步，无需额外配置
- 复用现有的 `copy_dir_all` 函数实现目录复制

## Impact

- **受影响的规范**：`specs/cli/spec.md`
- **受影响的代码**：
  - `aide/src/cli/init.rs` - 添加 `sync_plugins_to_codex` 函数
  - `aide/src/cli/init.rs` - 在 `handle_init` 和 `handle_init_global` 中调用新函数
- **用户体验**：
  - 执行 `aide init` 后，Codex 用户可以直接使用 aide-plugin
  - 不影响仅使用 Claude Code 的用户
  - 目录创建失败时会输出警告但不中断初始化流程
