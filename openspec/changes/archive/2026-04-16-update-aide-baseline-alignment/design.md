# Design: update-aide-baseline-alignment

## Context

当前仓库已经部分实现了 50adab8 基准中的新体系，但主规范层只保留了 `cli` 与 `plugin` 两个 capability，覆盖范围偏向初始化、同步和插件分发。审验报告表明，真正影响行为一致性的核心内容——工作流情境、阶段流转、场景预设、任务生命周期、memory 结构、commands/skills 语义、Agent 角色约束——要么未进入 current specs，要么只停留在 archive change 与实现细节中。

这导致以下问题长期并存：

1. current specs 无法完整表达当前系统应当如何工作；
2. archive 中存在更贴近目标体系的历史规范，但没有系统性回流 current truth；
3. 代码、文档、技能说明、配置默认值与审验基准之间持续漂移；
4. 后续修复缺少统一的 current spec 作为对照标准。

本设计聚焦于 capability 边界重建与 current truth 收敛，不讨论具体代码实现方案。

## Goals / Non-Goals

### Goals

- 把 50adab8 基准中已经被审验证明为核心的行为约束收敛到 current specs
- 以清晰 capability 边界承载工作流状态、任务生命周期、memory 结构与 workflow guidance
- 明确哪些要求属于 CLI 表层契约，哪些要求属于 workflow domain 语义，避免后续重复定义
- 为后续完整修复提供可验证、可分工、可归档的 OpenSpec 基线
- 明确 apply 阶段的实施组织方式必须采用“总工程师统筹 + 子代理/专家执行单元分工”的模式，而不是由单一 Agent 直接包办全部修复

### Non-Goals

- 不在本提案阶段修改代码、模板或文档实现
- 不试图保留旧体系的兼容语义；旧体系仅作为待清理对象被描述
- 不把 archive change 原样搬回 current specs；必须以 current truth 的形式重写 requirements
- 不在单个 capability 中混合过多横切关注点

## Capability Boundary Decisions

### 1. `cli`

`cli` 负责对用户直接可见的 `aide` 命令契约，包括命令输入、输出、状态提示、配置消费方式与错误处理。它不承载完整 workflow 语义，只引用 workflow capability 中的 domain 规则。

应放入 `cli` 的内容：
- `aide hi/go/bye`
- `aide verify/confirm/archive`
- `aide flow status/next/back/list/show`
- 路径输出、时间口径、输出符号、详细模式等 CLI 约束

### 2. `plugin`

`plugin` 负责插件仓库同步、项目级命令/skills 分发、Claude/Codex 双宿主部署与同步失败处理。它不负责命令/skills 的业务语义。

### 3. `track-workflow-state`

`track-workflow-state` 负责当前系统如何理解“项目现在处于什么状态”，以及阶段如何声明、识别、流转和回退。

应放入该 capability 的内容：
- situations
- phases
- presets
- 返工规则
- 状态展示时必须反映的 workflow truth

### 4. `manage-task-lifecycle`

`manage-task-lifecycle` 负责任务从草案到敲定、实施、确认、收尾、归档的生命周期闭环，以及任务编号、任务分支、归档目录与冲突保护规则。

### 5. `manage-project-memory`

`manage-project-memory` 负责 `aide-memory/` 目录真值、memory 文档集完整性、模板来源、核心文档同步与访问边界。

### 6. `provide-workflow-guidance`

`provide-workflow-guidance` 负责命令入口与 skills 的职责划分、总工程师 Agent 模型、阶段性 skill 加载规则以及命名收口约束。

## Baseline-to-Capability Mapping

| 审验问题 | 主要 capability | 次要 capability |
|---|---|---|
| situations 未完整落地 | `track-workflow-state` | `cli` |
| presets 定义错误、缺少入口和展示 | `track-workflow-state` | `cli` / `provide-workflow-guidance` |
| `aide hi` 未反映草案残留、常驻分支检查不足 | `cli` | `track-workflow-state` / `manage-task-lifecycle` |
| `build-task` / `finish` / `rework` 闭环不足 | `provide-workflow-guidance` | `manage-task-lifecycle` / `track-workflow-state` |
| `make-memory` / `load-memory` 只有定义层、缺少成果层 | `manage-project-memory` | `provide-workflow-guidance` |
| `task-now.md`、`task-now/`、归档与分支映射闭环 | `manage-task-lifecycle` | `manage-project-memory` |
| `config.toml` / `branches.json` / `branches.md` 访问边界不足 | `manage-project-memory` | `cli` |
| `aide decide` / `aide env` / 旧 flow / `task-parser` 残留 | `provide-workflow-guidance` | `track-workflow-state` / `cli` |
| 插件与宿主同步规则 | `plugin` | `provide-workflow-guidance` |

## Reconciliation Strategy

### 1. 以 current truth 重写，而不是复制 archive

archive change 只能作为参考来源，不能直接视为 current truth。所有 requirement 需要按照当前基准与审验结论重新表述，避免把历史阶段的过时语义带回主规范。

### 2. 旧体系按“待废弃/待清理约束”进入规范

`aide decide`、`aide env`、旧 `.aide/` 路径、旧 step 级 flow、`task-parser` 等内容不再作为正向能力描述，而是通过 requirement 约束 current truth 应如何收口和替代。

### 3. CLI 与 workflow 语义分层表达

- `cli` 描述命令可见行为
- `track-workflow-state` / `manage-task-lifecycle` 描述命令所依赖的 domain truth
- `provide-workflow-guidance` 描述命令与 skill 的提示词/角色契约

这样可以避免在 `cli` spec 中重复定义所有 workflow 细节。

## Risks / Trade-offs

### Trade-off 1：新增 capability 数量较多

优点：边界清晰，后续变更更容易定位。
缺点：一次 proposal 要维护多个 delta 文件，初次编写成本更高。

决策：接受这个成本，因为当前问题本质上就是 current spec 边界过粗导致的表达不足。

### Trade-off 2：部分 requirement 看似可放入 `cli`

例如 `aide hi` 的状态展示既是 CLI 表面行为，也是 workflow state 的投影。

决策：CLI 只描述“应展示什么”，domain spec 描述“系统如何判定这些状态”。

### Trade-off 3：是否单独新增 `agent-process-docs` capability

`AGENT.md` 与 `aide-process-overview.md` 很重要，但其内容与 commands/skills/角色契约深度耦合。

决策：先并入 `provide-workflow-guidance`，避免 capability 过度拆分。如果后续文档语义继续增长，再独立拆分。

## Implementation Orchestration

后续 apply 阶段应把本提案拆成可独立推进的 capability / 模块子任务，由主 Agent 统一调度。

- 主 Agent：担任总工程师，负责任务拆分、顺序安排、并行机会识别、子代理提示词约束、结果审阅与跨模块决策。
- 子代理或等效执行单元：负责各自边界内的代码修复、测试验证、文档同步与结果回传。
- 交付节奏：每完成一组子任务，主 Agent 都需要先汇总与复核，再决定是否进入下一组任务，避免失控并行。
- 兜底约束：若宿主不支持真正的子代理，也要在执行过程中显式按分工边界推进，不能退化为无阶段、无分工、无审阅的单体式实施。

## Open Questions

当前不需要额外用户澄清即可起草 proposal。以下问题留到实现阶段细化：

- `finish` 与 `archive` 的具体边界应由 CLI 还是 workflow manager 主导
- `branches.md` 的“不可读取”约束最终采用程序隔离、提示词约束还是两者结合
- 是否需要在 `plugin` capability 中单独表达 `.claude` 与 `.agents/skills` 的覆盖策略差异
