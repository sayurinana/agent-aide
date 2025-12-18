# 步骤 004：aide-program - flow.puml

## 元信息

| 属性 | 值 |
|------|-----|
| 状态 | pending |
| 所属区块 | aide-program |
| 流程图类型 | 模块图 |
| 预估工作量 | 大 |
| 依赖步骤 | step-001 |

## 任务描述

绘制 aide-program 的流程追踪核心流程图，展示 FlowTracker 协调存储、Git 和钩子的工作流程。

## 模块结构（已分析）

以下是执行本步骤所需的全部模块信息：

### 涉及文件

| 文件路径 | 职责 | 关键内容 |
|----------|------|----------|
| `aide-program/aide/flow/tracker.py` | 流程追踪器 | FlowTracker 类（233 行） |
| `aide-program/aide/flow/storage.py` | 状态存储 | FlowStorage 类（147 行） |
| `aide-program/aide/flow/validator.py` | 流程校验 | PhaseValidator 类（55 行） |
| `aide-program/aide/flow/git.py` | Git 集成 | GitManager 类（79 行） |
| `aide-program/aide/flow/hooks.py` | 环节钩子 | PlantUML/CHANGELOG 钩子（148 行） |
| `aide-program/aide/flow/types.py` | 数据结构 | FlowStatus, FlowAction 等（103 行） |
| `aide-program/aide/flow/errors.py` | 错误类型 | FlowError（9 行） |
| `aide-program/aide/flow/utils.py` | 工具函数 | 时间戳处理（19 行） |

### 模块关系

```
FlowTracker (协调器)
  ├→ FlowStorage (状态存储)
  │    ├→ load_status() 加载状态
  │    ├→ save_status() 保存状态
  │    └→ archive_existing_status() 归档
  │
  ├→ PhaseValidator (流程校验)
  │    └→ validate_transition() 校验环节跳转
  │
  ├→ GitManager (Git 集成)
  │    ├→ add_all() 暂存变更
  │    └→ commit() 提交
  │
  └→ Hooks (环节钩子)
       ├→ PlantUML 校验/构建
       └→ CHANGELOG 更新
```

### 数据流

```
用户命令 (e.g., aide flow next-step "完成")
  → FlowTracker.next_step(summary)
     → 1. storage.load_status() 加载当前状态
     → 2. 更新状态（step + 1, 记录操作）
     → 3. storage.save_status() 保存状态
     → 4. git.add_all() 暂存变更
     → 5. git.commit(message) 提交
     → 6. 输出结果

环节跳转 (e.g., aide flow next-part impl "进入实现")
  → FlowTracker.next_part(phase, summary)
     → 1. storage.load_status()
     → 2. validator.validate_transition(current, target) 校验
     → 3. hooks.run_phase_hooks(target) 执行钩子
     → 4. 更新状态（phase, step=1）
     → 5. storage.save_status()
     → 6. git.add_all() + git.commit()
     → 7. 输出结果
```

### 关键函数/类

| 名称 | 位置 | 说明 |
|------|------|------|
| `FlowTracker` | tracker.py:20 | 流程追踪协调器 |
| `start()` | tracker.py | 开始新任务 |
| `next_step()` | tracker.py | 步骤前进 |
| `back_step()` | tracker.py | 步骤回退 |
| `next_part()` | tracker.py | 环节前进 |
| `back_part()` | tracker.py | 环节回退（生成确认 key） |
| `back_confirm()` | tracker.py | 确认返工 |
| `FlowStorage` | storage.py:16 | 状态文件管理 |
| `PhaseValidator` | validator.py | 环节跳转校验 |
| `GitManager` | git.py | Git 操作封装 |

### 环节流转规则

```
有效的环节跳转：
  task-optimize → flow-design
  flow-design → impl
  impl → verify
  verify → docs
  docs → confirm
  confirm → finish

有效的回退：
  任意环节 → 之前的环节（需要 back-confirm）

无效跳转示例：
  flow-design → finish (跳过环节)
  impl → task-optimize (跨多环节回退需确认)
```

## 输出要求

- 文件：`aide-program/flow.puml`
- 类型：活动图 (activity diagram)
- 内容要求：
  - [ ] 展示 FlowTracker 主要操作流程
  - [ ] 展示 storage/validator/git/hooks 协作
  - [ ] 展示环节跳转和校验逻辑
  - [ ] 标注关键分支点

## PlantUML 模板

```plantuml
@startuml flow
skinparam defaultFontName "PingFang SC"
skinparam dpi 300
scale 0.5

title aide flow 核心流程

' TODO: 基于上述模块结构绘制活动图

@enduml
```

## 执行记录

| 时间 | 操作 | 备注 |
|------|------|------|
| | | |
