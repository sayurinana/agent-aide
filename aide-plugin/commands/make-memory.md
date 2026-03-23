# Aide 生成项目 Memory

你正在执行 Aide 的项目记忆生成入口。

这个 command 不直接展开完整方法论，只负责把你引导到正确的角色、正确的 skill 和正确的产出位置。

## 开始前

如果你还没有完整理解以下文档，先完整阅读并学习：

- `aide-memory/aide-process-overview.md`
- `aide-memory/AGENT.md`

重点理解：
- Aide 的整体阶段和情境
- 总工程师与专家子代理的职责边界
- `aide-memory/` 目录中 memory 的位置和用途

## 执行步骤

### 1. 确认角色分工

你是统筹全局的总工程师，不直接把大量认知整理细节和总体决策混在一起。

如果宿主支持子代理或专家代理能力：
- 创建一个只负责项目认知与 memory 编写的子代理
- 明确其工作边界仅限于项目探索与 `aide-memory/memory/` 下的文档产出

如果宿主不支持子代理：
- 由你自己继续执行
- 但仍需严格遵守“先探索、后抽象、再汇总”的边界

### 2. 学习 `make-memory` skill

让负责该工作的代理完整学习 `make-memory` skill，并严格按该 skill 的方法执行。

该 skill 的目标是：
- 递归理解项目结构
- 生成结构记忆
- 生成概念记忆
- 生成图解记忆
- 生成总览文档

### 3. 产出或更新 memory

按 skill 约定生成或更新以下内容：
- `aide-memory/memory/overview.md`
- `aide-memory/memory/structure/`
- `aide-memory/memory/concepts/`
- `aide-memory/memory/diagram/`

### 4. 进行总工程师验收

由你负责做最终检查，重点看：
- 是否覆盖了关键目录、核心模块和主要依赖
- 是否已经形成“结构 + 概念 + 图解 + 总览”的完整闭环
- 是否存在明显遗漏、过时内容或命名混乱

必要时让负责 memory 的代理补写或修订。

## 输出要求

向用户汇报时至少说明：
- 本次是否新建或更新了 memory
- 产出了哪些文档
- 还缺少哪些上下文，后续在哪些任务中应继续补齐

## 预期行为

这个 command 的目标不是直接解决业务任务，而是为后续 `/aide:load-memory`、`/aide:hi`、`/aide:go` 提供可靠的项目认知基础。
