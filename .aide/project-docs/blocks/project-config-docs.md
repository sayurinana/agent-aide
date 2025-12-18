# 项目配置与文档

> 路径：`.aide/`, `docs/`, `discuss/`, `reply/`, `statements/`, 根目录配置文件
> 最后更新：2025-12-18

## 概述

项目级配置、Aide 数据目录和项目文档资源。包括 Aide 配置、项目说明文档、Claude Code 指南文档和开发声明。

## 目录结构

```
ccoptimize/
├── .aide/                           Aide 配置和数据目录
│   ├── config.toml                  项目配置文件（225 行，全注释）
│   ├── branches.json                分支概况数据（JSON）
│   ├── branches.md                  分支概况文档（Markdown）
│   ├── pending-items.json           待定项数据
│   ├── diagrams/                    流程图目录
│   │   ├── *.puml                   PlantUML 源文件
│   │   └── *.png                    生成的图片
│   ├── logs/                        历史任务归档
│   │   └── flow-status.*.json       归档任务文件
│   └── project-docs/                项目文档（本文档所在）
│       ├── README.md                总导览
│       ├── block-plan.md            区块计划
│       └── blocks/                  子区块文档
│           ├── aide-program.md
│           ├── aide-marketplace.md
│           └── project-config-docs.md
├── docs/                            项目文档目录
│   ├── aide-overview.md             Aide 系统概述（136 行）
│   ├── project-details.md           项目详细说明（约 300 行）
│   ├── 01-自定义斜杠命令指南.md     Claude Code 命令指南（304 行）
│   ├── 02-技能指南.md               Claude Code 技能指南（约 400 行）
│   ├── 03-插件指南.md               Claude Code 插件指南（约 350 行）
│   ├── 04-插件市场指南.md           Claude Code 市场指南（约 320 行）
│   └── 为什么要更换到command+skill+专用处理程序.md  设计背景
├── discuss/                         [空目录] 讨论记录目录
├── reply/                           [空目录] 回复记录目录
├── statements/                      [空目录] 声明文档目录
├── AGENTS.md                        Agent 配置说明
├── CHANGELOG.md                     变更日志
├── CLAUDE.md                        Claude 配置指令
├── README.md                        项目说明
├── requirements.txt                 Python 依赖
├── task-now.md                      当前任务文档
└── .gitignore                       Git 忽略规则
```

## 文件清单

| 文件 | 类型 | 说明 |
|------|------|------|
| .aide/config.toml | 配置 | 项目配置，全注释自文档化 |
| .aide/branches.json | 数据 | 分支概况数据（JSON 格式） |
| .aide/branches.md | 文档 | 分支概况文档（Markdown 格式） |
| .aide/pending-items.json | 数据 | 待定项数据 |
| .aide/diagrams/ | 目录 | PlantUML 流程图 |
| .aide/logs/ | 目录 | 历史任务归档 |
| .aide/project-docs/ | 目录 | 面向 LLM 的项目文档 |
| docs/aide-overview.md | 文档 | Aide 系统架构和设计理念 |
| docs/project-details.md | 文档 | 项目详细说明和实现状态 |
| docs/01-自定义斜杠命令指南.md | 文档 | Claude Code 斜杠命令指南 |
| docs/02-技能指南.md | 文档 | Claude Code 技能指南 |
| docs/03-插件指南.md | 文档 | Claude Code 插件指南 |
| docs/04-插件市场指南.md | 文档 | Claude Code 市场指南 |
| docs/为什么要更换...md | 文档 | 架构设计背景说明 |
| discuss/ | 目录 | [空目录] 讨论记录 |
| reply/ | 目录 | [空目录] 回复记录 |
| statements/ | 目录 | [空目录] 声明文档 |
| AGENTS.md | 配置 | Agent 配置说明 |
| CHANGELOG.md | 文档 | 变更日志 |
| CLAUDE.md | 配置 | Claude 指令（简体中文要求） |
| README.md | 文档 | 项目说明和快速上手 |
| requirements.txt | 配置 | Python 依赖（tomli-w） |
| task-now.md | 文档 | 当前任务描述 |
| .gitignore | 配置 | Git 忽略规则 |

## 核心配置

### .aide/config.toml

Aide 的核心配置文件，完全自文档化（所有配置项均有详细注释）。

**配置节说明**：

| 配置节 | 说明 |
|--------|------|
| `[general]` | 通用配置（gitignore_aide） |
| `[runtime]` | 运行时要求（python_min, use_uv） |
| `[task]` | 任务文档路径（source, spec） |
| `[env]` | 环境检测模块配置 |
| `[docs]` | 项目文档路径配置 |
| `[flow]` | 流程追踪配置（phases, diagram_path） |
| `[plantuml]` | PlantUML 配置（jar_path） |
| `[decide]` | 待定项确认配置（port, bind, url, timeout） |

**当前环境配置**：
```toml
[env]
modules = ["python", "uv", "venv", "requirements"]

[env.venv]
path = ".venv"

[env.requirements]
path = "requirements.txt"
```

### CLAUDE.md

Claude 配置指令：
- 所有对话、思考、文档与注释必须使用简体中文
- 复杂或多模块任务必须调用 Sequential-Thinking
- Python 脚本必须使用 uv 管理的虚拟环境
- 涉及多文件操作必须创建临时 .sh 脚本

### .gitignore

忽略规则：
- `anthropic-agent-skills/` - 第三方技能库
- `__pycache__/` - Python 字节码
- `.venv/` - 虚拟环境
- `test-cache/` - 测试缓存

## 项目文档

### docs/aide-overview.md

Aide 系统概述，包含：
- 系统简介和解决的问题
- 核心设计原则（渐进式披露、确定性封装、信息隔离）
- 系统架构图
- 子区块索引
- 快速导航

### docs/project-details.md

项目详细说明，包含：
- 项目架构
- 实现状态
- 技术决策

### Claude Code 指南（01-04）

Claude Code 功能的完整指南系列：
1. 自定义斜杠命令：命令语法、frontmatter、参数用法、高级功能
2. 技能指南：SKILL.md 格式、资源文件、触发机制
3. 插件指南：plugin.json、目录结构、安装方式
4. 插件市场指南：marketplace.json、托管方式

## 空目录说明

| 目录 | 用途推断 |
|------|----------|
| .aide/decisions/ | 待定项决策记录存放目录 |
| .aide/diagrams/ | PlantUML 流程图存放目录 |
| discuss/ | 项目讨论记录存放目录 |
| reply/ | 项目回复记录存放目录 |
| statements/ | 声明文档目录（历史文件已清理） |

## 依赖关系

- **依赖**：无
- **被依赖**：aide-program（读取 config.toml）、aide-plugin（引用文档路径）

## 注意事项

1. **配置修改**：不要直接编辑 config.toml，使用 `aide config set` 命令
2. **流程图**：.puml 文件会被 aide flow 自动校验和构建
3. **文档同步**：更新代码后注意同步更新 CHANGELOG.md
4. **中文要求**：遵循 CLAUDE.md 中的简体中文要求
