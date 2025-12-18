# 任务细则：将任务计划文件清理功能迁移到 aide CLI

## 目标

将 finish 环节的任务计划文件清理功能从 command/run.md 中的手动指令迁移到 aide Python CLI 工具中自动执行。

## 背景

当前状态：
- `run.md` 第 412-418 行描述了清理任务计划文件的步骤，由 Claude 手动执行
- 这是不合理的设计，应该由工具自动完成

目标状态：
- 当执行 `aide flow next-part finish` 时，自动清理任务计划文件
- 清理操作在 git commit 之前执行，变更会被记录

## 具体步骤

### 步骤 1：修改 hooks.py

在 `aide-program/aide/flow/hooks.py` 中：

1. 添加新函数 `_hook_clean_task_plans`：
   - 从 config 获取 `task.plans_path`（默认 `.aide/task-plans`）
   - 检查目录是否存在
   - 删除目录下所有文件（保留目录本身）
   - 输出清理结果

2. 修改 `run_pre_commit_hooks` 函数：
   - 添加条件：当 `to_phase == "finish"` 且 `action == "next-part"` 时
   - 调用 `_hook_clean_task_plans`

### 步骤 2：修改 run.md

在 `aide-marketplace/aide-plugin/commands/run.md` 中：

- 删除第 413-418 行的手动清理指令
- 保留其他收尾任务描述

### 步骤 3：修改 auto-run.md

在 `aide-marketplace/aide-plugin/commands/auto-run.md` 中：

- 同步删除相应的手动清理指令（如有）

## 验证标准

1. 执行 `aide flow next-part finish` 时能自动清理任务计划文件
2. 清理操作的变更被 git commit 记录
3. 目录不存在或为空时不报错
4. run.md 和 auto-run.md 中无手动清理指令

## 影响范围

- `aide-program/aide/flow/hooks.py`
- `aide-marketplace/aide-plugin/commands/run.md`
- `aide-marketplace/aide-plugin/commands/auto-run.md`
