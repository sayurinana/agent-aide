# 提案：实现验证

## 概述

检验项目当前实现情况是否与 task-optimized.md 中的要求一致。

## 动机

确保所有已完成的提案实现都符合原始需求文档的规范，发现并修复任何偏差。

## 目标

1. 检查 Commands 实现与 task-optimized.md 要求的一致性
2. 检查 Skills 实现与 task-optimized.md 要求的一致性
3. 检查 aide 程序命令实现的一致性
4. 检查目录结构和配置的一致性
5. 记录并修复发现的任何问题

## 检查范围

### Commands（5个）

- make-memory
- load-memory
- hi
- go
- bye

### Skills（13个）

**基础 Skills（2个）**
- make-memory
- load-memory

**子过程 Skills（9个）**
- build-task
- make-graphics
- impl-verify
- integration
- review
- docs-update
- confirm
- finish
- rework

**技术参考 Skills（2个）**
- plantuml
- aide

### aide 程序命令

- hi / go / bye
- verify / confirm / archive
- flow

### 目录结构

- aide-memory/ 目录结构是否符合规范

## 影响范围

- 检验：所有已实现的文件
- 修复：发现的偏差问题