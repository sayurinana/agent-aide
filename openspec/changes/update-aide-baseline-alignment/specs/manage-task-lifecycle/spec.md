## ADDED Requirements

### Requirement: 任务草案结构与校验

系统 SHALL 使用统一的任务草案结构承载正式任务的 build-task 产物，并通过 `aide verify` 校验草案是否可进入敲定阶段。

#### Scenario: 草案包含必需文件与目录
- **WHEN** Agent 构建正式任务草案
- **THEN** 草案目录包含 `information.md`、`design.md`、`todo.md`、`task-summary.md` 与 `flow-graphics/`
- **AND** 这些文件满足阶段声明、图解标记与基础格式要求

#### Scenario: `aide verify` 阻止不完整草案进入敲定
- **WHEN** 用户执行 `aide verify`
- **AND** 草案缺少关键文件、缺少图解标记或阶段声明不合法
- **THEN** 系统报告失败
- **AND** 不允许将该草案视为已可敲定

### Requirement: 任务草案敲定与编号分配

系统 SHALL 在草案通过审验后将其敲定为正式任务，并分配可追踪的任务编号。

#### Scenario: 草案敲定为正式任务
- **WHEN** 用户执行 `aide confirm`
- **AND** 草案已通过审验
- **THEN** 系统分配下一个任务编号
- **AND** 将 `task-now/` 重命名为 `task-{n}/`
- **AND** 将任务描述入口重置为模板初始内容

### Requirement: 任务分支建立与分支映射维护

系统 SHALL 为正式任务维护任务分支与任务编号之间的映射，并在任务生命周期中持续更新这些映射。

#### Scenario: 敲定任务时创建任务分支映射
- **WHEN** 任务被正式敲定
- **THEN** 系统根据分支配置生成任务分支名
- **AND** 创建该任务分支
- **AND** 更新 `branches.json` 与 `branches.md`

#### Scenario: 归档任务时更新映射状态
- **WHEN** 用户归档任务
- **THEN** 系统更新任务状态为已归档
- **AND** 保持分支映射与目录状态一致

### Requirement: 任务实施确认闭环

系统 SHALL 明确区分“任务草案被敲定”与“实现成果被用户确认”，并保证 `impl-verify` 到 `confirm` 的闭环清晰可追踪。

#### Scenario: 成果确认阶段不等同于任务敲定命令
- **WHEN** 任务进入 `confirm` 阶段
- **THEN** 系统将其视为用户对实现成果的确认环节
- **AND** 不将 `aide confirm` 命令的草案敲定行为与之混淆

### Requirement: 正式收尾与归档闭环

系统 SHALL 在用户确认通过后，支持任务从完成状态进入正式收尾、归档与常驻分支收束闭环。

#### Scenario: finish 完成正式任务收尾
- **WHEN** 用户已经确认任务结果可接受
- **THEN** 系统指导或执行正式收尾动作
- **AND** 完成任务归档、分支收束与需要长期保留的信息同步

#### Scenario: 归档不等同于简单切换分支
- **WHEN** 任务满足归档条件
- **THEN** 系统将归档视为正式生命周期动作
- **AND** 不把“只切回常驻分支”视为任务已经结束

### Requirement: 草案冲突与任务冲突保护

系统 SHALL 在草案残留、任务并存或状态冲突时给出明确保护和提示，避免用户误将冲突状态继续推进。

#### Scenario: 草案残留时阻止误判为干净状态
- **WHEN** `task-now.md` 存在未处理变更或 `task-now/` 目录残留
- **THEN** 系统明确提示当前存在草案冲突或未完成草案
- **AND** 不将项目判定为可直接开始新任务的干净状态

#### Scenario: 未归档任务并存时提供明确接续入口
- **WHEN** 项目存在多个未归档任务
- **THEN** 系统向用户展示可接续任务
- **AND** 明确哪些动作会影响当前草案或当前任务状态
