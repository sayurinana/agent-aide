# 提案：基础 Skills

## 概述

实现 2 个基础 skills：make-memory 和 load-memory，从现有 commands 迁移而来。

## 动机

make-memory 和 load-memory 是项目认知的基础，需要从 commands 迁移为 skills。

## 目标

1. 迁移 make-memory skill（从 commands/docs.md）
2. 迁移 load-memory skill（从 commands/load.md）
3. 调整为新的目录结构和配置

## 设计

### make-memory skill

- 严格按项目目录结构递归扫描
- 提取内容概述 → memory/structure/
- 提取抽象概念 → memory/concepts/
- 编写概念图解 → memory/diagram/
- 编写导览 overview.md

### load-memory skill

- 按需载入 memory 文档
- 建立项目认知

## 实现计划

1. 迁移并调整 make-memory skill
2. 迁移并调整 load-memory skill

## 影响范围

- 新增：2 个 skills
- 依赖：提案 1-5
