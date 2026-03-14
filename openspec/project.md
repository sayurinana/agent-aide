# Project Context

## Purpose

Aide 是一套面向 Claude Code 的工作流辅助体系，通过 **Plugin + Program** 的架构，解决 AI 辅助开发中的信息过载、操作不确定性和流程耦合问题。

核心目标：
- 将 CLAUDE.md 中堆积的规则和流程转化为按需触发的模块化单元
- 提供结构化的任务执行流程（任务分析 → 流程设计 → 实现 → 验证 → 文档 → 收尾）
- 通过专用程序封装环境检测、进度追踪、待定项处理等确定性操作

## Project Structure

```
agent-aide/
├── aide-plugin/          # Claude Code 插件（Commands/Skills）
├── aide/                 # Rust 实现的 aide 命令行工具
│   ├── src/             # Rust 源码
│   ├── openspec/        # aide 程序的详细文档
│   ├── docs/            # 用户文档
│   └── Cargo.toml       # Rust 项目配置
├── openspec/            # 项目总览文档
└── docs/                # 整体项目文档
```

## Components

### 1. aide-plugin（Claude Code 插件）

提供 Commands 和 Skills，作为 AI 与工作流系统的交互界面。

**Commands** - 定义"做什么"和"按什么顺序做"的流程指导
**Skills** - 定义"怎么调用工具"的纯工具说明

### 2. aide（Rust 命令行工具）

核心运行时引擎，提供项目初始化、配置管理、流程追踪和待定项确认等功能。

详细文档见：`aide/openspec/project.md`

核心命令：
- `aide init` - 初始化 .aide 目录和默认配置
- `aide config` - 配置管理
- `aide flow` - 工作流追踪
- `aide decide` - 待定项确认

## Tech Stack

- **Rust** - aide 程序核心语言
- **Markdown** - 文档和配置格式
- **TOML** - 配置文件格式
- **JSON** - 数据存储格式
- **Git** - 版本控制和任务分支管理
- **PlantUML** - 流程图校验和渲染

## Project Conventions

### Code Style
- 所有文档、注释、对话必须使用简体中文
- Rust 代码遵循标准规范（见 `aide/openspec/project.md`）
- 配置文件必须包含详细的中文注释说明
- Markdown 文档使用清晰的层级结构和表格

### Architecture Patterns
- **三层架构**：aide-plugin (Commands/Skills) → aide (CLI) → .aide/ (数据存储)
- **渐进式披露**：信息按需加载，通过 Command 主动触发
- **确定性封装**：可变过程转化为固定接口，只暴露程序和参数
- **核心与形式分离**：LLM 负责分析决策，程序负责格式化和状态管理

### Git Workflow
- 任务执行时自动创建 `aide/NNN` 分支（三位递增编号）
- 使用 `aide flow` 命令管理进度和自动提交
- 任务完成时自动 squash 合并到源分支
- 提交信息格式：`[aide] <环节>: <摘要>`

## Domain Context

### 工作流阶段

1. **task-optimize** - 任务分析、复杂度评估、待定项处理、生成任务细则
2. **flow-design** - 创建 PlantUML 流程图（任务执行流程图 + 程序逻辑流图）
3. **impl** - 迭代实现，支持多轮开发
4. **verify** - 验证交付成果
5. **docs** - 更新项目文档
6. **finish** - 自动清理、合并分支、归档状态

### 核心概念

- **Commands** - 定义"做什么"和"按什么顺序做"的流程指导
- **Skills** - 定义"怎么调用工具"的纯工具说明
- **待定项（Pending Items）** - 需要用户决策的选项，通过 `aide decide` Web 界面确认
- **环节（Phase）** - 工作流中的大阶段
- **步骤（Step）** - 环节内的小进度单位

## Important Constraints

### 流程约束
- 复杂或多模块任务必须先调用 Sequential-Thinking 输出计划
- 任务细则生成后必须用户确认才能进入实现阶段
- 流程图必须通过 PlantUML 语法校验才能进入 impl 阶段
- 返工前必须更新任务文档，记录问题和调整方向

### 数据约束
- 配置文件：`.aide/config.toml`（自文档化，包含详细注释）
- 状态文件：`.aide/flow-status.json`（当前任务状态）
- 待定项：`.aide/decisions/*.json`（用户决策记录）

## External Dependencies

- **Git** - 版本控制和分支管理
- **PlantUML** - 流程图生成工具（通过 aide 自动管理）
- **Claude Code** - 宿主 IDE 环境，提供插件系统支持
