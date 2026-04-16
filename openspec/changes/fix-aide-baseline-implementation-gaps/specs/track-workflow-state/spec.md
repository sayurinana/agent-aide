## MODIFIED Requirements

### Requirement: 预设选择、识别与用户可见展示

系统 SHALL 同时支持显式选择预设和基于阶段流程识别预设，并在状态输出与阶段历史中向用户展示 preset 与 loop 信息。

#### Scenario: 根据阶段流程识别标准预设或自定义流程
- **WHEN** 系统读取任务已声明的阶段流程
- **THEN** 系统识别该流程属于 `full`、`standard`、`lite`、`docs`、`research` 中的哪个标准预设
- **AND** 当流程不匹配任何标准预设时标记为自定义流程

#### Scenario: `aide flow status` 与 `show` 展示 preset 和 loop 信息
- **WHEN** 用户执行阶段状态查看命令
- **THEN** 系统显示当前任务对应的预设名称或标记为自定义流程
- **AND** 展示哪些阶段启用了 loop 模式

#### Scenario: 当前循环阶段在状态中可见
- **WHEN** 当前阶段本身启用了 loop 模式
- **THEN** 系统在状态展示中明确标示该阶段可循环
- **AND** 不把 loop 信息隐藏在内部状态中

### Requirement: 返工回退规则

系统 SHALL 支持返工回退，并保证返工后必须重新经过的阶段、返工原因与阶段历史具备可追溯性。

#### Scenario: 从任意阶段发起返工并计算后续重走阶段
- **WHEN** 当前阶段发现方向性偏差或阶段产物不满足要求
- **THEN** 系统允许回退到符合规则的更早阶段
- **AND** 明确标出需要重新经过的后续阶段

#### Scenario: 返工原因写入状态历史
- **WHEN** 用户执行 `aide flow back` 时提供返工原因
- **THEN** 系统将该原因写入阶段状态历史
- **AND** 后续接续者可通过 `aide flow show` 理解返工发生的原因与影响
