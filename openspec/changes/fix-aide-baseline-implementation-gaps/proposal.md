# Change: 修复 aide 基线实现缺口

## Why

`update-aide-baseline-alignment` 已经把大量 current truth 收口到了当前 OpenSpec，但 `/repo/agent-aide/report-summary.md` 仍然确认仓库存在一批尚未闭环的实现缺口：预设定义与展示不完整、返工原因不可追溯、`hi/go/bye/flow` 的关键路径与基准不一致、`finish/archive` 生命周期闭环不足、模板与解析指导链路漂移、旧体系术语残留、memory 成果层缺失，以及插件源与项目副本没有完全同步收口。

这些问题已经不再是“补 current spec”本身，而是需要一个明确的实现型 change，把剩余缺口转化为可执行、可验证、可分批实施的修复范围。没有这一步，后续修复容易再次停留在文档层，或者在多模块修改时失去统一验收标准。

## What Changes

- 收紧 `cli` capability 的验收标准，明确 `aide hi/go/bye/flow`、绝对路径、UTC+8 时间口径与 PlantUML 状态反馈的最终行为
- 收紧 `track-workflow-state` capability 的验收标准，明确 preset 识别/展示、loop 可见性与返工原因追溯
- 收紧 `manage-task-lifecycle` capability 的验收标准，明确 finish、archive、暂停离场与正式结束之间的边界
- 收紧 `manage-project-memory` capability 的验收标准，明确 memory 最小成果集、模板来源、解析指导路径与配置/分支访问边界
- 收紧 `provide-workflow-guidance` capability 的验收标准，明确 build-task / make-graphics / rework / finish 的当前契约，并清理旧体系术语残留
- 收紧 `plugin` capability 的验收标准，确保 `aide-plugin` 源与 `.claude` / `.agents` / Codex 分发副本保持同一套 current truth
- 把实现工作拆成可验证的五个批次：flow 内核、CLI 与任务闭环、配置与模板、插件/skills/文档收口、memory 与总体验证

## Implementation Roles and Constraints

本提案在后续 `/openspec:apply` 实施时，SHALL 采用与 `task-optimized.50adab8.md` 一致的“精简 command 入口 + 总工程师统筹 + 专家执行单元”模式，而不是由单一 Agent 直接包办全部修复。

- **主 Agent / 总工程师**：负责使用 command 入口判断当前状态、拆分批次、指派专家执行单元、审阅结果、控制批次切换与最终验收。
- **Commands 角色**：`hi`、`go`、`load-memory`、`make-memory` 等 command 作为入口指引，只负责说明当前该进入什么工作路径、该学习哪些 skills、该调用哪些程序入口。
- **专家执行单元 / 子代理**：按批次分别承担 flow 内核、CLI/生命周期、配置模板、插件文档、memory/验证等边界内工作；执行前只学习本批次相关的 skill 或实现上下文，不在单轮中吞下全部任务。
- **主 Agent 审阅责任**：每一批完成后，主 Agent 必须先汇总、复核、决定是否返工或进入下一批，不能让并行修改失控扩散。
- **宿主兼容约束**：若宿主支持真正子代理，则应按边界并行推进；若宿主不支持，也必须在单上下文中显式保持上述角色分离与分批纪律。

## Impact

- Affected specs:
  - `cli`
  - `plugin`
  - `track-workflow-state`
  - `manage-task-lifecycle`
  - `manage-project-memory`
  - `provide-workflow-guidance`
- Affected code and assets:
  - `aide/src/flow/*.rs`
  - `aide/src/cli/*.rs`
  - `aide/src/core/*.rs`
  - `aide/tests/cli_integration.rs`
  - `aide-memory/*`
  - `templates/*`
  - `task-now.md`
  - `aide-plugin/commands/*`
  - `aide-plugin/skills/*`
  - `.claude/commands/*`
  - `.claude/skills/*`
  - `docs/*`
- Validation:
  - `openspec validate fix-aide-baseline-implementation-gaps --strict --no-interactive`
  - Rust 单测 / CLI 集成测试
  - 针对 `hi/go/bye/flow/verify/confirm/archive` 的关键冒烟验证
