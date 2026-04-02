# 提案：技术参考 Skills 更新

## 概述

更新 2 个技术参考 skills：plantuml 和 aide，使其适配新体系。

## 动机

这 2 个 skills 不对应具体阶段，而是作为技术参考供其他 skills 引用。

## 目标

1. 保留并调整 plantuml skill
2. 更新 aide skill 以反映新命令体系

## 设计

### plantuml skill

保留现有内容，供 make-graphics 等 skills 引用。

### aide skill

更新以反映新的命令体系：
- hi/go/bye 核心命令
- verify/confirm/archive 任务管理
- flow 阶段管理

## 影响范围

- 更新：2 个 skills
- 依赖：提案 1-8
