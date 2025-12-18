# ccoptimize 项目文档

> 最后更新：2025-12-18

## 项目概述

**ccoptimize** 是 Aide 工作流工具优化项目，包含 Aide 命令行工具的核心实现和 Claude Code 插件。

### 项目统计

| 指标 | 数值 |
|------|------|
| 总文件数 | 139（排除忽略项） |
| 总目录数 | 47（含 5 个空目录） |
| 代码行数 | 约 21000 行 |
| 主要语言 | Python |
| 被忽略项 | 7 个目录 |

## 区块索引

本文档系统将项目划分为以下区块：

| 区块 | 路径 | 文档 | 说明 |
|------|------|------|------|
| aide-program | `aide-program/` | [查看](blocks/aide-program.md) | 核心命令行工具实现 |
| aide-marketplace | `aide-marketplace/` | [查看](blocks/aide-marketplace.md) | 插件市场和 aide-plugin |
| 项目配置与文档 | `.aide/`, `docs/` 等 | [查看](blocks/project-config-docs.md) | 配置文件和项目文档 |

## 快速导航

### 按功能

| 功能 | 位置 | 说明 |
|------|------|------|
| 环境检测 | `aide-program/aide/env/` | 检测 Python/Node/Rust 等环境 |
| 流程追踪 | `aide-program/aide/flow/` | 任务状态管理和 Git 集成 |
| 待定项确认 | `aide-program/aide/decide/` | Web 界面交互式决策 |
| 斜杠命令 | `aide-marketplace/aide-plugin/commands/` | `/aide:run` 等命令定义 |
| 技能定义 | `aide-marketplace/aide-plugin/skills/` | aide、env-config、task-parser |

### 按文件类型

| 类型 | 数量 | 位置 |
|------|------|------|
| Python 源码 | ~35 | `aide-program/aide/` |
| Markdown 命令 | 4 | `aide-marketplace/aide-plugin/commands/` |
| Markdown 技能 | 3 | `aide-marketplace/aide-plugin/skills/` |
| 配置文件 | ~5 | `.aide/`, 根目录 |
| 文档 | ~15 | `docs/`, `aide-program/docs/` |

## 目录结构

```
ccoptimize/
├── .aide/                           项目级 Aide 配置
│   ├── config.toml                  配置文件
│   ├── branches.json                分支概况数据
│   ├── branches.md                  分支概况文档
│   ├── pending-items.json           待定项数据
│   ├── diagrams/                    PlantUML 流程图
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
├── aide-marketplace/                插件市场（~21 文件）
│   ├── .claude-plugin/              市场配置
│   └── aide-plugin/                 Aide 插件（版本 2.1.0）
│       ├── commands/                斜杠命令
│       ├── skills/                  技能定义
│       └── docs/                    插件文档
├── docs/                            项目文档
│   ├── aide-overview.md             系统概述
│   ├── project-details.md           详细说明
│   └── 01-04 指南系列               Claude Code 指南
├── statements/                      [空目录] 声明文档
├── discuss/                         [空目录] 讨论记录
├── reply/                           [空目录] 回复记录
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
| `discuss/` | 项目讨论记录 |
| `reply/` | 项目回复记录 |
| `statements/` | 声明文档 |

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
