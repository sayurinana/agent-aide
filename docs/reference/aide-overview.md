# Aide 系统概述

## 一、系统简介

Aide 是一套面向 Claude Code 的工作流辅助体系，旨在解决 AI 辅助开发中的信息过载、操作不确定性和流程耦合问题。

系统通过 **Command + Skill + 专用程序** 的架构，将原本堆积在 CLAUDE.md 中的规则和流程转化为按需触发的模块化单元，实现：

- **CLAUDE.md 精简化**：仅保留项目结构说明，不再堆积规则和流程
- **流程按需触发**：通过 Command 主动触发流程指导
- **操作确定性封装**：通过 Skill + 程序简化操作，减少不确定性

---

## 二、核心设计原则

| 原则 | 说明 |
|------|------|
| **渐进式披露** | 信息按需加载，用户/LLM 主动调用时才加载相关指引 |
| **确定性封装** | 将可变过程转化为固定接口，只暴露程序和参数 |
| **信息隔离** | LLM 只传核心语义数据，程序负责格式化和渲染 |
| **核心与形式分离** | 核心信息（分析、决策）由 LLM 发挥，形式问题（状态记录、环境配置）由程序处理 |

---

## 三、系统架构

```
┌─────────────────────────────────────────────────────────────┐
│                        用户                                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    aide-plugin                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │
│  │ /aide:setup │  │ /aide:load  │  │ /aide:docs  │  │ /aide:run   │  │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘  │
│                                                              │
│  ┌─────────────────────────────────────────────┐            │
│  │              aide skill                      │   Skill   │
│  │  (aide 命令使用指南)                         │            │
│  └─────────────────────────────────────────────┘            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ 调用
┌─────────────────────────────────────────────────────────────┐
│                    aide-program                              │
│  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐            │
│  │  init  │  │  env   │  │  flow  │  │ decide │  子命令    │
│  └────────┘  └────────┘  └────────┘  └────────┘            │
│                                                              │
│  ┌─────────────────────────────────────────────┐            │
│  │           .aide/ 数据目录                    │            │
│  │  config.toml | flow-status.json | decisions/ │            │
│  └─────────────────────────────────────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

**组件关系**：
- **Commands**：定义流程（做什么、按什么顺序做），指导 LLM 的思考方向
- **Skill**：定义工具使用方法（怎么调用 aide 程序），纯工具说明
- **Program**：执行具体操作（环境检测、进度追踪、待定项处理），返回精简结果

---

## 四、子区块索引

### 4.1 aide-plugin 区块

| 文档 | 位置 | 说明 |
|------|------|------|
| **导览** | [aide-plugin/docs/README.md](../aide-marketplace/aide-plugin/docs/README.md) | plugin 整体介绍和索引 |
| setup 命令 | [aide-plugin/docs/commands/setup.md](../aide-marketplace/aide-plugin/docs/commands/setup.md) | 环境配置（独立运行） |
| load 命令 | [aide-plugin/docs/commands/load.md](../aide-marketplace/aide-plugin/docs/commands/load.md) | 项目认知载入 |
| docs 命令 | [aide-plugin/docs/commands/docs.md](../aide-marketplace/aide-plugin/docs/commands/docs.md) | 项目文档管理（独立运行） |
| run 命令 | [aide-plugin/docs/commands/run.md](../aide-marketplace/aide-plugin/docs/commands/run.md) | 任务执行（核心命令） |
| aide skill | [aide-plugin/docs/skill/aide.md](../aide-marketplace/aide-plugin/docs/skill/aide.md) | aide 命令使用指南 |
| env-config skill | [aide-plugin/skills/env-config/SKILL.md](../aide-marketplace/aide-plugin/skills/env-config/SKILL.md) | 环境配置详细指南 |

### 4.2 aide-program 区块

| 文档 | 位置 | 说明 |
|------|------|------|
| **导览** | [aide-program/docs/README.md](../aide-program/docs/README.md) | program 整体介绍和索引 |
| env 子命令 | [aide-program/docs/commands/env.md](../aide-program/docs/commands/env.md) | 环境检测与修复 |
| flow 子命令 | [aide-program/docs/commands/flow.md](../aide-program/docs/commands/flow.md) | 进度追踪与 git 集成 |
| flow 详细设计 | [aide-program/docs/commands/flow/README.md](../aide-program/docs/commands/flow/README.md) | flow 实现细节与验证清单 |
| decide 子命令 | [aide-program/docs/commands/decide.md](../aide-program/docs/commands/decide.md) | 待定项 Web 确认 |
| init 子命令 | [aide-program/docs/commands/init.md](../aide-program/docs/commands/init.md) | 初始化 .aide 目录 |
| 配置格式 | [aide-program/docs/formats/config.md](../aide-program/docs/formats/config.md) | config.toml 规范 |
| 数据格式 | [aide-program/docs/formats/data.md](../aide-program/docs/formats/data.md) | 待定项、流程状态等格式 |

---

## 五、快速导航

### 想了解/修改 Commands 或 Skill

→ 阅读 [aide-plugin 导览](../aide-marketplace/aide-plugin/docs/README.md)

### 想了解/修改 aide 程序

→ 阅读 [aide-program 导览](../aide-program/docs/README.md)

### 想了解完整工作流程

```
/aide:setup     /aide:docs      /aide:run
(独立运行)      (独立运行)      (核心命令)
    │               │               │
    ▼               ▼               ▼
环境配置        项目文档        任务执行
依赖分析        区块划分        ├─ task-optimize (任务准备)
模块检测        文档生成        │   ├─ 任务分析
问题修复        增量更新        │   ├─ 复杂度评估
                               │   ├─ 待定项处理
                               │   └─ 生成细则
                               ├─ flow-design (流程设计)
                               │   └─ 创建流程图
                               ├─ impl (迭代实现)
                               ├─ verify (验证交付)
                               ├─ docs (文档更新)
                               └─ finish (收尾)
```

> `/aide:load` 由 `/aide:run` 自动调用，用于按需载入项目文档。

---

## 六、版本信息

- 当前版本：2.0.0
- 更新日期：2025-12-15
- 主要变更：Commands 体系重组（setup/load/docs/run 替代 init/prep/exec）
