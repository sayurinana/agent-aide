# Aide 载入项目 Memory

你正在执行 Aide 的项目记忆载入入口。

这个 command 只负责指导你如何载入 memory，不重复展开 skill 内部细节。

## 开始前

如果你还没有完整理解以下文档，先完整阅读并学习：

- `aide-memory/aide-process-overview.md`
- `aide-memory/AGENT.md`

## 执行步骤

### 1. 确认是否存在可用 memory

优先检查 `aide-memory/memory/` 是否已经存在且包含有效内容。

如果 memory 尚不存在或明显缺失：
- 明确告知用户当前缺少项目记忆
- 引导执行 `/aide:make-memory`
- 不要假装已经完成 memory 载入

### 2. 学习 `load-memory` skill

完整学习 `load-memory` skill，并按照该 skill 的方法载入项目 memory。

### 3. 按需载入，而不是一次性全读

载入顺序应遵循：
1. 先读 `aide-memory/memory/overview.md`
2. 再根据当前任务或当前阶段，按需选择 `structure/`、`concepts/`、`diagram/` 下的相关文档

载入目标是建立足够可靠的上下文，而不是机械地把所有 memory 全部读一遍。

### 4. 形成当前任务所需认知

完成载入后，你应能回答：
- 这个项目的大体结构是什么
- 当前任务涉及哪些模块
- 哪些历史认知可以直接复用
- 哪些部分仍需结合代码现状补充确认

## 输出要求

向用户汇报时至少说明：
- 本次载入了哪些 memory 文档
- 当前已建立的项目认知范围
- 仍需后续按需补载的部分

## 预期行为

这个 command 的目标是让你在进入 `/aide:go` 或响应复杂任务前，基于已有 memory 建立正确的项目上下文。
