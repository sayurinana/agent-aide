# Change: 修复 aide 基线实现缺口

## Why

`task-optimized.md` 已经把 aide 体系的 current truth 讲清楚，但当前仓库仍有一批关键行为没有真正实现或没有完全对齐，主要集中在 **aide 程序、commands、skills** 三类源实现：preset/loop/返工追溯、`aide hi/go/bye/flow` 关键路径、finish/archive 生命周期闭环、解析指导路径、阶段 skill 契约，以及旧体系术语残留等。

此前变更范围把项目内已经生成的运行期数据也一并纳入了修复目标，容易把“实现缺口”和“本地数据状态”混在一起。用户现已明确：本项目中的 `aide-memory/`、宿主分发副本等 **由 aide 产生或同步出的运行/分发产物不属于本次修复对象**。因此，本 change 改为聚焦 **aide 程序 + commands + skills 的实现闭环**，而不是补齐当前仓库里已生成的数据实例。如果需要验证 aide 程序在真实工作目录中的运行效果，应统一在 `/repo/test-aide` 下进行；当前 `agent-aide` 仓库尚未作为 aide 工作目录使用，因此不以本目录下的 aide 运行结果作为验收依据。

## What Changes

- 收紧 `cli` capability 的验收标准，明确 `aide hi/go/bye/flow`、绝对路径、UTC+8 时间口径与 PlantUML 状态反馈的最终行为
- 收紧 `track-workflow-state` capability 的验收标准，明确 preset 识别/展示、loop 可见性与返工原因追溯
- 收紧 `manage-task-lifecycle` capability 的验收标准，明确 finish、archive、暂停离场与正式结束之间的边界
- 收紧 `provide-workflow-guidance` capability 的验收标准，明确 `build-task`、`make-graphics`、`rework`、`finish` 等 skills 的当前契约，并清理旧体系术语残留
- 收紧 `manage-project-memory` capability 的验收标准，但范围仅限于 `make-memory` / `load-memory`、模板来源、缺失提示与访问边界；**不要求为当前仓库补齐或维护已生成的 `aide-memory/` 数据实例**
- 如涉及 `init` / 分发逻辑，只修正 `aide-plugin` 源与程序行为，不把当前仓库中的 `.claude/`、`.agents/` 等已生成副本作为单独修复对象
- 把实现工作拆成四个可验证批次：flow 内核、CLI 与任务闭环、commands/skills 源实现、分发逻辑与总体验证

## Implementation Roles and Constraints

本提案在后续 `/openspec:apply` 实施时，SHALL 采用与 `task-optimized.50adab8.md` 一致的“精简 command 入口 + 总工程师统筹 + 专家执行单元”模式，而不是由单一 Agent 直接包办全部修复。

- **主 Agent / 总工程师**：负责使用 command 入口判断当前状态、拆分批次、指派专家执行单元、审阅结果、控制批次切换与最终验收。
- **Commands 角色**：`hi`、`go`、`load-memory`、`make-memory` 等 command 作为入口指引，只负责说明当前该进入什么工作路径、该学习哪些 skills、该调用哪些程序入口。
- **专家执行单元 / 子代理**：按批次分别承担 flow 内核、CLI/生命周期、commands/skills 源实现、分发/验证等边界内工作；执行前只学习本批次相关的 skill 或实现上下文，不在单轮中吞下全部任务。
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
  - `aide-plugin/commands/*`
  - `aide-plugin/skills/*`
  - 相关源模板、说明文档与测试夹具
- Explicitly out of scope:
  - 当前仓库里已经生成的 `aide-memory/` 运行数据
  - 当前仓库里的 `.claude/`、`.agents/` 等宿主分发副本现状
  - 在当前 `agent-aide` 目录直接把 aide 程序运行效果作为验收对象
- Validation:
  - `openspec validate fix-aide-baseline-implementation-gaps --strict --no-interactive`
  - Rust 单测 / CLI 集成测试
  - 基于 `/repo/test-aide` 的 `hi/go/bye/flow/verify/confirm/archive` 关键命令冒烟验证
  - 基于临时夹具与 `/repo/test-aide` 的 `init` / `make-memory` / `load-memory` 行为验证
