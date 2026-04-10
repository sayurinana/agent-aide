# Implementation Tasks

## 1. 实现 Codex 同步功能

- [ ] 1.1 在 `aide/src/cli/init.rs` 中添加 `sync_plugins_to_codex` 函数
  - 处理全局目录：`~/.codex/prompts/`
  - 处理项目目录：`.agents/skills/`
  - 复用 `copy_dir_all` 函数
  - 添加错误处理和日志输出

- [ ] 1.2 在 `handle_init` 函数中集成 Codex 同步
  - 在 `sync_plugins_to_project` 之后调用
  - 确保不影响现有的 Claude Code 同步流程

- [ ] 1.3 在 `handle_init_global` 函数中集成 Codex 同步
  - 同步 commands 到 `~/.codex/prompts/`
  - 在 `sync_plugin_repo` 之后调用

## 2. 测试验证

- [ ] 2.1 测试项目初始化场景
  - 验证 `.agents/skills/` 目录创建和文件复制
  - 验证 `~/.codex/prompts/` 目录创建和文件复制
  - 验证目录不存在时自动创建

- [ ] 2.2 测试全局初始化场景
  - 验证 `aide init --global` 时同步到 `~/.codex/prompts/`

- [ ] 2.3 测试错误处理
  - 验证全局插件仓库不存在时的警告输出
  - 验证目录创建失败时的错误处理
  - 验证文件复制失败时的警告输出

- [ ] 2.4 测试兼容性
  - 验证不影响现有的 Claude Code 同步功能
  - 验证 Codex 同步失败不影响整体初始化流程

## 3. 文档更新

- [ ] 3.1 更新用户文档说明 Codex 支持
- [ ] 3.2 在初始化输出中添加 Codex 同步的提示信息
