# aide-marketplace（插件市场）

> 路径：`aide-marketplace/`
> 最后更新：2025-12-18

## 概述

Aide 插件市场目录，包含 aide-plugin 插件的源码。aide-plugin 是 Claude Code 插件，为 Aide 工作流体系提供 Commands（斜杠命令）和 Skills（技能）。

## 目录结构

```
aide-marketplace/
├── .claude-plugin/                  市场配置
│   └── marketplace.json             市场元数据
└── aide-plugin/                     Aide 插件
    ├── .claude-plugin/              插件配置
    │   └── plugin.json              插件元数据（版本 2.1.0）
    ├── commands/                    斜杠命令定义
    │   ├── auto-run.md              /aide:auto-run 全自动任务执行（572 行）
    │   ├── docs.md                  /aide:docs 文档管理（400 行）
    │   ├── load.md                  /aide:load 项目认知载入（96 行）
    │   ├── run.md                   /aide:run 任务执行（392 行）
    │   └── setup.md                 /aide:setup 环境配置（93 行）
    ├── skills/                      技能定义
    │   ├── aide/                    基础命令指南
    │   │   └── SKILL.md             aide skill（569 行）
    │   ├── env-config/              环境配置指南
    │   │   └── SKILL.md             env-config skill（299 行）
    │   └── task-parser/             口语化内容解析
    │       └── SKILL.md             task-parser skill（280 行）
    └── docs/                        插件文档
        ├── README.md                设计文档导览（207 行）
        ├── commands/                命令文档
        │   ├── docs.md
        │   ├── load.md
        │   ├── run.md
        │   ├── setup.md
        │   ├── _deprecated_exec.md  [废弃]
        │   ├── _deprecated_init.md  [废弃]
        │   └── _deprecated_prep.md  [废弃]
        └── skill/
            └── aide.md              aide skill 文档
```

## 文件清单

| 文件 | 类型 | 说明 |
|------|------|------|
| .claude-plugin/marketplace.json | 配置 | 市场元数据，定义市场名和插件列表 |
| aide-plugin/.claude-plugin/plugin.json | 配置 | 插件元数据，版本 2.1.0 |
| aide-plugin/commands/auto-run.md | Command | /aide:auto-run 全自动任务执行流程 |
| aide-plugin/commands/docs.md | Command | /aide:docs 项目文档管理流程 |
| aide-plugin/commands/load.md | Command | /aide:load 项目认知载入流程 |
| aide-plugin/commands/run.md | Command | /aide:run 任务执行核心流程 |
| aide-plugin/commands/setup.md | Command | /aide:setup 环境配置流程 |
| aide-plugin/skills/aide/SKILL.md | Skill | aide 命令使用指南 |
| aide-plugin/skills/env-config/SKILL.md | Skill | 环境配置详细指南 |
| aide-plugin/skills/task-parser/SKILL.md | Skill | 口语化内容解析器 |
| aide-plugin/docs/README.md | 文档 | 插件设计文档导览 |

## 核心组件

### Commands（斜杠命令）

| 命令 | 职责 | 触发 Skill | 独立运行 |
|------|------|------------|----------|
| `/aide:setup` | 环境配置（分析、检测、修复） | env-config | 是 |
| `/aide:load` | 项目认知载入 | aide | 否（由 run 调用） |
| `/aide:docs` | 项目文档创建和维护 | aide | 是 |
| `/aide:run` | 任务执行（核心命令） | aide | 否 |
| `/aide:auto-run` | 全自动任务执行 | aide, task-parser | 是 |

### /aide:run 工作流程

```
task-optimize → flow-design → impl → verify → docs → confirm → finish
    │               │
    ├─ 任务分析      ├─ 流程图设计
    ├─ 复杂度评估    └─ PlantUML 校验
    ├─ 待定项处理
    └─ 生成任务细则
```

### /aide:auto-run 工作流程

`/aide:run` 的自动化版本，去除所有需要用户参与的环节：

```
task-optimize → flow-design → impl → verify → docs → finish
    │                                              │
    ├─ 待定项自动决策                               └─ 跳过 confirm 阶段
    └─ 任务细则自动确认
```

**与 /aide:run 的区别**：
- 待定项自动决策（无需用户 Web 确认）
- 任务细则自动确认（无需用户确认）
- 跳过 confirm 阶段（无需用户验收）
- 错误自动处理（优先委托子代理，否则自行解决）

### Skills（技能）

| 技能 | 触发方式 | 职责 |
|------|----------|------|
| aide | /aide:run 强制触发 | aide 命令基础用法 |
| env-config | /aide:setup 强制触发 | 环境配置详细指南 |
| task-parser | 检测到口语化内容时 | 口语化任务内容解析 |

### aide skill

提供 aide 命令行工具的完整使用指南，包括：

- `aide env` - 环境管理（ensure/list/set）
- `aide flow` - 进度追踪（start/next-step/back-step/next-part/back-part/issue/error/status/list/show）
- `aide decide` - 待定项确认（submit/result）
- `aide config` - 配置管理（get/set）
- `aide init` - 初始化

### env-config skill

提供详细的环境配置指导：

- 模块分类（类型A：全局工具检测，类型B：项目级检测）
- 项目类型配置示例
- 多项目场景处理
- node_deps 模块详解
- 故障排除指南

### task-parser skill

用于解析口语化任务内容：

- 语义解析（表层理解、深层提取、结构重组）
- 批判性分析（逻辑漏洞检测、盲点识别）
- 建设性优化（优化方向建议、替代方案对比）
- 上下文关联分析

## 接口说明

### 插件元数据格式

**marketplace.json**:
```json
{
  "name": "aide-marketplace",
  "plugins": [
    {"name": "aide-plugin", "source": "./aide-plugin"}
  ]
}
```

**plugin.json**:
```json
{
  "name": "aide-plugin",
  "version": "2.1.0",
  "description": "Aide 工作流体系插件"
}
```

## 依赖关系

- **依赖**：aide-program（aide 命令行工具）
- **被依赖**：无

## 注意事项

1. **Commands 与 Skills 分工**：
   - Commands 定义"做什么"和"按什么顺序做"
   - Skills 定义"怎么调用工具"

2. **版本管理**：当前版本 2.1.0
   - 2.0.8 → 2.1.0：新增 `/aide:auto-run` 全自动任务执行命令
   - 原 `/aide:init`、`/aide:prep`、`/aide:exec` 已重组为 `/aide:setup`、`/aide:load`、`/aide:docs`、`/aide:run`

3. **触发机制**：Skills 按需触发，避免信息过载
