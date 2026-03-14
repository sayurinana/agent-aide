# Project Context

## Purpose
Aide 是一套面向 Claude Code 的工作流辅助体系，通过 **Command + Skill + 专用程序** 的架构，解决 AI 辅助开发中的信息过载、操作不确定性和流程耦合问题。

核心目标：
- 将 CLAUDE.md 中堆积的规则和流程转化为按需触发的模块化单元
- 提供结构化的任务执行流程（任务分析 → 流程设计 → 实现 → 验证 → 文档 → 收尾）
- 通过专用程序封装环境检测、进度追踪、待定项处理等确定性操作

## Tech Stack
- **Python 3.x** - 核心程序语言（aide-program）
- **uv** - Python 包管理和虚拟环境
- **PlantUML + Java JRE** - 流程图校验和渲染
- **Git** - 版本控制和任务分支管理
- **Markdown** - 文档和配置格式
- **TOML** - 配置文件格式（config.toml）
- **JSON** - 数据存储格式（flow-status.json, decisions/*.json）

## Project Conventions

### Code Style
- 所有文档、注释、对话必须使用简体中文
- Python 代码遵循 PEP 8 规范
- 配置文件必须包含详细的中文注释说明
- Markdown 文档使用清晰的层级结构和表格

### Architecture Patterns
- **三层架构**：aide-plugin (Commands/Skills) → aide-program (CLI) → .aide/ (数据存储)
- **渐进式披露**：信息按需加载，通过 Command 主动触发
- **确定性封装**：可变过程转化为固定接口，只暴露程序和参数
- **核心与形式分离**：LLM 负责分析决策，程序负责格式化和状态管理

### Testing Strategy
- Python 项目必须使用 uv 管理的虚拟环境（.venv/）
- 禁止直接使用全局 `python` 或 `python3` 命令
- 必须维护 requirements.txt 和项目 README.md

### Git Workflow
- 任务执行时自动创建 `aide/NNN` 分支（三位递增编号）
- 使用 `aide flow` 命令管理进度和自动提交
- 任务完成时自动 squash 合并到源分支
- 提交信息格式：`完成：<分支名> - <任务名>` 或 `任务中断，清理：<分支名> - <任务名>`

## Domain Context

**工作流阶段**：
1. **task-optimize** - 任务分析、复杂度评估、待定项处理、生成任务细则
2. **flow-design** - 创建 PlantUML 流程图（任务执行流程图 + 程序逻辑流图）
3. **impl** - 迭代实现，支持多轮开发
4. **verify** - 验证交付成果
5. **docs** - 更新项目文档
6. **confirm** - 用户审阅和测试阶段
7. **finish** - 自动清理、合并分支、归档状态

**核心概念**：
- **Commands** - 定义"做什么"和"按什么顺序做"的流程指导
- **Skills** - 定义"怎么调用工具"的纯工具说明
- **待定项（Pending Items）** - 需要用户决策的选项，通过 `aide decide` Web 界面确认
- **任务细则（Task Spec）** - 存储在 `.aide/task-plans/` 的详细执行计划
- **项目文档** - 面向 LLM 的区块化文档，支持总导览 + 多子区块结构

## Important Constraints

**技术约束**：
- Python 脚本必须在 uv 管理的虚拟环境中运行
- PlantUML 需要 Java JRE 支持
- 涉及两个以上文件的移动/复制/重命名操作必须通过 ./cache/ 下的临时 .sh 脚本执行

**流程约束**：
- 复杂或多模块任务必须先调用 Sequential-Thinking 输出计划
- 任务细则生成后必须用户确认才能进入实现阶段
- 流程图必须通过 PlantUML 语法校验才能进入 impl 阶段
- 返工前必须更新任务文档，记录问题和调整方向

**数据约束**：
- 配置文件：`.aide/config.toml`（自文档化，包含详细注释）
- 状态文件：`.aide/flow-status.json`（当前任务状态）
- 待定项：`.aide/decisions/*.json`（用户决策记录）
- 任务计划：`.aide/task-plans/`（复杂任务的子计划文档）

## External Dependencies

**必需依赖**：
- **uv** - Python 包管理器，用于创建和管理虚拟环境
- **Java JRE** - PlantUML 运行时环境
- **Git** - 版本控制和分支管理

**可选依赖**：
- **PlantUML jar** - 内置在项目中，用于流程图校验和渲染
- **Claude Code** - 宿主 IDE 环境，提供插件系统支持
