# aide-plugin-commands

> 路径：aide-marketplace/aide-plugin/commands/
> 最后更新：2025-12-16

## 概述

Aide 插件的斜杠命令定义，是面向 LLM 的执行指令文件。

## 文件清单

| 文件 | 说明 |
|------|------|
| `setup.md` | `/aide:setup` - 环境配置命令 |
| `load.md` | `/aide:load` - 项目认知载入 |
| `docs.md` | `/aide:docs` - 项目文档管理 |
| `run.md` | `/aide:run` - 任务执行（核心命令） |

## 命令说明

### /aide:setup

- **用途**：环境配置（独立运行）
- **触发 skill**：env-config
- **流程**：
  1. 检查 aide 运行时环境
  2. 初始化 .aide 目录
  3. 分析项目依赖
  4. 配置环境模块
  5. 执行环境检测

### /aide:load

- **用途**：项目认知载入（由 run 调用）
- **触发 skill**：aide
- **流程**：
  1. 检查项目文档配置
  2. 载入总导览
  3. 建立脉络认知
  4. 按需深入

### /aide:docs

- **用途**：项目文档创建和维护（独立运行）
- **触发 skill**：aide
- **流程**：
  - 创建流程：目录探索 → 区块划分 → 逐区块生成文档 → 生成总导览
  - 更新流程：读取区块计划 → 分区块验证 → 增量更新

### /aide:run

- **用途**：任务执行（核心命令）
- **触发 skill**：aide, task-parser（按需）
- **新增功能**：
  - 口语化内容检测：在任务分析前检测内容特征
  - 自动触发 task-parser skill 解析口语化内容
  - 流程图规范：区分任务执行流程图和程序逻辑流图
- **标准流程**：
  1. task-optimize - 任务准备（含口语化检测）
  2. flow-design - 流程设计（含流程图规范）
  3. impl - 迭代实现
  4. verify - 验证交付
  5. docs - 文档更新
  6. finish - 收尾

## 流程图要求

### 任务执行流程图（所有任务必需）
- 展示任务执行的步骤顺序
- 体现决策点和依赖关系

### 程序逻辑流图（程序类任务必需）
- 从入口函数开始
- 体现顺序/分支/循环结构
- 语义化抽象，模块化表示
- 支持层次化组织

## 设计原则

- 执行文件给 LLM 看，聚焦执行指令
- 设计文档给人看，包含完整上下文
- 每个命令开始时触发对应的 skill 学习必要知识

## 依赖关系

- 依赖：aide skill, env-config skill, task-parser skill
- 调用：aide 命令行工具

## 注意事项

- `/aide:setup` 和 `/aide:docs` 是独立运行的命令
- `/aide:load` 通常由 `/aide:run` 内部调用
- `/aide:run` 是最常用的核心命令
