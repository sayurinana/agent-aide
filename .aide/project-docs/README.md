# agent-aide 项目文档

> 最后更新：2025-12-19

## 项目概述

**agent-aide** 是 Aide 工作流工具项目，包含 Aide 命令行工具的核心实现和 Claude Code 插件。

### 项目统计

| 指标 | 数值 |
|------|------|
| 总文件数 | 161（排除忽略项） |
| 总目录数 | 54（含 3 个空目录） |
| 代码行数 | 约 5350 行（Python） |
| 主要语言 | Python |
| 被忽略项 | 7 个目录 |

## 区块索引

本文档系统将项目划分为以下区块：

| 区块 | 路径 | 文档 | 说明 |
|------|------|------|------|
| aide-program | `aide-program/` | [查看](blocks/aide-program.md) | 核心命令行工具实现 |
| aide-marketplace | `aide-marketplace/` | [查看](blocks/aide-marketplace.md) | 插件市场和 aide-plugin（版本 2.1.3） |
| 项目配置与文档 | `.aide/`, `docs/` 等 | [查看](blocks/project-config-docs.md) | 配置文件和项目文档 |

## 快速导航

### 按功能

| 功能 | 位置 | 说明 |
|------|------|------|
| 环境检测 | `aide-program/aide/env/` | 检测 Python/Node/Rust 等环境 |
| 流程追踪 | `aide-program/aide/flow/` | 任务状态管理和 Git 集成 |
| 待定项确认 | `aide-program/aide/decide/` | Web 界面交互式决策 |
| 斜杠命令 | `aide-marketplace/aide-plugin/commands/` | 8 个命令定义 |
| 技能定义 | `aide-marketplace/aide-plugin/skills/` | 5 个技能定义 |

### 按文件类型

| 类型 | 数量 | 位置 |
|------|------|------|
| Python 源码 | ~35 | `aide-program/aide/` |
| Markdown 命令 | 8 | `aide-marketplace/aide-plugin/commands/` |
| Markdown 技能 | 5 | `aide-marketplace/aide-plugin/skills/` |
| 配置文件 | ~5 | `.aide/`, 根目录 |
| 文档 | ~15 | `docs/reference/`, `aide-program/docs/` |

## 目录结构

```
agent-aide/
├── .aide/                           项目级 Aide 配置
│   ├── config.toml                  配置文件
│   ├── branches.json                分支概况数据
│   ├── branches.md                  分支概况文档
│   ├── decisions/                   [空目录] 决策记录
│   ├── diagrams/                    [空目录] PlantUML 流程图
│   ├── task-plans/                  [空目录] 复杂任务计划
│   ├── logs/                        历史任务归档
│   └── project-docs/                本文档目录
├── aide-program/                    核心程序（~72 文件）
│   ├── aide/                        Python 源码
│   │   ├── core/                    核心模块
│   │   ├── env/                     环境检测
│   │   ├── flow/                    流程追踪（含分支管理）
│   │   └── decide/                  待定项确认
│   ├── bin/                         可执行脚本
│   ├── docs/                        程序文档
│   └── lib/                         依赖库
├── aide-marketplace/                插件市场（~39 文件）
│   ├── .claude-plugin/              市场配置
│   └── aide-plugin/                 Aide 插件（版本 2.1.3）
│       ├── commands/                斜杠命令（8 个）
│       ├── skills/                  技能定义（5 个）
│       └── docs/                    插件文档
├── docs/                            项目文档
│   └── reference/                   参考文档（7 个）
├── CLAUDE.md                        Claude 配置指令
├── CHANGELOG.md                     变更日志
├── README.md                        项目说明
└── task-now.md                      当前任务
```

## 核心概念

### Aide 工作流体系

Aide 是一套面向 LLM 驱动开发的工作流工具，核心设计原则：

1. **渐进式披露**：按需加载信息，避免信息过载
2. **确定性封装**：工具调用产生确定性输出
3. **信息隔离**：Commands 定义"做什么"，Skills 定义"怎么做"

### Commands 与 Skills

| 组件 | 职责 | 示例 |
|------|------|------|
| Commands | 定义流程和步骤 | `/aide:run` 定义任务执行流程 |
| Skills | 提供工具使用指南 | `aide` skill 提供 CLI 命令用法 |

### 命令清单

| 命令 | 说明 |
|------|------|
| `/aide:setup` | 环境配置（分析、检测、修复） |
| `/aide:load` | 项目认知载入 |
| `/aide:docs` | 项目文档创建和维护 |
| `/aide:run` | 任务执行（核心命令） |
| `/aide:auto-run` | 全自动任务执行 |
| `/aide:readme` | README 生成 |
| `/aide:user-docs` | 用户文档生成 |
| `/aide:user-graph` | 用户流程图生成 |

### 环境检测模块

| 类型 | 模块 | 说明 |
|------|------|------|
| 类型 A（全局） | python, uv, rust, node, flutter, android | 检测全局工具 |
| 类型 B（项目级） | venv, requirements, node_deps | 项目依赖管理 |

## 空目录清单

| 目录 | 用途 |
|------|------|
| `.aide/decisions/` | 待定项决策记录 |
| `.aide/diagrams/` | PlantUML 流程图 |
| `.aide/task-plans/` | 复杂任务计划文档 |

## 被忽略项

| 目录 | 说明 |
|------|------|
| `anthropic-agent-skills/` | 第三方技能库 |
| `.venv/` | 根目录虚拟环境 |
| `aide-program/.venv/` | 程序虚拟环境 |
| `__pycache__/` | Python 字节码 |
| `cache/`, `.cache/` | 缓存目录 |
| `test-cache/` | 测试缓存目录 |

## 使用指南

### 运行 aide 命令

```bash
# 激活虚拟环境
source aide-program/.venv/bin/activate

# 或使用启动脚本
./aide-program/bin/aide <command>
```

### 常用命令

```bash
aide env ensure          # 检测环境
aide flow start <phase>  # 开始任务
aide flow status         # 查看状态
aide decide submit       # 提交待定项
```

## 文档维护

本文档由 `/aide:docs` 命令生成，遵循"完全深度探索"原则：

- 每个文件和目录都被完整覆盖
- 空目录被明确标注
- 被忽略项被单独列出

更新文档请运行：`/aide:docs`
