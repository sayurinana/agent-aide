# 用户流程图编写计划

> 最后更新：2025-12-19 06:50

## 项目概述

**agent-aide** 是 Aide 工作流工具项目，包含核心命令行工具（Python）和 Claude Code 插件。本计划用于生成面向用户的流程图文档，展示项目的整体架构和核心流程。

## 区块索引

| # | 区块名称 | 类型 | 步骤数 | 状态 | 步骤范围 |
|---|----------|------|--------|------|----------|
| 1 | aide-program | 库项目/应用 | 6 | completed | 001-006 |
| 2 | aide-marketplace | 应用 | 3 | completed | 007-009 |
| 3 | project-config-docs | 文档 | 1 | completed | 010 |

## 步骤索引

| 步骤 | 所属区块 | 流程图 | 状态 |
|------|----------|--------|------|
| [001](steps/step-001.md) | aide-program | guide.puml | completed |
| [002](steps/step-002.md) | aide-program | main.puml | completed |
| [003](steps/step-003.md) | aide-program | env.puml | completed |
| [004](steps/step-004.md) | aide-program | flow.puml | completed |
| [005](steps/step-005.md) | aide-program | decide.puml | completed |
| [006](steps/step-006.md) | aide-program | branch.puml | completed |
| [007](steps/step-007.md) | aide-marketplace | guide.puml | completed |
| [008](steps/step-008.md) | aide-marketplace | commands.puml | completed |
| [009](steps/step-009.md) | aide-marketplace | skills.puml | completed |
| [010](steps/step-010.md) | project-config-docs | guide.puml | completed |

## 整体进度

- 总步骤数：10
- 已完成：10
- 进行中：0
- 待处理：0

## 流程图清单

### aide-program（6 个）

| 文件 | 类型 | 说明 |
|------|------|------|
| guide.puml | 组件图 | 整体架构和模块关系 |
| main.puml | 活动图 | CLI 主入口流程 |
| env.puml | 活动图 | 环境检测流程 |
| flow.puml | 活动图 | 流程追踪核心流程 |
| decide.puml | 序列图 | 待定项确认流程 |
| branch.puml | 活动图 | 分支管理流程 |

### aide-marketplace（3 个）

| 文件 | 类型 | 说明 |
|------|------|------|
| guide.puml | 组件图 | 插件架构和组件关系 |
| commands.puml | 活动图 | 核心命令执行流程 |
| skills.puml | 思维导图 | 技能体系和触发机制 |

### project-config-docs（1 个）

| 文件 | 类型 | 说明 |
|------|------|------|
| guide.puml | 思维导图 | 配置体系导航图 |

## 执行日志

| 时间 | 步骤 | 操作 | 备注 |
|------|------|------|------|
| 2025-12-19 06:45 | - | 计划创建 | 生成 10 个步骤文档 |
| 2025-12-19 06:46 | 001 | 完成 | aide-program/guide.puml |
| 2025-12-19 06:46 | 002 | 完成 | aide-program/main.puml |
| 2025-12-19 06:47 | 003 | 完成 | aide-program/env.puml |
| 2025-12-19 06:47 | 004 | 完成 | aide-program/flow.puml |
| 2025-12-19 06:48 | 005 | 完成 | aide-program/decide.puml |
| 2025-12-19 06:48 | 006 | 完成 | aide-program/branch.puml |
| 2025-12-19 06:49 | 007 | 完成 | aide-marketplace/guide.puml |
| 2025-12-19 06:49 | 008 | 完成 | aide-marketplace/commands.puml |
| 2025-12-19 06:50 | 009 | 完成 | aide-marketplace/skills.puml |
| 2025-12-19 06:50 | 010 | 完成 | project-config-docs/guide.puml |
