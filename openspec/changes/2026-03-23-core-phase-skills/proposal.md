# 提案：核心子过程 Skills

## 概述

实现 4 个核心子过程 skills：build-task、impl-verify、confirm、finish。

## 动机

这 4 个 skills 对应固定阶段，每个任务都必须经过。

## 目标

1. 实现 build-task skill（重命名自 task-parser）
2. 实现 impl-verify skill
3. 实现 confirm skill
4. 实现 finish skill

## 设计

### build-task skill

- 指导如何构建符合 aide 体系规范的任务
- 解析风格由用户指定的解析指导文档决定
- 输出 information.md、design.md、todo.md、task-summary.md

### impl-verify skill

- 按 todo.md 中的任务点逐一实施
- 每完成一个任务点立即进行审验
- 支持循环模式

### confirm skill

- 向用户展示成果和变更摘要
- 收集用户反馈

### finish skill

- 使用 aide 程序完成任务归档
- 合并任务分支回常驻分支

## 影响范围

- 新增：4 个 skills
- 依赖：提案 1-6
