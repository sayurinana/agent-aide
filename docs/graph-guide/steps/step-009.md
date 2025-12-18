# 步骤 009：aide-marketplace - skills.puml

## 元信息

| 属性 | 值 |
|------|-----|
| 状态 | pending |
| 所属区块 | aide-marketplace |
| 流程图类型 | 模块图 |
| 预估工作量 | 小 |
| 依赖步骤 | step-007 |

## 任务描述

绘制 aide-marketplace 的技能体系图，展示各 Skill 的触发机制和作用范围。

## 模块结构（已分析）

以下是执行本步骤所需的全部模块信息：

### 涉及文件

| 文件路径 | 职责 | 关键内容 |
|----------|------|----------|
| `aide-marketplace/aide-plugin/skills/aide/SKILL.md` | aide 技能 | 命令使用指南（603 行） |
| `aide-marketplace/aide-plugin/skills/env-config/SKILL.md` | env-config 技能 | 环境配置指南（298 行） |
| `aide-marketplace/aide-plugin/skills/readme-templates/SKILL.md` | readme-templates 技能 | README 模板（106 行） |
| `aide-marketplace/aide-plugin/skills/rework/SKILL.md` | rework 技能 | 返工流程指南（165 行） |
| `aide-marketplace/aide-plugin/skills/task-parser/SKILL.md` | task-parser 技能 | 口语化解析（279 行） |

### 技能体系

```
Skills (技能)
  │
  ├→ aide (基础)
  │    ├→ 触发：/aide:run, /aide:load, /aide:docs 强制触发
  │    ├→ 内容：aide env/flow/decide/config 命令用法
  │    └→ 作用：提供 CLI 操作指南
  │
  ├→ env-config (环境)
  │    ├→ 触发：/aide:setup 强制触发，或 aide env ensure 失败时
  │    ├→ 内容：模块配置、项目类型示例、故障排除
  │    └→ 作用：提供环境配置详细指导
  │
  ├→ readme-templates (模板)
  │    ├→ 触发：/aide:readme, /aide:user-docs 强制触发
  │    ├→ 内容：5 个基础模板 + 10 个可选模块
  │    └→ 作用：提供 README 生成模板
  │
  ├→ rework (返工)
  │    ├→ 触发：检测到返工场景时
  │    ├→ 内容：返工类型判断、确认机制、用户提醒
  │    └→ 作用：提供返工流程指导
  │
  └→ task-parser (解析)
       ├→ 触发：检测到口语化内容时
       ├→ 内容：语义解析、批判性分析、建设性优化
       └→ 作用：将口语化内容转为结构化任务
```

### 触发机制

| 技能 | 触发方式 | 触发条件 |
|------|----------|----------|
| aide | 强制触发 | Command 首行声明 |
| env-config | 强制触发/条件触发 | /aide:setup 或 env ensure 失败 |
| readme-templates | 强制触发 | /aide:readme, /aide:user-docs |
| rework | 条件触发 | 检测到返工场景 |
| task-parser | 条件触发 | 检测到口语化内容 |

### 技能内容结构

```
SKILL.md 结构：
  ├→ 概述（何时使用）
  ├→ 详细指南
  │    ├→ 命令/工具用法
  │    ├→ 配置选项
  │    └→ 示例
  └→ 资源文件（如有）
       ├→ 模板文件
       └→ 参考文档
```

### readme-templates 技能详情

```
readme-templates/
  ├→ SKILL.md (选择指南)
  ├→ templates/ (5 个基础模板)
  │    ├→ minimal.md (小工具/脚本)
  │    ├→ library.md (库/SDK)
  │    ├→ application.md (应用程序)
  │    ├→ documentation.md (文档/教程)
  │    └→ monorepo.md (多项目仓库)
  └→ modules/ (10 个可选模块)
       ├→ module-quickstart.md
       ├→ module-installation.md
       ├→ module-examples.md
       ├→ module-api.md
       ├→ module-configuration.md
       ├→ module-architecture.md
       ├→ module-contributing.md
       ├→ module-changelog.md
       ├→ module-license.md
       └→ module-faq.md
```

## 输出要求

- 文件：`aide-marketplace/skills.puml`
- 类型：思维导图 (mindmap) 或组件图
- 内容要求：
  - [ ] 展示 5 个技能的层次结构
  - [ ] 标注各技能的触发方式
  - [ ] 展示技能与命令的关联
  - [ ] 标注 readme-templates 的子结构

## PlantUML 模板

```plantuml
@startuml skills
skinparam defaultFontName "PingFang SC"
skinparam dpi 300
scale 0.5

title aide-plugin 技能体系

' TODO: 基于上述模块结构绘制技能体系图

@enduml
```

## 执行记录

| 时间 | 操作 | 备注 |
|------|------|------|
| | | |
