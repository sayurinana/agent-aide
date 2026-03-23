# Aide 查看当前状态

你正在执行 Aide 的状态判断入口。

这个 command 的职责是：先理解体系，再读取真实状态，再决定下一步建议。

## 开始前

如果你还没有完整理解以下文档，先完整阅读并学习：

- `aide-memory/aide-process-overview.md`
- `aide-memory/AGENT.md`

随后学习 `aide` skill 中与以下命令有关的部分：
- `aide hi`
- `aide go`
- `aide bye`
- `aide flow`
- `aide verify`
- `aide confirm`
- `aide archive`

## 执行步骤

### 1. 执行 `aide hi`

先执行：

```bash
aide hi
```

只有当用户明确需要更多细节，或你判断精简输出不足以支撑决策时，才执行：

```bash
aide hi -v
```

### 2. 理解输出代表什么

你需要明确解释：
- 当前所在分支属于常驻分支、任务分支还是其他分支
- 当前是否存在未归档任务
- 当前任务是否处于可继续实施、待确认、待归档或已收尾状态

解释时以 `aide hi` 的真实输出为准，不要自行臆造状态。

### 3. 判断是否需要载入 memory

如果出现以下任一情况，应继续学习 `load-memory` skill 并载入 memory：
- 用户要求继续之前的任务
- 需要理解项目结构或任务上下文
- `aide hi` 输出不足以支持下一步判断

如果当前只是在做纯状态确认，且 `aide hi` 已足够回答问题，可以不额外载入 memory。

### 4. 给出建议行动

结合 `aide hi` 的输出和已载入的项目 memory，向用户给出下一步建议。

建议应尽量落到具体动作，例如：
- 执行 `/aide:go`
- 先执行 `/aide:load-memory`
- 继续当前阶段对应的 skill
- 执行 `aide verify` / `aide confirm` / `aide archive`
- 当前无需动作，仅记录状态

## 输出要求

你的输出至少包含三部分：
- 当前状态摘要
- 是否已载入或建议载入 memory
- 建议的下一步行动

## 预期行为

这个 command 不负责直接推进实施，而是负责做一次可靠的状态诊断和下一步导航。
