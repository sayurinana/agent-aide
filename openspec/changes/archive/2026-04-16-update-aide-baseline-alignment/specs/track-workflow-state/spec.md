## ADDED Requirements

### Requirement: 可叠加情境模型

系统 SHALL 使用可叠加的 workflow 情境来表达项目当前状态，至少包括干净状态、不干净状态、草案进行中与任务实施中。

#### Scenario: 草案进行中与任务实施中可同时成立
- **WHEN** 项目存在未归档任务
- **AND** `task-now.md` 已修改或 `aide-memory/tasks/task-now/` 存在
- **THEN** 系统同时识别任务实施中与草案进行中两类情境
- **AND** 不将其简化为单一状态

#### Scenario: 干净状态要求无草案残留
- **WHEN** 项目位于常驻分支或任务分支
- **AND** Git 状态干净
- **AND** 配置有效
- **AND** 不存在草案残留
- **THEN** 系统可识别为干净状态

### Requirement: 标准阶段流程声明与提取

系统 SHALL 通过任务文档中的阶段声明定义任务所采用的阶段流程，并将其作为后续 flow 管理、命令判断和阶段性 skill 加载的基础真值。

#### Scenario: 从 `todo.md` 提取阶段流程
- **WHEN** 任务文档包含阶段声明
- **THEN** 系统从该声明提取阶段顺序
- **AND** 正确识别 `impl-verify:loop` 等特殊阶段模式

#### Scenario: 阶段流程以 `build-task` 开始并以 `finish` 结束
- **WHEN** 用户创建正式任务草案
- **THEN** 阶段流程以 `build-task` 作为起始阶段
- **AND** 以 `finish` 作为最终阶段

### Requirement: 固定阶段与可选阶段约束

系统 SHALL 区分每个任务都必须经过的固定阶段与按任务特性插入的可选阶段。

#### Scenario: 固定阶段始终存在
- **WHEN** 任务被定义为正式任务
- **THEN** 阶段流程必须包含 `build-task`、`impl-verify`、`confirm`、`finish`

#### Scenario: 可选阶段按任务特性插入
- **WHEN** 任务需要图解、集成测试、审查或文档同步
- **THEN** 阶段流程可按规则插入 `make-graphics`、`integration`、`review`、`docs-update`
- **AND** 插入后的相对顺序满足 workflow 约束

### Requirement: 标准预设定义

系统 SHALL 提供标准预设，用于表达常见任务场景下的阶段流程，并保证每个预设的阶段定义与 current truth 一致。

#### Scenario: `full` 预设使用完整质量保证链路
- **WHEN** 任务选择 `full` 预设
- **THEN** 阶段流程包含 `build-task`、`make-graphics`、`impl-verify:loop`、`integration`、`review`、`docs-update`、`confirm`、`finish`

#### Scenario: `standard` 预设不引入额外的 integration 与 docs-update
- **WHEN** 任务选择 `standard` 预设
- **THEN** 阶段流程为 `build-task`、`impl-verify:loop`、`review`、`confirm`、`finish`

#### Scenario: `lite` 预设使用最短固定链路
- **WHEN** 任务选择 `lite` 预设
- **THEN** 阶段流程为 `build-task`、`impl-verify`、`confirm`、`finish`

#### Scenario: `docs` 预设服务于文档类工作
- **WHEN** 任务选择 `docs` 预设
- **THEN** 阶段流程为 `build-task`、`impl-verify`、`review`、`confirm`、`finish`

#### Scenario: `research` 预设保留图解与文档更新阶段
- **WHEN** 任务选择 `research` 预设
- **THEN** 阶段流程为 `build-task`、`make-graphics`、`impl-verify`、`docs-update`、`confirm`、`finish`

### Requirement: 预设选择、识别与用户可见展示

系统 SHALL 同时支持显式选择预设和基于阶段流程识别预设，并在适当的状态输出中向用户展示预设信息。

#### Scenario: 任务文档显式选择预设
- **WHEN** 用户或 Agent 在构建任务时为任务选定标准预设
- **THEN** 系统保存该预设信息
- **AND** 后续阶段管理据此校验阶段流程

#### Scenario: 根据阶段流程识别预设
- **WHEN** 系统读取任务已声明的阶段流程
- **THEN** 系统识别该流程属于哪个标准预设或自定义流程
- **AND** 在状态存储中记录识别结果

#### Scenario: `aide flow status` 展示当前预设
- **WHEN** 用户执行阶段状态查看命令
- **THEN** 系统显示当前任务对应的预设名称或标记为自定义流程

### Requirement: 返工回退规则

系统 SHALL 支持返工回退，并保证返工后后续阶段的重新经过与原因记录具备可追溯性。

#### Scenario: 从任意阶段发起返工
- **WHEN** 当前阶段发现方向性偏差或阶段产物不满足要求
- **THEN** 系统允许回退到符合规则的更早阶段
- **AND** 明确标出需要重新经过的后续阶段

#### Scenario: 返工原因可追溯
- **WHEN** 任务被回退到更早阶段
- **THEN** 系统记录返工原因或返工上下文
- **AND** 后续接续者能够理解回退发生的原因与影响
