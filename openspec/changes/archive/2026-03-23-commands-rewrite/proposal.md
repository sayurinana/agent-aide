# 提案：Commands 重写

## 概述

重写 5 个 commands：make-memory、load-memory、hi、go、bye，精简为入口指引。

## 动机

新体系中 commands 定位为精简的入口指引，指导 Agent 学习相应 skills 并执行操作。

## 目标

1. 重写 5 个 commands
2. 编写 aide-process-overview.md
3. 编写 AGENT.md

## 设计

### Commands 内容结构

- 简要说明用途
- 指出需要学习的 skills
- 指导执行的 aide 子命令
- 说明预期行为

### aide-process-overview.md

概述 aide 体系的情境、阶段、预设，提供全局流程视图。

### AGENT.md

定义 Agent 身份为总工程师，明确职责分工。

## 实现计划

1. 编写 5 个 commands
2. 编写 aide-process-overview.md
3. 编写 AGENT.md

## 影响范围

- 新增/重写：5 个 commands
- 新增：aide-process-overview.md、AGENT.md
- 依赖：提案 1-4
