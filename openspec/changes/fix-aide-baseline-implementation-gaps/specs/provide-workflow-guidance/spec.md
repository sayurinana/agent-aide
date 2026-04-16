## MODIFIED Requirements

### Requirement: 核心阶段技能契约

系统 SHALL 为 `build-task`、`impl-verify`、`confirm` 与 `finish` 提供清晰且互不混淆的技能契约。

#### Scenario: `build-task` 优先消费程序输出的解析指导路径
- **WHEN** Agent 进入 `build-task` 阶段
- **THEN** 优先使用 aide 程序输出的任务解析指导文档绝对路径
- **AND** 仅在程序未提供明确路径时才回退到默认模板来源

#### Scenario: `confirm` 与 `aide confirm` 命令语义分离
- **WHEN** Agent 进入 `confirm` 阶段
- **THEN** 该阶段表示向用户展示成果并收集反馈
- **AND** 不与 `aide confirm` 的任务敲定命令混淆

#### Scenario: `finish` 执行正式收尾而不是简单离场
- **WHEN** Agent 进入 `finish` 阶段
- **THEN** 该阶段负责正式收尾、归档、分支收束与长期信息同步
- **AND** 不被降格为简单离场说明

### Requirement: 可选阶段技能契约

系统 SHALL 为 `make-graphics`、`integration`、`review`、`docs-update` 与 `rework` 提供与阶段体系一致的技能契约。

#### Scenario: `make-graphics` 与 HTML 注释图解协议一致
- **WHEN** Agent 进入 `make-graphics` 阶段
- **THEN** 该 skill 使用 `<!-- GRAPHICS: required -->` 与 `<!-- GRAPHICS: skip: 原因 -->` 等当前图解标记协议
- **AND** 不再与旧图解语义并存

#### Scenario: `rework` 使用当前 flow back 命令并记录 reason
- **WHEN** Agent 进入返工流程
- **THEN** 该 skill 使用当前 `aide flow back <phase> [reason]` 语义
- **AND** 记录返工原因与影响范围

### Requirement: 命名与术语收口

系统 SHALL 收口新旧体系命名，避免旧术语长期与 current truth 并存。

#### Scenario: 活跃 command、skill 与文档不再暴露旧体系主路径
- **WHEN** 系统描述当前 workflow 命令、阶段或技能入口
- **THEN** 使用新的阶段级 flow 术语和命令集合
- **AND** 不把 `aide decide`、`aide env`、旧 step 级 flow、旧 `.aide/` 路径继续描述为现行主路径

#### Scenario: `task-parser` 仅保留为历史迁移线索
- **WHEN** 系统提供任务构建相关入口、模板或指导文档
- **THEN** 统一使用 `build-task` 作为正式命名
- **AND** 不再把 `task-parser` 作为 current truth 的活跃入口
