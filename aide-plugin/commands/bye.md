# Aide 收尾并离场

你正在执行 Aide 的收尾入口。

这个 command 的职责是：先看清当前状态，再决定是否需要补做收尾动作，最后再执行离场。

## 开始前

如果你还没有完整理解以下文档，先完整阅读并学习：

- `aide-memory/aide-process-overview.md`
- `aide-memory/AGENT.md`

随后学习 `aide` skill 中与以下命令有关的部分：
- `aide hi`
- `aide bye`
- `aide flow`
- `aide verify`
- `aide confirm`
- `aide archive`

## 执行步骤

### 1. 先执行 `aide hi`

先执行：

```bash
aide hi
```

用它判断当前处于哪一种状态：
- 已在常驻分支
- 仍在任务分支
- 当前任务已完成但尚未确认或归档
- 当前仍有未整理的实施结果

### 2. 判断是否需要补做收尾操作

根据状态决定是否先做以下动作：
- 更新任务摘要或 todo 进度
- 执行 `aide verify`
- 执行 `aide confirm`
- 执行 `aide archive`

如果当前只是结束本轮会话，而不是完成整个任务，也可以只做最小必要收尾。

### 3. 执行 `aide bye`

在确认可以离场后，执行：

```bash
aide bye
```

如果 `aide bye` 输出了自动暂存、提交或切回常驻分支的信息，应如实向用户解释其含义。

### 4. 给出下次接续建议

根据本次收尾后的状态，告诉用户下次更适合从哪里继续，例如：
- `/aide:hi`
- `/aide:go 3`
- 继续某个阶段 skill
- 直接开始新任务

## 输出要求

你的输出至少包含：
- 收尾前状态
- 实际执行了哪些补充收尾动作
- `aide bye` 的结果
- 下次接续建议

## 预期行为

这个 command 的目标是安全、可追踪地结束当前一轮工作，而不是简单退出。
