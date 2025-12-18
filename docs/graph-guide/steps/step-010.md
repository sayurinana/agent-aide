# 步骤 010：project-config-docs - guide.puml

## 元信息

| 属性 | 值 |
|------|-----|
| 状态 | pending |
| 所属区块 | project-config-docs |
| 流程图类型 | guide |
| 预估工作量 | 小 |
| 依赖步骤 | 无 |

## 任务描述

绘制 project-config-docs 的配置体系导航图，展示项目配置和文档的组织结构。

## 模块结构（已分析）

以下是执行本步骤所需的全部模块信息：

### 涉及文件

| 文件路径 | 职责 | 关键内容 |
|----------|------|----------|
| `.aide/config.toml` | 项目配置 | 全注释自文档化配置 |
| `.aide/branches.json` | 分支数据 | 分支概况 JSON |
| `.aide/branches.md` | 分支文档 | 分支概况 Markdown |
| `.aide/project-docs/` | 项目文档 | 面向 LLM 的文档 |
| `docs/reference/` | 参考文档 | Claude Code 指南等 |
| `CLAUDE.md` | Claude 配置 | Claude 指令 |
| `README.md` | 项目说明 | 快速上手 |
| `CHANGELOG.md` | 变更日志 | 版本历史 |

### 配置体系结构

```
项目配置与文档
  │
  ├→ .aide/ (Aide 数据目录)
  │    ├→ config.toml (核心配置)
  │    │    ├→ [general] 通用配置
  │    │    ├→ [runtime] 运行时要求
  │    │    ├→ [task] 任务文档路径
  │    │    ├→ [env] 环境检测配置
  │    │    ├→ [docs] 项目文档配置
  │    │    ├→ [user_docs] 用户文档配置
  │    │    ├→ [flow] 流程追踪配置
  │    │    ├→ [plantuml] PlantUML 配置
  │    │    └→ [decide] 待定项确认配置
  │    │
  │    ├→ branches.json/md (分支概况)
  │    ├→ decisions/ (决策记录)
  │    ├→ diagrams/ (流程图)
  │    ├→ task-plans/ (任务计划)
  │    ├→ logs/ (历史归档)
  │    └→ project-docs/ (LLM 项目文档)
  │
  ├→ docs/ (项目文档)
  │    └→ reference/ (参考文档)
  │         ├→ aide-overview.md
  │         ├→ project-details.md
  │         └→ 01-04 Claude Code 指南
  │
  └→ 根目录配置
       ├→ CLAUDE.md (Claude 指令)
       ├→ README.md (项目说明)
       ├→ CHANGELOG.md (变更日志)
       ├→ AGENTS.md (Agent 配置)
       ├→ requirements.txt (Python 依赖)
       └→ .gitignore (忽略规则)
```

### 配置节说明

| 配置节 | 说明 | 关键配置项 |
|--------|------|------------|
| `[general]` | 通用配置 | gitignore_aide |
| `[runtime]` | 运行时要求 | python_min, use_uv |
| `[task]` | 任务文档 | source, spec, plans_path |
| `[env]` | 环境检测 | modules, venv.path, requirements.path |
| `[docs]` | 项目文档 | path, block_plan_path |
| `[user_docs]` | 用户文档 | readme_path, rules_path, docs_path, graph_path |
| `[flow]` | 流程追踪 | phases, diagram_path |
| `[plantuml]` | PlantUML | jar_path, font_name, dpi, scale |
| `[decide]` | 待定项 | port, bind, url, timeout |

### 数据文件格式

```
日志归档（.aide/logs/）：
  ├→ YYYY-MM-DDTHH-MM-SS-status.json (任务状态)
  └→ YYYY-MM-DDTHH-MM-SS-decisions/ (决策记录)

流程图（.aide/diagrams/）：
  ├→ *.puml (PlantUML 源文件)
  └→ *.png (生成的图片)

任务计划（.aide/task-plans/）：
  ├→ guide.md (任务计划总导览)
  └→ spec-NN.md (子计划细则)
```

## 输出要求

- 文件：`project-config-docs/guide.puml`
- 类型：思维导图 (mindmap)
- 内容要求：
  - [ ] 展示 .aide/ 目录结构
  - [ ] 展示 config.toml 配置节
  - [ ] 展示 docs/ 目录结构
  - [ ] 展示根目录配置文件

## PlantUML 模板

```plantuml
@startuml guide
skinparam defaultFontName "PingFang SC"
skinparam dpi 300
scale 0.5

title 项目配置与文档体系

' TODO: 基于上述结构绘制思维导图

@enduml
```

## 执行记录

| 时间 | 操作 | 备注 |
|------|------|------|
| | | |
