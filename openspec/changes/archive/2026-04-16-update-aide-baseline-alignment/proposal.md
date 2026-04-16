# Change: 对齐 aide 当前实现与 50adab8 基准

## Why

当前仓库已经落地了新体系的大量主体结构与核心骨架，但根据 `/repo/agent-aide/report-summary.md` 与分段审验报告，现有实现仍未完整达到 `task-optimized.50adab8.md` 所描述的全部需求。

当前 `openspec/specs/` 只有 `cli` 与 `plugin` 两个 current spec，覆盖重点偏向初始化、同步与插件分发，尚未把新体系最核心的工作流语义、任务生命周期、memory 结构、commands/skills 契约与总工程师角色约束沉淀为 current truth。与此同时，部分能力虽然曾在 archive change 中定义，但没有完整回流到当前主规范，导致 current spec、实现、文档与审验基准之间出现持续漂移。

本变更用于把 50adab8 基准中的核心要求正式收敛为现行规范，并为后续完整修复提供一致、可验证的 OpenSpec 基线。

## What Changes

- 修改 `cli` capability，补齐 `aide hi/go/bye/verify/confirm/archive/flow` 的用户可见行为契约，以及配置、路径、时间、输出一致性要求
- 修改 `plugin` capability，明确插件仓库同步、Claude/Codex 双宿主分发与同步边界
- 新增 `track-workflow-state` capability，定义 situations、phases、presets、返工与状态展示规则
- 新增 `manage-task-lifecycle` capability，定义任务草案、敲定、实施、归档、任务分支与收尾闭环
- 新增 `manage-project-memory` capability，定义 `aide-memory/` 目录结构、memory 产物、模板来源与访问边界
- 新增 `provide-workflow-guidance` capability，定义 commands/skills 的职责边界、按阶段加载规则、总工程师协作模型与命名收口要求

## Implementation Constraint

- 本提案在后续 `/openspec:apply` 与实际修复阶段，必须沿用 `task-optimized.md` 中约定的“主 Agent 作为总监 / 总工程师、子代理作为专家执行单元”的实施方式。
- 主 Agent 负责按 capability、阶段或主题拆分任务，编写与分派子代理提示，审阅子代理结果，并汇总最终结论与交付物。
- 子代理负责在各自边界内完成实现、验证与记录；不得把整套修复退化为单一 Agent 在单上下文中直接串行硬做全部事务。
- 当宿主支持并行子代理时，应优先并行推进相互独立的子任务；当宿主不支持真正的子代理时，也必须在单上下文中显式保持“总工程师统筹 / 专家分工执行”的职责边界。

## Impact

- Affected specs:
  - `cli`
  - `plugin`
  - `track-workflow-state` (new)
  - `manage-task-lifecycle` (new)
  - `manage-project-memory` (new)
  - `provide-workflow-guidance` (new)
- Affected code:
  - `aide/src/cli/*.rs`
  - `aide/src/flow/*.rs`
  - `aide/src/core/*.rs`
  - `aide-memory/*`
  - `templates/*`
  - `aide-plugin/commands/*`
  - `aide-plugin/skills/*`
  - `.claude/commands/*`
  - `.claude/skills/*`
  - `docs/reference/*`
- Behavioral impact:
  - 明确新体系 current truth，修正当前实现与基准不一致的行为
  - 结束 archive 提案与 current specs 脱节的状态
  - 为后续“总工程师 + 子代理”方式实施完整修复提供统一约束
