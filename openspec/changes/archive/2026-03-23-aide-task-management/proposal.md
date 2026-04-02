# 提案：aide 任务管理子命令

## 概述

实现任务管理的三个子命令：verify（审验任务规范性）、confirm（敲定任务）、archive（归档任务）。

## 动机

任务从起草到归档需要规范化的管理流程：
- verify 确保任务文档符合规范
- confirm 将草案转为正式任务并分配编号
- archive 将完成的任务归档

## 目标

1. 实现 `aide verify` 审验命令
2. 实现 `aide confirm` 敲定命令
3. 实现 `aide archive [n]` 归档命令
4. 实现 branches.json 和 branches.md 自动维护

## 非目标

- 不实现阶段管理功能
- 不实现任务内容生成（由 Skills 负责）

## 设计

### aide verify

审验 task-now/ 目录的规范性：
- 检查必需文件存在
- 检查文件内容格式
- 检查 plantuml 文件编译
- 输出审验结果

### aide confirm

敲定任务：
- 重置 task-now.md
- 分配任务编号
- 重命名 task-now/ → task-{n}/
- 创建任务分支
- 更新 branches.json/md

### aide archive [n]

归档任务：
- 移动 tasks/task-{n}/ → archived-tasks/
- 更新 branches.json/md

## 实现计划

1. 实现任务文档规范验证
2. 实现任务编号分配
3. 实现 branches.json 读写
4. 实现 branches.md 生成
5. 实现三个子命令

## 影响范围

- 新增：aide verify/confirm/archive 命令
- 依赖：提案 1、提案 2
