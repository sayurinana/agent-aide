# 提案：aide 阶段管理（flow）

## 概述

实现阶段级别的流程追踪和管理，支持场景预设和动态阶段选定。

## 动机

新体系需要灵活的阶段管理：
- 从 todo.md 提取阶段流程定义
- 追踪当前所处阶段
- 支持场景预设（full/standard/lite/docs/research）
- 支持阶段间流转和返工

## 目标

1. 实现阶段流程提取和解析
2. 实现当前阶段追踪
3. 实现场景预设定义
4. 实现 `aide flow` 相关子命令

## 非目标

- 不追踪 step 级别（仅阶段级别）
- 不实现具体阶段的执行逻辑（由 Skills 负责）

## 设计

### 阶段定义

**固定阶段**：build-task, impl-verify, confirm, finish
**可选阶段**：make-graphics, integration, review, docs-update

### 场景预设

- full: 全阶段
- standard: 标准流程
- lite: 轻量流程
- docs: 文档流程
- research: 调研流程

### aide flow 子命令

- `aide flow status` - 显示当前阶段
- `aide flow next` - 进入下一阶段
- `aide flow back` - 返工到指定阶段

## 实现计划

1. 定义阶段数据结构
2. 实现 todo.md 阶段流程解析
3. 实现阶段状态持久化
4. 实现 flow 子命令

## 影响范围

- 新增：aide flow 命令
- 依赖：提案 1、2、3
