## ADDED Requirements

### Requirement: 总工程师命令入口

系统 SHALL 提供面向总工程师 Agent 的 command 入口，用于根据当前状态选择正确的 workflow 路径，而不是让单一 Agent 直接硬做全部事务。

#### Scenario: `hi` 作为状态诊断入口
- **WHEN** Agent 需要理解当前项目状态
- **THEN** 使用 `hi` 入口先读取总览、理解当前状态并决定是否载入 memory 或接续任务

#### Scenario: `go` 作为任务接续入口
- **WHEN** Agent 需要进入既有任务继续执行
- **THEN** 使用 `go` 入口先理解状态，再进入对应任务与阶段

### Requirement: 基础认知技能

系统 SHALL 提供 `make-memory` 与 `load-memory` 两个基础技能，使 Agent 能建立和载入项目长期认知。

#### Scenario: memory 不完整时引导补齐
- **WHEN** Agent 需要依赖项目 memory 进行工作
- **AND** 当前 memory 文档集不完整
- **THEN** 系统引导执行 `make-memory`
- **AND** 在 memory 可用后使用 `load-memory` 按需载入

### Requirement: 阶段性技能加载规则

系统 SHALL 要求 Agent 在进入对应阶段后再学习和调用相应 skill，而不是在任务开始时一次性加载全部技能。

#### Scenario: 按阶段加载 skill
- **WHEN** Agent 确认当前所处阶段
- **THEN** 仅学习与该阶段相关的 skill
- **AND** 在阶段切换后再进入下一阶段 skill

### Requirement: 核心阶段技能契约

系统 SHALL 为 `build-task`、`impl-verify`、`confirm` 与 `finish` 提供清晰且互不混淆的技能契约。

#### Scenario: `build-task` 基于解析指导构建正式草案
- **WHEN** Agent 进入 `build-task` 阶段
- **THEN** 按任务解析指导文档打磨需求并产出正式草案文件
- **AND** 明确当前只是构建草案，不是进入实现

#### Scenario: `confirm` 与 `aide confirm` 命令语义分离
- **WHEN** Agent 进入 `confirm` 阶段
- **THEN** 该阶段表示向用户展示成果并收集反馈
- **AND** 不与 `aide confirm` 的任务敲定命令混淆

#### Scenario: `finish` 指向正式收尾归档
- **WHEN** Agent 进入 `finish` 阶段
- **THEN** 该阶段负责正式收尾、归档、分支收束与长期信息同步
- **AND** 不被降格为简单离场说明

### Requirement: 可选阶段技能契约

系统 SHALL 为 `make-graphics`、`integration`、`review`、`docs-update` 与 `rework` 提供与阶段体系一致的技能契约。

#### Scenario: `make-graphics` 与图解标记规则一致
- **WHEN** Agent 进入 `make-graphics` 阶段
- **THEN** 该 skill 使用与 `build-task`、`aide verify` 一致的图解标记协议
- **AND** 不再与旧语义并存

#### Scenario: `rework` 记录返工原因
- **WHEN** Agent 进入返工流程
- **THEN** 系统或流程要求记录返工原因与影响范围
- **AND** 保证后续阶段能够理解返工背景

### Requirement: 总工程师角色与专家执行模型

系统 SHALL 将 Agent 定义为总工程师，并用 commands、skills 与总览文档共同约束“总工程师 + 专家执行单元”的协作模式。

#### Scenario: 宿主支持子代理时按专家分工执行
- **WHEN** 宿主环境支持子代理或等效受限上下文
- **THEN** 总工程师按阶段或主题拆分任务给专家执行单元
- **AND** 自身负责统筹、审查与汇总

#### Scenario: 宿主不支持子代理时保留角色边界
- **WHEN** 宿主环境不支持真正的子代理
- **THEN** Agent 仍按总工程师视角工作
- **AND** 在单一上下文中显式保留各阶段的职责边界

### Requirement: 命名与术语收口

系统 SHALL 收口新旧体系命名，避免旧术语长期与 current truth 并存。

#### Scenario: `task-parser` 收口为 `build-task`
- **WHEN** 系统提供任务构建相关入口、模板或指导文档
- **THEN** 统一使用 `build-task` 作为正式命名
- **AND** 不再把 `task-parser` 作为 current truth 的活跃入口

#### Scenario: 旧 flow 与旧命令术语退出 current truth
- **WHEN** 系统描述当前 workflow 命令与阶段
- **THEN** 使用新的阶段级 flow 术语和命令集合
- **AND** 不把旧 step 级 flow、`aide decide`、`aide env` 等旧能力继续描述为现行主路径
