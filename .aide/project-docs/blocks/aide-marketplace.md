# aide-marketplace（插件市场）

> 路径：`aide-marketplace/`
> 最后更新：2025-12-19

## 概述

Aide 插件市场目录，包含 aide-plugin 插件的源码。aide-plugin 是 Claude Code 插件，为 Aide 工作流体系提供 Commands（斜杠命令）和 Skills（技能）。

## 目录结构

```
aide-marketplace/
├── .claude-plugin/                  市场配置
│   └── marketplace.json             市场元数据
└── aide-plugin/                     Aide 插件
    ├── .claude-plugin/              插件配置
    │   └── plugin.json              插件元数据（版本 2.1.3）
    ├── commands/                    斜杠命令定义（8 个）
    │   ├── auto-run.md              /aide:auto-run 全自动任务执行（566 行）
    │   ├── docs.md                  /aide:docs 文档管理（402 行）
    │   ├── load.md                  /aide:load 项目认知载入（95 行）
    │   ├── readme.md                /aide:readme README 生成（253 行）
    │   ├── run.md                   /aide:run 任务执行（557 行）
    │   ├── setup.md                 /aide:setup 环境配置（92 行）
    │   ├── user-docs.md             /aide:user-docs 用户文档生成（401 行）
    │   └── user-graph.md            /aide:user-graph 用户流程图生成（329 行）
    ├── skills/                      技能定义（5 个）
    │   ├── aide/                    基础命令指南
    │   │   └── SKILL.md             aide skill（603 行）
    │   ├── env-config/              环境配置指南
    │   │   └── SKILL.md             env-config skill（298 行）
    │   ├── readme-templates/        README 模板集
    │   │   ├── SKILL.md             readme-templates skill（106 行）
    │   │   ├── templates/           基础模板（5 个）
    │   │   │   ├── application.md   应用程序模板
    │   │   │   ├── documentation.md 文档模板
    │   │   │   ├── library.md       库模板
    │   │   │   ├── minimal.md       最小模板
    │   │   │   └── monorepo.md      多项目仓库模板
    │   │   └── modules/             可选模块（10 个）
    │   │       ├── module-api.md
    │   │       ├── module-architecture.md
    │   │       ├── module-changelog.md
    │   │       ├── module-configuration.md
    │   │       ├── module-contributing.md
    │   │       ├── module-examples.md
    │   │       ├── module-faq.md
    │   │       ├── module-installation.md
    │   │       ├── module-license.md
    │   │       └── module-quickstart.md
    │   ├── rework/                  返工流程指南
    │   │   └── SKILL.md             rework skill（165 行）
    │   └── task-parser/             口语化内容解析
    │       └── SKILL.md             task-parser skill（279 行）
    └── docs/                        插件文档
        ├── README.md                设计文档导览
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
| aide-plugin/.claude-plugin/plugin.json | 配置 | 插件元数据，版本 2.1.3 |
| aide-plugin/commands/auto-run.md | Command | /aide:auto-run 全自动任务执行流程 |
| aide-plugin/commands/docs.md | Command | /aide:docs 项目文档管理流程 |
| aide-plugin/commands/load.md | Command | /aide:load 项目认知载入流程 |
| aide-plugin/commands/readme.md | Command | /aide:readme README 生成流程 |
| aide-plugin/commands/run.md | Command | /aide:run 任务执行核心流程 |
| aide-plugin/commands/setup.md | Command | /aide:setup 环境配置流程 |
| aide-plugin/commands/user-docs.md | Command | /aide:user-docs 用户文档生成流程 |
| aide-plugin/commands/user-graph.md | Command | /aide:user-graph 用户流程图生成流程 |
| aide-plugin/skills/aide/SKILL.md | Skill | aide 命令使用指南 |
| aide-plugin/skills/env-config/SKILL.md | Skill | 环境配置详细指南 |
| aide-plugin/skills/readme-templates/SKILL.md | Skill | README 模板选择和使用指南 |
| aide-plugin/skills/rework/SKILL.md | Skill | 返工流程处理指南 |
| aide-plugin/skills/task-parser/SKILL.md | Skill | 口语化内容解析器 |
| aide-plugin/docs/README.md | 文档 | 插件设计文档导览 |

## 核心组件

### Commands（斜杠命令）

| 命令 | 职责 | 触发 Skill | 独立运行 |
|------|------|------------|----------|
| `/aide:setup` | 环境配置（分析、检测、修复） | env-config | 是 |
| `/aide:load` | 项目认知载入 | aide | 否（由 run 调用） |
| `/aide:docs` | 项目文档创建和维护 | aide | 是 |
| `/aide:run` | 任务执行（核心命令） | aide, rework | 否 |
| `/aide:auto-run` | 全自动任务执行 | aide, task-parser | 是 |
| `/aide:readme` | README 生成 | readme-templates | 是 |
| `/aide:user-docs` | 用户文档生成 | readme-templates | 是 |
| `/aide:user-graph` | 用户流程图生成 | - | 是 |

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

### /aide:readme 工作流程

```
检查规范文件 → 规范引导/README生成
     │
     ├─ 不存在 → 分析项目 → 推荐模板 → 创建规范
     └─ 存在 → 读取规范 → 加载项目信息 → 生成README
```

### /aide:user-docs 工作流程

支持分步执行和接续执行：

```
检查计划文件 → 分析和计划/接续执行
     │
     ├─ 不存在 → 分析项目 → 规划结构 → 生成计划
     └─ 存在 → 读取进度 → 继续执行步骤
```

### /aide:user-graph 工作流程

用户流程图生成，支持分步执行：

```
检查计划文件 → 分析和计划/接续执行
     │
     ├─ 不存在 → 区块划分 → 复杂度分析 → 生成计划
     └─ 存在 → 读取进度 → 继续执行步骤
```

### Skills（技能）

| 技能 | 触发方式 | 职责 |
|------|----------|------|
| aide | /aide:run 强制触发 | aide 命令基础用法 |
| env-config | /aide:setup 强制触发 | 环境配置详细指南 |
| readme-templates | /aide:readme 强制触发 | README 模板选择和使用指南 |
| rework | 返工场景时触发 | 返工流程处理指南 |
| task-parser | 检测到口语化内容时 | 口语化任务内容解析 |

### aide skill

提供 aide 命令行工具的完整使用指南，包括：

- `aide env` - 环境管理（ensure/list/set）
- `aide flow` - 进度追踪（start/next-step/back-step/next-part/back-part/back-confirm/issue/error/status/list/show）
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

### readme-templates skill

提供 README 模板体系：

**基础模板**（5 个）：
- `minimal.md` - 小工具/脚本
- `library.md` - 库/SDK
- `application.md` - 应用程序
- `documentation.md` - 文档/教程
- `monorepo.md` - 多项目仓库

**可选模块**（10 个）：
- quickstart、installation、examples、api、configuration
- architecture、contributing、changelog、license、faq

### rework skill

提供返工流程处理指南：

- 返工类型判断（task-optimize/flow-design/impl/verify）
- task.source 更新格式
- new-requirements.md 格式
- 确认机制（back-part → back-confirm）
- 用户提醒模板

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
  "version": "2.1.3",
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

2. **版本管理**：当前版本 2.1.3
   - 2.1.3：新增 `/aide:readme`、`/aide:user-docs`、`/aide:user-graph` 命令，新增 `readme-templates`、`rework` 技能
   - 2.1.0：新增 `/aide:auto-run` 全自动任务执行命令
   - 原 `/aide:init`、`/aide:prep`、`/aide:exec` 已重组为 `/aide:setup`、`/aide:load`、`/aide:docs`、`/aide:run`

3. **触发机制**：Skills 按需触发，避免信息过载

4. **分步执行支持**：`/aide:user-docs` 和 `/aide:user-graph` 支持分步执行和接续执行，适用于大型项目
