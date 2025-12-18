# 步骤 007：aide-marketplace - guide.puml

## 元信息

| 属性 | 值 |
|------|-----|
| 状态 | pending |
| 所属区块 | aide-marketplace |
| 流程图类型 | guide |
| 预估工作量 | 小 |
| 依赖步骤 | 无 |

## 任务描述

绘制 aide-marketplace 的插件架构图，展示 Commands 和 Skills 的组件关系。

## 模块结构（已分析）

以下是执行本步骤所需的全部模块信息：

### 涉及文件

| 文件路径 | 职责 | 关键内容 |
|----------|------|----------|
| `aide-marketplace/.claude-plugin/marketplace.json` | 市场配置 | 市场元数据 |
| `aide-marketplace/aide-plugin/.claude-plugin/plugin.json` | 插件配置 | 插件元数据（版本 2.1.3） |
| `aide-marketplace/aide-plugin/commands/*.md` | 斜杠命令 | 8 个命令定义 |
| `aide-marketplace/aide-plugin/skills/*/SKILL.md` | 技能定义 | 5 个技能定义 |

### 模块关系

```
aide-marketplace (插件市场)
  └→ aide-plugin (Aide 插件)
       ├→ Commands (斜杠命令)
       │    ├→ /aide:setup    → 触发 env-config skill
       │    ├→ /aide:load     → 触发 aide skill
       │    ├→ /aide:docs     → 触发 aide skill
       │    ├→ /aide:run      → 触发 aide + rework skills
       │    ├→ /aide:auto-run → 触发 aide + task-parser skills
       │    ├→ /aide:readme   → 触发 readme-templates skill
       │    ├→ /aide:user-docs → 触发 readme-templates skill
       │    └→ /aide:user-graph → (无特定 skill)
       │
       └→ Skills (技能)
            ├→ aide           → aide 命令用法指南
            ├→ env-config     → 环境配置详细指南
            ├→ readme-templates → README 模板选择
            ├→ rework         → 返工流程处理
            └→ task-parser    → 口语化内容解析
```

### 数据流

```
用户输入 → /aide:xxx 命令 → Command 定义流程步骤
  → 按需触发 Skill → 提供详细指南
  → 调用 aide CLI → 执行具体操作
```

### 关键组件

| 组件 | 类型 | 说明 |
|------|------|------|
| `/aide:run` | Command | 核心任务执行流程 |
| `/aide:auto-run` | Command | 全自动任务执行 |
| `/aide:setup` | Command | 环境配置流程 |
| `aide` | Skill | 命令基础用法 |
| `env-config` | Skill | 环境配置指南 |
| `readme-templates` | Skill | README 模板体系 |

### 设计原则

```
Commands（命令）：
- 定义"做什么"和"按什么顺序做"
- 描述流程步骤和触发条件

Skills（技能）：
- 定义"怎么调用工具"
- 提供详细的操作指南和模板

分工关系：
- Command 是流程的"目录"
- Skill 是流程的"参考手册"
```

## 输出要求

- 文件：`aide-marketplace/guide.puml`
- 类型：组件图 (component diagram)
- 内容要求：
  - [ ] 展示 aide-plugin 整体结构
  - [ ] 展示 Commands 和 Skills 的关系
  - [ ] 展示命令与技能的触发关系
  - [ ] 标注核心命令和技能

## PlantUML 模板

```plantuml
@startuml guide
skinparam defaultFontName "PingFang SC"
skinparam dpi 300
scale 0.5

title aide-plugin 架构概览

' TODO: 基于上述模块结构绘制组件图

@enduml
```

## 执行记录

| 时间 | 操作 | 备注 |
|------|------|------|
| | | |
