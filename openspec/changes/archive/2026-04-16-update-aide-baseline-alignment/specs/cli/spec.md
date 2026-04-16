## MODIFIED Requirements

### Requirement: 状态查询与任务接续命令

系统 SHALL 提供 `aide hi` 与 `aide go` 命令，使用户和总工程师 Agent 能基于当前 workflow truth 识别项目状态并接续任务。

#### Scenario: `aide hi` 展示常驻分支上的完整状态
- **WHEN** 用户在常驻分支执行 `aide hi`
- **THEN** 系统显示项目绝对路径与当前分支
- **AND** 根据 workflow truth 显示未归档任务、草案残留、仓库干净状态与下一步建议
- **AND** 如果存在未归档任务，显示各任务摘要与最近活跃信息

#### Scenario: `aide hi` 识别草案进行中状态
- **WHEN** 用户在常驻分支执行 `aide hi`
- **AND** `task-now.md` 已修改或 `aide-memory/tasks/task-now/` 存在
- **THEN** 系统将其识别并展示为草案进行中
- **AND** 不将当前项目错误展示为干净状态

#### Scenario: `aide hi` 在其他分支给出可操作提示
- **WHEN** 用户在既非常驻分支也非任务分支的分支上执行 `aide hi`
- **THEN** 系统说明当前分支不属于 aide 管理范围
- **AND** 明确提示常驻分支是否存在以及用户可采取的后续动作

#### Scenario: `aide go` 接续唯一未归档任务
- **WHEN** 用户执行 `aide go`
- **AND** 当前仅存在一个未归档任务
- **THEN** 系统自动切换到该任务分支
- **AND** 恢复任务摘要、阶段信息与后续建议

#### Scenario: `aide go -v` 在无明确目标时补充状态信息
- **WHEN** 用户执行 `aide go -v`
- **AND** 命令无法直接确定唯一接续任务
- **THEN** 系统输出足够详细的状态信息
- **AND** 帮助用户决定下一步接续目标

### Requirement: 工作区清理与离场命令

系统 SHALL 提供 `aide bye` 命令，用于按当前状态完成安全离场，并在需要时引导进入任务收尾闭环。

#### Scenario: 常驻分支安全离场
- **WHEN** 用户在常驻分支执行 `aide bye`
- **THEN** 系统检查仓库状态
- **AND** 在需要时按配置完成暂存提交
- **AND** 输出离场结果

#### Scenario: 未完成任务的离场
- **WHEN** 用户在任务分支执行 `aide bye`
- **AND** 当前任务尚未进入正式结束阶段
- **THEN** 系统完成必要的暂存提交
- **AND** 切回常驻分支
- **AND** 明确说明任务只是暂停而不是结束

#### Scenario: 已完成任务的离场
- **WHEN** 用户在任务分支执行 `aide bye`
- **AND** 当前任务已满足正式结束条件
- **THEN** 系统引导或串联进入任务收尾闭环
- **AND** 不把“简单切回常驻分支”误判为任务正式结束

### Requirement: 任务管理命令集

系统 SHALL 提供 `aide verify`、`aide confirm` 与 `aide archive` 命令，分别用于审验草案、敲定任务和归档任务。

#### Scenario: `aide verify` 审验任务草案
- **WHEN** 用户执行 `aide verify`
- **THEN** 系统检查草案目录中的必需文件、阶段声明、图解规则与关键格式要求
- **AND** 明确输出通过、失败与警告项

#### Scenario: `aide confirm` 敲定任务
- **WHEN** 用户执行 `aide confirm`
- **AND** 草案审验通过
- **THEN** 系统分配任务编号、重命名草案目录、重置任务描述文件、创建任务分支并更新分支映射

#### Scenario: `aide archive` 归档任务
- **WHEN** 用户执行 `aide archive <n>`
- **THEN** 系统将任务目录移至 `archived-tasks/`
- **AND** 更新分支映射与归档状态
- **AND** 输出归档结果

### Requirement: 阶段管理命令集

系统 SHALL 提供 `aide flow status`、`next`、`back`、`list` 与 `show` 命令，用于展示和推进阶段级 workflow。

#### Scenario: `aide flow status` 展示阶段流
- **WHEN** 用户执行 `aide flow status`
- **THEN** 系统展示当前任务的阶段序列
- **AND** 标记已完成、当前和待执行阶段
- **AND** 在适用时展示当前 preset 与循环模式信息

#### Scenario: `aide flow back` 执行返工回退
- **WHEN** 用户执行 `aide flow back <phase>`
- **THEN** 系统回退到目标阶段
- **AND** 明确提示必须重新经过的后续阶段
- **AND** 保留返工所需的可追溯信息

### Requirement: CLI 输出一致性约束

系统 SHALL 对 aide CLI 的状态输出采用一致的路径、时间与符号规范。

#### Scenario: 输出路径使用绝对路径
- **WHEN** `aide` 命令输出项目路径、配置路径或任务相关路径
- **THEN** 系统使用绝对路径表达这些位置

#### Scenario: 输出时间使用统一口径
- **WHEN** 系统展示任务最后提交时间或最近活跃时间
- **THEN** 系统使用统一的目标时区口径输出完整时间与相对时间

#### Scenario: 输出符号保持一致
- **WHEN** 系统输出成功、警告、失败与进行中状态
- **THEN** 系统统一使用既定符号规范

### Requirement: 配置键与命令行为一致性

系统 SHALL 保证用户可见的配置键、配置文档与 CLI 实际消费路径保持一致。

#### Scenario: 用户根据配置文档读取命令配置
- **WHEN** 用户或 Agent 通过配置文档或 `aide config` 查询键值
- **THEN** 所见键路径应与 CLI 实际读取行为一致
- **AND** 不出现文档平铺键名与程序嵌套键名长期并存的歧义
