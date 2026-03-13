# Project Context

## Purpose

Aide 是一个命令行工作流辅助工具，作为 Aide 插件系统的核心运行时引擎。提供项目初始化、配置管理、流程追踪和待定项确认等功能，帮助开发者在 AI 辅助编程场景下管理工作流。

当前状态：正在从 Python 参考实现迁移到 Rust 实现。Python 源码保留在 `aide/` 目录作为参考，Rust 新实现位于 `src/`。

### 核心功能

| 命令 | 说明 |
|------|------|
| `aide init` | 初始化 .aide 目录和默认配置 |
| `aide config get/set` | 读写 TOML 配置（点分隔键值） |
| `aide flow start/next-step/back-step/next-part/back-part/issue/error/status/list/show` | 工作流追踪（任务、步骤、环节管理） |
| `aide decide submit/result` | 待定项确认（提交决策项、获取用户决策结果） |

> 注意：`aide env` 及其所有子命令不在 Rust 迁移范围内。

## Tech Stack

- **目标语言**：Rust（edition 2024）
- **参考实现**：Python 3.11+（位于 `aide/` 目录）
- **构建工具**：Cargo
- **配置格式**：TOML（项目配置）、JSON（状态和决策数据）
- **外部工具**：
  - Git（流程追踪的自动提交）
  - PlantUML（流程图生成，通过 `aide init --global` 自动安装自包含可执行程序）
- **包管理**：Cargo（Rust 依赖管理）

## Project Conventions

### Code Style

- **语言**：所有文档、注释和用户交互文本使用简体中文
- **命名规范**：
  - 结构体/枚举：PascalCase（如 `FlowTracker`、`ConfigManager`）
  - 函数/方法：snake_case（如 `next_step`、`load_config`）
  - 常量：UPPER_SNAKE_CASE（如 `DEFAULT_PHASES`）
  - 文件名：snake_case（如 `flow_tracker.rs`）
- **输出符号**：统一使用 ✓（成功）、⚠（警告）、✗（错误）、→（信息）、[n/m]（步骤计数）
- **静默成功原则**：操作成功时默认不产生输出，仅在需要用户关注时才显示信息

### Architecture Patterns

- **模块化架构**：按功能域划分模块（core、flow、decide）
- **单一职责**：每个模块只负责一个功能域
- **关注点分离**：追踪器（tracker）、存储（storage）、校验（validator）、Git 集成各自独立
- **数据驱动**：TOML 配置 + JSON 状态存储
- **原子化状态管理**：使用文件锁保证并发安全的状态读写

#### 核心组件职责

| 组件 | 职责 |
|------|------|
| ConfigManager | .aide 目录维护、TOML 配置读写、.gitignore 管理 |
| FlowTracker | 编排校验→钩子→存储→Git 提交流程 |
| FlowStorage | 原子化读写 flow-status.json、状态归档 |
| FlowValidator | 环节跳转规则校验 |
| BranchManager | 任务分支创建、记录、合并 |
| DecideServer | HTTP 服务器生命周期管理、端口探测 |
| DecideStorage | pending/result JSON 文件管理 |

#### .aide 目录结构

```
.aide/
├── config.toml              # 项目配置
├── flow-status.json         # 当前任务状态
├── archive/                 # 已完成任务归档
│   └── {task_id}.json
├── decisions/               # 决策记录
│   ├── pending.json
│   └── {timestamp}.json
├── diagrams/                # PlantUML 图表
├── branches/                # 分支追踪
│   ├── branches.json
│   └── branches.md
└── logs/                    # 操作日志
```

#### 默认工作流环节

`task-optimize` → `flow-design` → `impl` → `verify` → `docs` → `finish`

### Testing Strategy

- 使用 Rust 标准测试框架（`#[cfg(test)]` 模块 + `cargo test`）
- 单元测试与源码同文件
- 集成测试放在 `tests/` 目录
- 重点覆盖：状态转换逻辑、配置读写、Git 集成、命令行解析

### Git Workflow

- **主分支**：`main`
- **任务分支**：`aide/NNN`（由 flow 命令自动创建）
- **自动提交格式**：`[aide] <环节>: <摘要>`
- **提交触发**：flow 命令执行时自动 `git add .` 并提交
- **分支追踪**：通过 `.aide/branches/branches.json` 记录分支生命周期

## Domain Context

### 关键概念

- **环节（Phase）**：工作流中的大阶段，如 task-optimize、impl、verify 等
- **步骤（Step）**：环节内的小进度单位，通过 next-step/back-step 管理
- **待定项（Decide Item）**：需要用户决策的选项，通过 Web UI 呈现给用户
- **任务（Task）**：一个完整的工作流实例，包含 task_id、当前环节、步骤和历史记录

### 数据结构

**FlowStatus**（flow-status.json）：
- `task_id`：时间戳格式的任务标识
- `current_phase`：当前环节名称
- `current_step`：当前步骤编号
- `started_at`：ISO 时间戳
- `history`：历史记录列表
- `source_branch`、`start_commit`、`task_branch`：分支管理信息

**DecideInput**（提交格式）：
- `task`：任务描述
- `source`：来源标识
- `items`：决策项列表（含 id、title、location、context、options、recommend）

**DecideOutput**（结果格式）：
- `decisions`：决策结果列表（含 id、chosen、note）

## Important Constraints

1. **Rust 迁移范围**：不实现 `aide env` 及其所有子命令，仅迁移 init、config、flow、decide
2. **Git 依赖**：flow 命令假设在 Git 仓库中运行，自动执行 git 操作
3. **PlantUML 依赖**：流程图生成使用自包含可执行程序（内嵌 JRE），通过 `aide init --global` 自动安装到 `~/.aide/utils/plantuml/`
4. **端口配置**：decide 服务默认端口 3721，可通过配置修改
5. **文件锁**：状态文件操作需要文件锁保证并发安全
6. **配置保留**：更新 TOML 配置时需保留用户注释
7. **文档要求**：完成 Rust 实现后需编写完整配套文档，保存到 `docs/` 目录

## External Dependencies

- **Git**：版本控制系统，flow 命令通过子进程调用 git 命令
- **Java**（不再需要）：PlantUML 使用自包含可执行程序，内嵌 JRE
- **PlantUML**（`~/.aide/utils/plantuml/bin/plantuml`）：UML 图表生成工具，通过 aide 自动管理
