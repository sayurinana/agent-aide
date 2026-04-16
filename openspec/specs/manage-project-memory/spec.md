# manage-project-memory Specification

## Purpose
定义 aide-memory 目录、memory 文档集、模板与核心文档来源，以及 Agent 访问配置与分支数据时的边界。

## Requirements

### Requirement: `aide-memory` 目录结构

系统 SHALL 使用 `aide-memory/` 作为项目级 workflow 数据与长期记忆的统一根目录，并保持关键子目录与核心文件结构稳定。

#### Scenario: 初始化项目时创建标准目录
- **WHEN** 用户执行项目初始化
- **THEN** 系统创建 `aide-memory/`
- **AND** 创建 `memory/`、`tasks/`、`archived-tasks/`、`templates/` 等子目录
- **AND** 创建 `config.toml`、`config.md`、`branches.json`、`branches.md`、`AGENT.md`、`aide-process-overview.md`

### Requirement: memory 文档集完整性

系统 SHALL 为 `make-memory` / `load-memory` 工作流维护一套可消费的 memory 文档集，而不是只保留占位文件。

#### Scenario: 生成完整 memory 文档集
- **WHEN** 项目执行 memory 生成流程
- **THEN** 生成 `overview.md`、`structure/index.md`、`structure/*.md`、`concepts/term.md`、`concepts/arch.md` 与 `diagram/*`
- **AND** 这些内容足以支持后续按需载入项目认知

#### Scenario: memory 缺失时显式提示
- **WHEN** Agent 或用户尝试载入 memory
- **AND** memory 文档集不完整
- **THEN** 系统明确指出缺失项
- **AND** 指向补齐 memory 的正确入口

### Requirement: 配置与分支数据访问边界

系统 SHALL 明确区分配置、分支数据与面向 Agent 的可读文档，避免把内部数据文件直接暴露为默认工作入口。

#### Scenario: 通过文档或命令理解配置
- **WHEN** Agent 需要理解项目配置
- **THEN** 系统提供面向理解的配置文档或程序化查询入口
- **AND** 不要求 Agent 直接面向内部数据文件工作

#### Scenario: 分支状态通过正式入口查询
- **WHEN** Agent 需要理解任务分支映射和当前任务状态
- **THEN** 系统通过正式的 aide 命令或面向理解的产物提供这些信息
- **AND** 保持内部数据文件与用户可读报告之间的边界清晰

### Requirement: 初始化模板与核心文档来源

系统 SHALL 维护任务模板、解析指导文档、`AGENT.md` 与 `aide-process-overview.md` 的来源与同步规则，使初始化结果与 current truth 保持一致。

#### Scenario: 初始化使用 current truth 模板
- **WHEN** 新项目执行初始化
- **THEN** 系统写入的模板和核心文档与当前规范一致
- **AND** 不回退到已过时的旧体系模板或说明内容

#### Scenario: 任务解析指导文档与 build-task 收口一致
- **WHEN** 系统为项目提供任务解析指导文档
- **THEN** 该文档与 `build-task` 工作流使用的来源和命名保持一致
- **AND** 不再长期依赖旧 `task-parser` 命名残留

### Requirement: memory 与任务文档的同步责任

系统 SHALL 明确 memory 文档集、任务文档与阶段性技能之间的同步责任边界。

#### Scenario: 任务结束后同步长期保留信息
- **WHEN** 任务完成正式收尾
- **THEN** 系统或 Agent 同步需要长期保留的 memory 信息
- **AND** 不把临时任务草案内容直接等同于长期记忆
