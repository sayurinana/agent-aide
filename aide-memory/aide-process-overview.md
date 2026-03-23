# Aide 过程总览

本文档给出 Aide 体系的全局流程视图，帮助 Agent 在开始任何工作前先建立统一的过程认知。

## 一、体系定位

Aide 不是单个命令，也不是单份提示词，而是一套围绕以下四类对象协同工作的体系：

- `commands`：入口指引，负责告诉 Agent 现在应该做什么
- `skills`：方法手册，负责告诉 Agent 具体怎么做
- `aide` 程序：负责状态查询、分支切换、阶段推进和任务管理
- `aide-memory/`：负责保存项目记忆、任务文档和全局说明

## 二、默认预设

进入 Aide 体系后，默认遵循以下预设：

1. 先看全局，再进细节。首次进入项目时，先读本文与 `AGENT.md`。
2. 先读真实状态，再做判断。状态类问题以 `aide hi` 输出为准。
3. memory 是项目长期认知的主存，不依赖单次对话记忆。
4. command 只做入口导航，具体方法交给对应 skill。
5. 总工程师负责统筹、验收和沟通，具体深入工作可交给专家子代理或受限执行上下文。

## 三、目录视角

新体系围绕 `aide-memory/` 工作：

```text
aide-memory/
├── memory/
│   ├── overview.md
│   ├── structure/
│   ├── concepts/
│   └── diagram/
├── tasks/
├── archived-tasks/
├── aide-process-overview.md
└── AGENT.md
```

其中：
- `memory/` 保存项目长期认知
- `tasks/` 保存未归档任务
- `archived-tasks/` 保存已归档任务
- 本文和 `AGENT.md` 提供全局运行说明

## 四、常见情境

### 1. 初次接管项目

适用场景：
- 第一次进入项目
- 不确定当前项目状态
- 还没有可靠的项目认知

建议顺序：
1. 阅读本文
2. 阅读 `aide-memory/AGENT.md`
3. 执行 `/aide:hi`
4. 若缺少 memory，执行 `/aide:make-memory`
5. 若已有 memory，执行 `/aide:load-memory`

### 2. 进入或接续任务

适用场景：
- 用户要求继续之前的工作
- 项目已有未归档任务

建议顺序：
1. `/aide:hi`
2. `/aide:load-memory`
3. `/aide:go`
4. 根据当前阶段进入对应 skill

### 3. 新建正式任务

适用场景：
- 用户提出一个尚未进入 `tasks/` 的新任务
- 当前需要先把任务打磨为可实施文档

建议顺序：
1. `/aide:hi`
2. 视情况 `/aide:load-memory`
3. 进入 `build-task`
4. 视需要进入 `make-graphics`
5. 完成后执行 `aide verify` 和 `aide confirm`

### 4. 返工

适用场景：
- 用户反馈现有结果不符合预期
- 发现需求理解、设计或实现路径存在偏差

建议顺序：
1. 先确认当前阶段与问题影响面
2. 进入 `rework`
3. 回退到需要重做的阶段
4. 重新经过后续必要阶段

### 5. 结束本轮工作

适用场景：
- 本轮对话告一段落
- 需要安全切回常驻分支

建议顺序：
1. `/aide:hi`
2. 必要时补做 verify / confirm / archive
3. `/aide:bye`

## 五、Commands 入口

当前体系中，command 的定位是“入口导航”：

| Command | 作用 |
|---------|------|
| `/aide:make-memory` | 生成或更新项目 memory |
| `/aide:load-memory` | 按需载入项目 memory |
| `/aide:hi` | 读取当前状态并给出建议 |
| `/aide:go` | 进入任务分支并恢复上下文 |
| `/aide:bye` | 收尾并离场 |

这些 command 本身不展开全部细节，而是负责把 Agent 导到正确的 skill 和正确的下一步。

## 六、Skills 分类

### 1. 基础 Skills

用于建立项目认知：

- `make-memory`
- `load-memory`

### 2. 固定阶段 Skills

每个正式任务都必须经过的核心阶段：

- `build-task`
- `impl-verify`
- `confirm`
- `finish`

### 3. 可选阶段 Skills

根据任务特性启用：

- `make-graphics`
- `integration`
- `review`
- `docs-update`
- `rework`

### 4. 参考 Skills

不直接对应某一阶段，但会被多个阶段复用：

- `aide`
- `plantuml`

## 七、阶段流程

任务阶段由 `todo.md` 中的流程声明决定。典型流程如下：

```text
build-task
  -> make-graphics（可选）
  -> impl-verify（可循环）
  -> integration（可选）
  -> review（可选）
  -> docs-update（可选）
  -> confirm
  -> finish
```

说明：
- `build-task` 负责把任务打磨成可实施的任务文档
- `impl-verify` 是主循环，每完成一个任务点就立即审验
- 可选阶段是否启用，由任务特性决定
- `confirm` 面向用户做结果确认
- `finish` 做最终归档和收尾

### 阶段与 skill 对应表

| 阶段 | 对应 skill | 说明 |
|------|------------|------|
| `build-task` | `build-task` | 产出并完善任务文档 |
| `make-graphics` | `make-graphics` | 视需要补充图解 |
| `impl-verify` | `impl-verify` | 实施并即时审验 |
| `integration` | `integration` | 做整体集成检查 |
| `review` | `review` | 做质量评审或方案审校 |
| `docs-update` | `docs-update` | 更新项目文档与 memory |
| `confirm` | `confirm` | 面向用户做结果确认 |
| `finish` | `finish` | 收尾、归档、同步全局信息 |

### 任务管理命令

除了阶段 skill，Aide 还通过以下程序命令管理任务状态：

- `aide verify`：审验当前任务文档是否满足进入确认的条件
- `aide confirm`：敲定任务并创建正式任务分支
- `aide archive`：将已完成任务移入归档区

## 八、建议的实际执行顺序

### 情况 A：只是看状态

1. `/aide:hi`
2. 必要时 `/aide:load-memory`
3. 给出建议，不急于改动代码或文档

### 情况 B：继续一个已有任务

1. `/aide:hi`
2. `/aide:load-memory`
3. `/aide:go`
4. 根据当前阶段调用相应 skill

### 情况 C：开始一个新任务

1. `/aide:hi`
2. 必要时 `/aide:load-memory`
3. 进入 `build-task`
4. 视需要进入 `make-graphics`
5. 执行 `aide verify` 与 `aide confirm`

### 情况 D：补齐项目认知

1. `/aide:make-memory`
2. `/aide:load-memory`
3. 再进入 `/aide:hi` 或 `/aide:go`

## 九、判断原则

当你不确定该走哪条路径时，优先遵循：

1. 先确认状态，而不是猜状态。
2. 先确认 memory 是否可用，而不是假设已有认知。
3. 先确认当前阶段，再决定调用哪个 skill。
4. 先做局部专家产出，再由总工程师统一验收与对外沟通。

## 十、一句话总结

Aide 的运行方式可以概括为：

先用 command 找入口，再用 skill 做方法，用 `aide` 程序读真实状态和推进流程，用 `aide-memory/` 保存长期认知与任务上下文。
