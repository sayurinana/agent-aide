# ccoptimize 项目导览

> 本文档面向 LLM，用于快速了解项目结构和脉络。
> 最后更新：2025-12-17

## 项目简介

ccoptimize 是 Aide 工具的开发项目。Aide 是一套命令行工具集，用于支持 LLM 辅助开发的工作流体系。项目提供环境管理、进度追踪、待定项确认等功能，并通过 Claude Code 插件形式提供斜杠命令和技能定义。

## 技术栈

- 语言：Python 3.11+
- 包管理：uv
- HTTP 服务：Python 标准库（http.server）
- 前端：原生 HTML/CSS/JavaScript
- 配置格式：TOML
- 流程图：PlantUML

## 项目结构（简化版）

```
ccoptimize/
├── aide-program/                Aide CLI 核心程序
│   ├── aide/                    Python 包
│   │   ├── core/                核心模块（配置、输出）
│   │   ├── env/                 环境管理模块
│   │   ├── flow/                进度追踪模块
│   │   └── decide/              待定项确认模块
│   ├── bin/                     可执行脚本
│   ├── docs/                    命令文档
│   └── lib/                     第三方库
├── aide-marketplace/            插件市场组件
│   └── aide-plugin/             Aide 插件定义
│       ├── commands/            斜杠命令
│       └── skills/              技能定义
├── anthropic-agent-skills/      [ignored] 第三方技能库
├── .aide/                       项目 Aide 配置
├── .venv/                       [ignored] 虚拟环境
├── cache/                       [ignored] 缓存目录
├── CLAUDE.md                    Claude 指令
├── README.md                    项目说明
└── requirements.txt             Python 依赖
```

> 详细结构见各区块文档

## 架构概述

```
┌─────────────────────────────────────────────────────────────┐
│                     Claude Code 插件层                        │
├──────────────────────────┬──────────────────────────────────┤
│      commands/           │           skills/                 │
│  ┌──────────────────┐   │   ┌──────────────────────────┐   │
│  │ /aide:setup      │   │   │ aide (工具使用指南)       │   │
│  │ /aide:load       │   │   │ env-config (环境配置)     │   │
│  │ /aide:docs       │   │   │ task-parser (任务解析)    │   │
│  │ /aide:run        │   │   └──────────────────────────┘   │
│  └──────────────────┘   │                                   │
├─────────────────────────┴───────────────────────────────────┤
│                      Aide CLI 程序层                          │
├──────────┬──────────┬─────────────┬────────────────────────┤
│   core   │   env    │    flow     │        decide          │
│ 配置管理  │ 环境检测  │  进度追踪   │      待定项确认         │
│ 输出格式  │ 模块注册  │  Git 集成   │      Web 服务          │
└──────────┴──────────┴─────────────┴────────────────────────┘
```

## 区块索引

| 区块 | 路径 | 文件数 | 说明 |
|------|------|--------|------|
| [aide-program-core](./blocks/aide-program-core.md) | aide-program/aide/core/ | 6 | 核心配置和输出模块 |
| [aide-program-env](./blocks/aide-program-env.md) | aide-program/aide/env/ | 13 | 环境检测和管理模块 |
| [aide-program-flow](./blocks/aide-program-flow.md) | aide-program/aide/flow/ | 9 | 进度追踪和流程控制模块 |
| [aide-program-decide](./blocks/aide-program-decide.md) | aide-program/aide/decide/ | 10 | 待定项确认 Web 服务模块 |
| [aide-plugin-commands](./blocks/aide-plugin-commands.md) | aide-marketplace/aide-plugin/commands/ | 4 | 斜杠命令定义 |
| [aide-plugin-skills](./blocks/aide-plugin-skills.md) | aide-marketplace/aide-plugin/skills/ | 3 | 技能定义 |

## 快速导航

- 想了解 aide 命令实现 → 查看 [aide-program-core](./blocks/aide-program-core.md)
- 想修改环境检测逻辑 → 查看 [aide-program-env](./blocks/aide-program-env.md)
- 想修改进度追踪功能 → 查看 [aide-program-flow](./blocks/aide-program-flow.md)
- 想修改待定项确认界面 → 查看 [aide-program-decide](./blocks/aide-program-decide.md)
- 想新增斜杠命令 → 查看 [aide-plugin-commands](./blocks/aide-plugin-commands.md)
- 想新增技能定义 → 查看 [aide-plugin-skills](./blocks/aide-plugin-skills.md)

## 核心命令体系

| 命令 | 说明 | 独立运行 |
|------|------|----------|
| `/aide:setup` | 环境配置（分析、检测、修复） | 是 |
| `/aide:load` | 项目认知载入 | 否（由 run 调用） |
| `/aide:docs` | 项目文档创建和维护 | 是 |
| `/aide:run` | 任务执行（核心命令） | 是 |

## 统计信息

- 总目录数：约 50（核心项目）
- 总文件数：约 45（核心项目）
- 被忽略项：anthropic-agent-skills、.venv、cache、__pycache__
- 代码行数：约 5000 行（核心项目）
