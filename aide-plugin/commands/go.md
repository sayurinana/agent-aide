# Aide 进入并接续任务

你正在执行 Aide 的任务接续入口。

这个 command 的职责是：先确认体系和上下文，再进入正确任务分支，然后按阶段流程继续推进。

## 开始前

如果你还没有完整理解以下文档，先完整阅读并学习：

- `aide-memory/aide-process-overview.md`
- `aide-memory/AGENT.md`

如果你尚未载入项目 memory，先学习 `load-memory` skill 并完成必要的 memory 载入。

随后学习 `aide` skill 中与以下命令有关的部分：
- `aide hi`
- `aide go`
- `aide flow`
- `aide verify`
- `aide confirm`
- `aide archive`

## 执行步骤

### 1. 先执行 `aide hi`

先通过以下命令确认当前状态：

```bash
aide hi
```

你需要先知道：
- 当前是否已经在任务分支
- 当前有哪些未归档任务
- 当前接续哪一个任务最合理

### 2. 再执行 `aide go`

根据用户输入和 `aide hi` 输出决定调用方式：

- 用户指定了任务编号：执行 `aide go <编号>`
- 用户未指定编号且只有一个未归档任务：直接执行 `aide go`
- 用户未指定编号且存在多个候选任务：先基于状态给出建议，再请用户确认编号

不要在任务不明确时擅自进入错误的任务分支。

### 3. 进入任务后恢复上下文

成功进入任务分支后，你需要继续恢复以下上下文：
- 当前任务摘要
- 当前 todo 进度
- 当前阶段流程
- 已有的项目 memory

### 4. 按阶段继续实施

根据当前任务阶段，调用对应 skill 继续推进：

- `build-task`
- `make-graphics`
- `impl-verify`
- `integration`
- `review`
- `docs-update`
- `confirm`
- `finish`

如遇返工，按 `rework` skill 的方法处理。

### 5. 向用户说明你将继续什么

在真正展开下一阶段工作前，明确告诉用户：
- 已进入哪个任务
- 当前位于哪个阶段
- 接下来将调用哪个 skill 或执行哪个环节

## 输出要求

向用户汇报时至少说明：
- `aide hi` 和 `aide go` 的关键结果
- 当前接续的任务编号和阶段
- 下一步要执行的 skill 或任务点

## 预期行为

这个 command 的目标是把你带入正确的任务现场，并让你从已有状态平滑接续，而不是重新从零开始猜测上下文。
