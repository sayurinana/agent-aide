# 核心子过程 Skills 变更

## ADDED Requirements

### Requirement: build-task 阶段 skill

系统 SHALL 提供 `build-task` skill，用于把原始任务输入打磨为符合 aide 体系规范的任务草案。

该 skill 必须：

- 读取用户指定或 aide 程序输出的任务解析指导文档
- 指导产出 `information.md`、`design.md`、`todo.md`、`task-summary.md`
- 要求 `todo.md` 中根据任务特性声明阶段流程
- 要求 `design.md` 中明确图解标记
- 引导用户对任务内容持续澄清和打磨

#### Scenario: 基于解析指导文档构建任务草案

- **WHEN** Agent 进入 `build-task` 阶段并收到任务原始输入与解析指导文档路径
- **THEN** Agent 应先按指导文档理解和澄清需求
- **AND** 产出 `information.md`、`design.md`、`todo.md`、`task-summary.md`
- **AND** 确保 `todo.md` 的阶段流程以 `build-task` 开始
- **AND** 确保 `design.md` 标记是否需要图解

### Requirement: impl-verify 阶段 skill

系统 SHALL 提供 `impl-verify` skill，用于按 `todo.md` 的任务点逐一实施并即时审验。

该 skill 必须：

- 按任务点小步推进实施
- 每完成一个任务点立即执行对应验证
- 审验通过后回写 `todo.md` 与 `task-summary.md`
- 审验失败时优先在当前阶段内修复
- 在需要循环推进时支持 `impl-verify:loop` 模式

#### Scenario: 循环完成多个实施任务点

- **WHEN** `todo.md` 的阶段声明包含 `impl-verify:loop`
- **THEN** Agent 应重复执行“实施一个任务点并立即审验”的循环
- **AND** 只有在当前阶段负责的任务点全部通过验证后才进入后续阶段
- **AND** 若发现需求或设计偏差，应停止硬做并转入返工处理

### Requirement: confirm 阶段 skill

系统 SHALL 提供 `confirm` skill，用于向用户展示成果、汇总验证结果并收集反馈。

该 skill 必须：

- 明确区分成果确认阶段与 `aide confirm` 任务敲定命令
- 基于真实改动、真实验证结果和真实限制向用户汇报
- 收集用户对结果是否符合预期的反馈
- 在反馈表明需要返工时触发返工决策

#### Scenario: 用户对结果进行确认

- **WHEN** Agent 完成实施与必要的后续阶段并进入 `confirm`
- **THEN** Agent 应向用户展示成果摘要、验证证据和已知限制
- **AND** 主动询问结果是否符合预期、是否有遗漏场景
- **AND** 用户确认通过时进入 `finish`
- **AND** 用户指出方向性问题时转入返工处理

### Requirement: finish 阶段 skill

系统 SHALL 提供 `finish` skill，用于在用户确认通过后完成任务生命周期收尾。

该 skill 必须：

- 使用 aide 程序完成任务收束
- 指导将任务目录从 `tasks/` 移至 `archived-tasks/`
- 指导将任务分支收束回常驻分支
- 指导同步需要长期保留的 memory 信息

#### Scenario: 正式任务完成收尾归档

- **WHEN** 用户已经确认结果可接受，且任务文档状态与真实结果一致
- **THEN** Agent 应先完成最终检查
- **AND** 再使用 aide 程序完成任务分支收束与任务归档
- **AND** 更新需要长期保留的 memory 信息
- **AND** 向用户说明任务已正式结束及后续接续入口
