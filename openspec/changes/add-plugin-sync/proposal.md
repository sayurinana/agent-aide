# Proposal: add-plugin-sync

## Summary

增强 `aide init` 命令，支持自动克隆 agent-aide 仓库并将 commands 和 skills 同步到项目目录，实现插件体系的自动化部署。

## Motivation

当前用户需要手动从 agent-aide 仓库复制 commands 和 skills 到项目的 `.claude/` 目录，过程繁琐且难以保持更新。通过自动化这一流程：

1. **简化初始化流程**：用户执行 `aide init` 即可获得最新的 commands 和 skills
2. **统一版本管理**：通过 git 克隆确保插件版本可追溯
3. **便于更新**：用户可通过重新执行 `aide init --global` 更新全局插件库

## Scope

- 修改 `aide init --global` 命令，增加克隆/更新 agent-aide 仓库的步骤
- 修改 `aide init` 项目初始化命令，增加同步 commands 和 skills 到项目 `.claude/` 目录的步骤
- 新增 `[plugin]` 配置段，包含仓库地址等配置项
- 更新 `config.md` 文档说明新增配置项

## Out of Scope

- 插件版本锁定机制（后续可扩展）
- 多插件源支持（后续可扩展）
- 插件热更新机制

## Dependencies

- Git 命令行工具（用户环境需已安装 git）
- 网络访问能力（克隆仓库需要）

## Risks

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| Git 未安装 | 克隆失败 | 检测 git 可用性，给出明确提示 |
| 网络问题 | 克隆超时或失败 | 提供重试提示，允许跳过继续初始化 |
| 仓库地址变更 | 无法拉取最新版本 | 配置项允许用户自定义仓库地址 |
| .claude 目录冲突 | 覆盖用户自定义内容 | 提示用户确认，或备份现有内容 |

## Success Criteria

- [ ] 执行 `aide init --global` 能成功克隆 agent-aide 仓库到 `~/.aide/agent-aide/`
- [ ] 执行 `aide init` 能将 commands 和 skills 复制到项目 `.claude/` 目录
- [ ] 配置文件包含 `[plugin]` 段和 `repo_url` 配置项
- [ ] 仓库地址可通过配置自定义
- [ ] Git 不可用时有明确错误提示