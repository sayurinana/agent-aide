# Aide 项目导览

> 本文档面向 LLM，用于快速了解项目结构和脉络。
> 最后更新：2025-12-15

## 项目简介

Aide 是一套面向 Claude Code 的工作流辅助体系，旨在解决 AI 辅助开发中的信息过载、操作不确定性和流程耦合问题。通过模块化的 Commands、Skills 和命令行工具，帮助开发者更高效地进行 AI 辅助开发。

## 技术栈

- **语言**：Python 3.11+, Markdown
- **包管理**：uv
- **配置格式**：TOML
- **前端**：原生 HTML/CSS/JS（decide 模块）
- **流程图**：PlantUML

## 架构概述

```
用户
  │
  ▼
aide-plugin (Claude Code 插件)
  ├── Commands: /aide:setup, /aide:load, /aide:docs, /aide:run
  └── Skills: aide, env-config
  │
  ▼ 调用
aide-program (命令行工具)
  ├── aide init   - 初始化配置
  ├── aide env    - 环境检测（模块化）
  ├── aide config - 配置读写
  ├── aide flow   - 进度追踪 + git 集成
  └── aide decide - 待定项 Web 确认
```

## 区块索引

| 区块 | 路径 | 说明 |
|------|------|------|
| [aide-program-core](./blocks/aide-program-core.md) | aide-program/aide/core/ | 核心模块（配置、输出） |
| [aide-program-env](./blocks/aide-program-env.md) | aide-program/aide/env/ | 环境检测模块 |
| [aide-program-flow](./blocks/aide-program-flow.md) | aide-program/aide/flow/ | 进度追踪模块 |
| [aide-program-decide](./blocks/aide-program-decide.md) | aide-program/aide/decide/ | 待定项确认模块 |
| [aide-plugin-commands](./blocks/aide-plugin-commands.md) | aide-marketplace/aide-plugin/commands/ | 插件命令 |
| [aide-plugin-skills](./blocks/aide-plugin-skills.md) | aide-marketplace/aide-plugin/skills/ | 插件技能 |

## 快速导航

- 想了解配置管理 → 查看 [aide-program-core](./blocks/aide-program-core.md)
- 想了解环境检测 → 查看 [aide-program-env](./blocks/aide-program-env.md)
- 想了解进度追踪 → 查看 [aide-program-flow](./blocks/aide-program-flow.md)
- 想了解待定项确认 → 查看 [aide-program-decide](./blocks/aide-program-decide.md)
- 想了解插件命令 → 查看 [aide-plugin-commands](./blocks/aide-plugin-commands.md)
- 想了解插件技能 → 查看 [aide-plugin-skills](./blocks/aide-plugin-skills.md)

## 核心数据文件

| 文件 | 说明 |
|------|------|
| `.aide/config.toml` | 项目配置（自文档化） |
| `.aide/flow-status.json` | 当前任务进度 |
| `.aide/decisions/` | 待定项决策记录 |
| `.aide/logs/` | 历史任务归档 |
| `.aide/diagrams/` | 流程图目录 |

## 标准工作流程

1. **task-optimize** - 任务优化：分析任务、识别待定项
2. **flow-design** - 流程设计：创建 PlantUML 流程图
3. **impl** - 迭代实现：按计划执行
4. **verify** - 验证交付：对照任务细则验证
5. **docs** - 文档更新：更新相关文档
6. **finish** - 收尾：清理临时文件、汇报完成

## 输出格式约定

| 前缀 | 含义 |
|------|------|
| `✓` | 成功 |
| `⚠` | 警告（可继续） |
| `✗` | 错误（需处理） |
| `→` | 进行中/信息 |
