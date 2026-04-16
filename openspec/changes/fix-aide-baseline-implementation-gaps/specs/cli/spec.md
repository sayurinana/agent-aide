## MODIFIED Requirements

### Requirement: 状态查询与任务接续命令

系统 SHALL 提供 `aide hi` 与 `aide go` 命令，使用户和总工程师 Agent 能基于当前 workflow truth 识别项目状态、理解冲突并接续任务。

#### Scenario: `aide hi` 在常驻分支汇总可叠加情境
- **WHEN** 用户在常驻分支执行 `aide hi`
- **THEN** 系统显示项目绝对路径与当前分支
- **AND** 分别评估 Git 干净状态、草案进行中状态与未归档任务状态
- **AND** 将草案残留、任务并存与下一步建议一起展示

#### Scenario: `aide hi` 在其他分支检查常驻分支可用性
- **WHEN** 用户在既非常驻分支也非任务分支的分支上执行 `aide hi`
- **THEN** 系统说明当前分支不属于 aide 管理范围
- **AND** 明确提示常驻分支是否存在
- **AND** 提示用户可执行的后续动作

#### Scenario: `aide go` 自动接续唯一未归档任务
- **WHEN** 用户执行 `aide go`
- **AND** 当前仅存在一个未归档任务
- **THEN** 系统自动切换到该任务分支
- **AND** 恢复任务摘要、阶段信息与后续建议

#### Scenario: `aide go -v` 在无法直接确定目标时输出详细状态
- **WHEN** 用户执行 `aide go -v`
- **AND** 命令无法直接确定唯一接续任务
- **THEN** 系统输出与 `aide hi -v` 同级别的详细状态信息
- **AND** 帮助用户决定下一步接续目标

#### Scenario: `aide go -v` 在切换成功后展示任务详情
- **WHEN** 用户执行 `aide go -v`
- **AND** 系统已成功切换到目标任务分支
- **THEN** 系统继续展示当前任务的详细状态
- **AND** 不要求用户额外手动执行一次 `aide hi -v`

### Requirement: 工作区清理与离场命令

系统 SHALL 提供 `aide bye` 命令，用于按当前状态完成安全离场，并在需要时引导进入任务收尾闭环。

#### Scenario: 常驻分支安全离场
- **WHEN** 用户在常驻分支执行 `aide bye`
- **THEN** 系统检查仓库状态
- **AND** 在需要时按配置完成暂存提交
- **AND** 输出离场结果

#### Scenario: 未完成任务的离场
- **WHEN** 用户在任务分支执行 `aide bye`
- **AND** 当前任务尚未进入正式结束条件
- **THEN** 系统完成必要的暂存提交
- **AND** 切回常驻分支
- **AND** 明确说明任务只是暂停而不是结束

#### Scenario: 已完成任务的离场进入正式收尾路径
- **WHEN** 用户在任务分支执行 `aide bye`
- **AND** 当前任务已满足正式结束条件
- **THEN** 系统引导进入 `finish` / `archive` 闭环
- **AND** 不把“简单切回常驻分支”误判为任务正式结束

### Requirement: 阶段管理命令集

系统 SHALL 提供 `aide flow status`、`next`、`back`、`list` 与 `show` 命令，用于展示和推进阶段级 workflow。

#### Scenario: `aide flow status` 与 `show` 展示预设、循环与阶段历史
- **WHEN** 用户执行 `aide flow status` 或 `aide flow show <task-id>`
- **THEN** 系统展示当前任务的阶段序列
- **AND** 标记已完成、当前和待执行阶段
- **AND** 在适用时展示当前 preset、loop 信息与阶段变更历史

#### Scenario: `aide flow next` 推进到下一阶段
- **WHEN** 用户执行 `aide flow next`
- **THEN** 系统按既定阶段流程推进到下一阶段
- **AND** 在适用时保留阶段循环与返工后的状态信息

#### Scenario: `aide flow back <phase> [reason]` 记录返工原因
- **WHEN** 用户执行 `aide flow back <phase>`
- **AND** 可选地提供返工原因
- **THEN** 系统回退到目标阶段
- **AND** 明确提示必须重新经过的后续阶段
- **AND** 保留返工原因与返工上下文的可追溯信息

#### Scenario: `aide flow list` 与 `show` 使用一致的任务标识
- **WHEN** 用户执行 `aide flow list` 或 `aide flow show <task-id>`
- **THEN** 系统使用与任务目录和任务状态一致的任务标识
- **AND** 不把阶段名误作为任务详情查询参数

### Requirement: CLI 输出一致性约束

系统 SHALL 对 aide CLI 的状态输出采用一致的路径、时间与详细信息格式规范。

#### Scenario: 输出路径使用绝对路径
- **WHEN** `aide` 命令输出项目路径、配置路径或任务相关路径
- **THEN** 系统使用绝对路径表达这些位置

#### Scenario: 输出时间固定为 UTC+8
- **WHEN** 系统展示任务最后提交时间或最近活跃时间
- **THEN** 系统使用 UTC+8 口径输出完整时间与相对时间

#### Scenario: 详细状态使用统一展示格式
- **WHEN** 用户执行详细模式命令
- **THEN** 系统以一致格式展示 Git 状态、路径、PlantUML 处理结果与任务细节
- **AND** 不因命令成功或失败路径不同而丢失必要状态信息

## ADDED Requirements

### Requirement: PlantUML 自动处理与状态反馈

系统 SHALL 在需要编译 PlantUML 时优先处理与当前任务或当前工作区变更相关的图解文件，并向用户反馈处理结果。

#### Scenario: 优先处理变更图解文件
- **WHEN** `aide hi`、`aide verify` 或其他入口触发 PlantUML 处理
- **AND** 系统能够识别当前任务或当前工作区的变更图解文件
- **THEN** 系统优先编译这些变更文件
- **AND** 避免每次都对整个仓库执行全量扫描

#### Scenario: 无法确定变更集合时执行兜底扫描
- **WHEN** 系统无法可靠确定变更图解集合
- **THEN** 系统执行既定的兜底扫描策略
- **AND** 明确向用户说明当前采用的是兜底处理路径
