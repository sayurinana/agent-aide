# 步骤 006：aide-program - branch.puml

## 元信息

| 属性 | 值 |
|------|-----|
| 状态 | pending |
| 所属区块 | aide-program |
| 流程图类型 | 模块图 |
| 预估工作量 | 中 |
| 依赖步骤 | step-001 |

## 任务描述

绘制 aide-program 的分支管理流程图，展示任务分支的创建、记录和合并流程。

## 模块结构（已分析）

以下是执行本步骤所需的全部模块信息：

### 涉及文件

| 文件路径 | 职责 | 关键内容 |
|----------|------|----------|
| `aide-program/aide/flow/branch.py` | 分支管理 | BranchManager 类（462 行） |
| `aide-program/aide/flow/git.py` | Git 操作 | GitManager 类（79 行） |
| `.aide/branches.json` | 分支数据 | 分支概况 JSON |
| `.aide/branches.md` | 分支文档 | 分支概况 Markdown |

### 模块关系

```
BranchManager
  ├→ GitManager (Git 操作)
  │    ├→ get_current_branch()
  │    ├→ create_branch()
  │    ├→ checkout_branch()
  │    └→ merge_branch()
  │
  ├→ branches.json (数据存储)
  │    └→ 记录分支元数据
  │
  └→ branches.md (文档生成)
       └→ 人类可读的分支概况
```

### 数据流

```
创建任务分支：
  aide flow start → BranchManager.create_task_branch()
    → 1. 检查当前分支状态
    → 2. 生成分支名（aide/{task_id}）
    → 3. GitManager.create_branch()
    → 4. GitManager.checkout_branch()
    → 5. 记录到 branches.json
    → 6. 更新 branches.md

完成任务合并：
  aide flow finish → BranchManager.finish_task_branch()
    → 1. 检查任务完成状态
    → 2. GitManager.checkout_branch(main)
    → 3. GitManager.merge_branch(task_branch)
    → 4. 更新 branches.json 状态
    → 5. 更新 branches.md
```

### 关键函数/类

| 名称 | 位置 | 说明 |
|------|------|------|
| `BranchManager` | branch.py | 分支管理器 |
| `create_task_branch()` | branch.py | 创建任务分支 |
| `finish_task_branch()` | branch.py | 完成任务分支（合并） |
| `get_branch_status()` | branch.py | 获取分支状态 |
| `update_branches_doc()` | branch.py | 更新分支文档 |
| `load_branches_data()` | branch.py | 加载分支数据 |
| `save_branches_data()` | branch.py | 保存分支数据 |

### 分支命名规则

```
分支格式：aide/{task_id}

示例：
- aide/2025-12-15T17-28-53
- aide/2025-12-19T10-00-00

主分支：master 或 main
```

### 分支状态

| 状态 | 说明 |
|------|------|
| active | 活跃任务分支 |
| completed | 已完成并合并 |
| abandoned | 已废弃 |

## 输出要求

- 文件：`aide-program/branch.puml`
- 类型：活动图 (activity diagram)
- 内容要求：
  - [ ] 展示分支创建流程
  - [ ] 展示分支合并流程
  - [ ] 展示 branches.json 和 branches.md 的更新
  - [ ] 标注分支状态转换

## PlantUML 模板

```plantuml
@startuml branch
skinparam defaultFontName "PingFang SC"
skinparam dpi 300
scale 0.3

title aide 分支管理流程

' TODO: 基于上述模块结构绘制活动图

@enduml
```

## 执行记录

| 时间 | 操作 | 备注 |
|------|------|------|
| | | |
