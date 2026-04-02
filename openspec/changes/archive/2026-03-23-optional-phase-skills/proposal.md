# 提案：可选子过程 Skills

## 概述

实现 5 个可选子过程 skills：make-graphics、integration、review、docs-update、rework。

## 动机

这 5 个 skills 对应可选阶段，根据任务特性在 build-task 阶段选定。

## 目标

1. 实现 make-graphics skill
2. 实现 integration skill
3. 实现 review skill
4. 实现 docs-update skill
5. 调整 rework skill

## 设计

### make-graphics skill
为任务编写 PlantUML 图解

### integration skill
整体集成测试方法论

### review skill
代码审查/文档审校/方案评审

### docs-update skill
更新项目文档和 memory

### rework skill
返工流程（调整现有）

## 影响范围

- 新增：5 个 skills
- 依赖：提案 1-7
