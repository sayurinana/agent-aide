# 步骤 008：aide-marketplace - commands.puml

## 元信息

| 属性 | 值 |
|------|-----|
| 状态 | pending |
| 所属区块 | aide-marketplace |
| 流程图类型 | 模块图 |
| 预估工作量 | 中 |
| 依赖步骤 | step-007 |

## 任务描述

绘制 aide-marketplace 的核心命令执行流程图，重点展示 /aide:run 和 /aide:auto-run 的工作流程。

## 模块结构（已分析）

以下是执行本步骤所需的全部模块信息：

### 涉及文件

| 文件路径 | 职责 | 关键内容 |
|----------|------|----------|
| `aide-marketplace/aide-plugin/commands/run.md` | /aide:run | 任务执行核心流程（557 行） |
| `aide-marketplace/aide-plugin/commands/auto-run.md` | /aide:auto-run | 全自动任务执行（566 行） |
| `aide-marketplace/aide-plugin/commands/setup.md` | /aide:setup | 环境配置流程（92 行） |
| `aide-marketplace/aide-plugin/commands/load.md` | /aide:load | 项目认知载入（95 行） |
| `aide-marketplace/aide-plugin/commands/docs.md` | /aide:docs | 文档管理（402 行） |

### /aide:run 工作流程

```
task-optimize (任务优化)
  ├→ 任务分析
  ├→ 复杂度评估
  ├→ 待定项处理 (aide decide)
  └→ 生成任务细则

    ↓

flow-design (流程设计)
  ├→ 流程图设计 (PlantUML)
  └→ 校验 puml 文件

    ↓

impl (迭代实现)
  ├→ 按流程图实现
  └→ 小步骤前进 (aide flow next-step)

    ↓

verify (验证交付)
  ├→ 测试验证
  └→ 问题记录 (aide flow issue/error)

    ↓

docs (文档更新)
  ├→ 更新文档
  └→ CHANGELOG 更新

    ↓

confirm (用户确认)
  └→ 等待用户验收

    ↓

finish (收尾)
  └→ 任务完成
```

### /aide:auto-run 与 /aide:run 的区别

```
/aide:run (有人工参与)：
- 待定项通过 Web 界面确认
- 任务细则需用户确认
- 有 confirm 阶段
- 错误需人工处理

/aide:auto-run (全自动)：
- 待定项自动决策（选推荐项）
- 任务细则自动确认
- 跳过 confirm 阶段
- 错误自动处理（委托子代理或自行解决）

流程对比：
run:      task-optimize → flow-design → impl → verify → docs → confirm → finish
auto-run: task-optimize → flow-design → impl → verify → docs → finish
```

### 关键环节

| 环节 | 说明 | aide 命令 |
|------|------|-----------|
| task-optimize | 任务优化 | `aide flow start task-optimize` |
| flow-design | 流程设计 | `aide flow next-part flow-design` |
| impl | 迭代实现 | `aide flow next-part impl` |
| verify | 验证交付 | `aide flow next-part verify` |
| docs | 文档更新 | `aide flow next-part docs` |
| confirm | 用户确认 | `aide flow next-part confirm` |
| finish | 收尾 | `aide flow next-part finish` |

### 数据流

```
任务文档 (task-now.md)
  → 读取任务描述
  → 分析和优化
  → 生成任务细则 (.aide/task-plans/)
  → 按细则执行
  → 更新进度 (aide flow next-step)
  → 完成任务 (aide flow next-part finish)
```

## 输出要求

- 文件：`aide-marketplace/commands.puml`
- 类型：活动图 (activity diagram)
- 内容要求：
  - [ ] 展示 /aide:run 完整流程
  - [ ] 标注各环节的 aide flow 命令
  - [ ] 对比标注 /aide:auto-run 的差异
  - [ ] 展示环节间的跳转条件

## PlantUML 模板

```plantuml
@startuml commands
skinparam defaultFontName "PingFang SC"
skinparam dpi 300
scale 0.3

title /aide:run 任务执行流程

' TODO: 基于上述模块结构绘制活动图

@enduml
```

## 执行记录

| 时间 | 操作 | 备注 |
|------|------|------|
| | | |
