# Design: fix-aide-baseline-implementation-gaps

## Context

当前仓库已经具备新体系的主体骨架，但 `/repo/agent-aide/report-summary.md` 说明还存在一批跨模块的实现缺口：

1. `FlowPreset` 只有 `full` 对齐基准，其余预设实现不一致；
2. `flow-status` 缺少返工原因，`flow status/show` 对 preset 与 loop 的对外展示不完整；
3. `aide hi/go/bye` 没有完整体现 situations、草案冲突、正式 finish 闭环与详细回显要求；
4. `init/config/templates` 仍残留旧阶段、旧命令、旧 fallback 文本与 `task-parser` 命名；
5. `aide-plugin` 源、`.claude` 副本、Codex 副本与 skills/docs 语义没有完全收口；
6. `aide-memory/memory/` 只有占位内容，尚未形成 `make-memory` / `load-memory` 需要的最小成果集。

这组问题横跨 Rust CLI、flow 数据模型、初始化模板、插件分发、skills 文档和 memory 产物，必须按依赖顺序拆批实施，而不能继续以单点补丁方式推进。

## Goals / Non-Goals

### Goals

- 让 `report-summary.md` 中列出的实现缺口拥有可直接落地的验收标准
- 先修复 flow 状态内核，再推进 CLI 行为与生命周期闭环，减少跨模块返工
- 统一模板、技能、插件副本与 memory 成果的真相源，避免“代码修了、文档没修”
- 在不破坏现有任务数据的前提下补齐状态字段与输出内容
- 为后续 `/openspec:apply` 提供清晰的分批实施顺序与验证策略

### Non-Goals

- 不重新设计 capability 边界；边界沿用现有 six capabilities
- 不引入新的 workflow 阶段或新的宿主分发模型
- 不在 proposal 阶段直接修改实现代码或产物文件
- 不保留旧体系为 current truth，只保留必要的迁移/清理约束

## Decisions

### Decision 1：先修 flow 状态内核，再修 CLI 展现

`preset`、`loop`、`rework reason`、`flow-status` 持久化与 `todo.md` 阶段提取都属于状态内核。只有这些基础 truth 稳定后，`aide flow status/show/back` 与 `aide hi/go/bye` 的展示和决策才不会反复返工。

具体要求：
- 对齐 `full/standard/lite/docs/research` 的 preset 定义
- 让 `flow-status` 能记录返工原因，并兼容已有状态文件
- 在 `flow status/show` 中展示 preset、loop 与返工历史

### Decision 2：明确区分“暂停离场”与“正式结束”

`aide bye` 只负责安全离场；`finish/archive` 才承担正式结束、归档、分支收束与 memory 同步。这样才能避免“切回常驻分支”被误认为任务已经完成。

具体要求：
- 未完成任务执行 `bye` 时，只能视为暂停
- 已达到 finish 条件的任务，`bye` 必须引导进入正式收尾闭环，而不是静默代替 finish
- `archive` 需要与任务目录、分支映射、任务状态保持一致

### Decision 3：模板、解析指导和插件副本统一追溯到单一来源

`task-now.md`、`任务解析指导.md`、`aide-process-overview.md`、skills 文档以及 `.claude/.agents/Codex` 副本必须从各自的真相源统一生成或同步，否则实现修复后很快会再次漂移。

具体要求：
- `build-task` 优先消费程序输出的解析指导文档绝对路径
- `init` 不再回退到过时的旧体系 fallback 文案
- `aide-plugin` 作为分发源，项目副本与宿主副本都要跟它同步收口

### Decision 4：memory 先补最小可用成果集，再做 load-memory 约束

`make-memory` / `load-memory` 的问题不在命令名，而在成果层为空。先补齐最小可用 memory 文档集，才能让 `load-memory` 的行为有稳定输入。

最小成果集包括：
- `overview.md`
- `structure/index.md` 与必要的 structure 子页
- `concepts/term.md`
- `concepts/arch.md`
- 至少一组 `diagram/*.puml`

### Decision 5：提案实施本身必须遵循“command 入口 + 总工程师 + 专家执行单元”

本提案要求的不只是“修哪些内容”，还要求“如何实施这些修复”。实施方式必须与 `task-optimized.50adab8.md` 中 commands 的定位保持一致：command 作为精简入口指引，总工程师负责统筹，专家执行单元负责分边界落地。

具体要求：
- 主 Agent 先通过 `hi` / `go` / `load-memory` 等入口判断状态与进入路径，而不是直接无上下文开工
- 主 Agent 负责把五个批次拆为可验证子任务，并决定哪些工作可以并行
- 每个专家执行单元只处理本批次边界内的问题，不跨批次无限扩张
- 每批结束后由主 Agent 汇总、审阅、决定是否进入下一批或触发返工
- 如果宿主不支持真正子代理，也要在单上下文中显式保持这种分工与节奏

## Implementation Batches

### Batch 1：flow 状态内核
- 修 preset 定义、detect 与 status 展示
- 为返工历史增加 reason 字段与展示
- 补齐 flow 相关单元/集成测试

### Batch 2：CLI 与任务生命周期闭环
- 修 `aide hi/go/bye/flow`
- 补 situations 判断、`go -v` 失败路径回显、finish/archive 边界
- 统一绝对路径、UTC+8 时间与 PlantUML 状态反馈

### Batch 3：配置、初始化与模板来源
- 清理旧阶段、旧命令与旧 fallback 文案
- 打通 `task.parse_guide` 绝对路径输出链路
- 更新 `task-now.md`、模板与核心 overview 文档

### Batch 4：插件、skills 与文档收口
- 以 `aide-plugin` 为源同步 commands/skills
- 收口 `.claude` / `.agents` / Codex 副本
- 清理 `aide decide`、`aide env`、旧 flow step 命令与 `task-parser` 残留

### Batch 5：memory 与回归验证
- 补齐最小可用 memory 文档集
- 校验 `make-memory` / `load-memory` 契约
- 运行 OpenSpec、Rust 测试与关键 CLI 冒烟验证

## Risks / Trade-offs

### Risk 1：flow-status 字段扩展可能影响旧任务数据

缓解：新增字段采用可选字段并保持向后兼容读取；只有在状态更新时才回写新字段。

### Risk 2：文档与插件副本收口容易遗漏

缓解：把 `aide-plugin`、`.claude`、`.agents`、Codex 目标和 `docs/` 一起纳入同一批次，并用搜索校验旧术语是否清理干净。

### Risk 3：memory 产物补齐会放大范围

缓解：先定义“最小可用成果集”，只补齐 `load-memory` 的必要输入，不在本次 change 中扩展额外百科式内容。

## Validation Strategy

- `openspec validate fix-aide-baseline-implementation-gaps --strict --no-interactive`
- 针对 `aide flow`、`aide hi/go/bye`、`aide verify/confirm/archive` 的 Rust 测试
- 针对模板、技能、副本文档与旧术语的搜索校验
- 针对 memory 最小成果集的文件存在性与入口可用性验证
