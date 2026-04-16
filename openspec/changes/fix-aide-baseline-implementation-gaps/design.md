# Design: fix-aide-baseline-implementation-gaps

## Context

当前仓库已经具备新体系的主体骨架，但 `task-optimized.md` 中围绕 aide 体系定义的 current truth 仍有一批关键实现缺口集中在三类源实现：

1. `FlowPreset` 只有部分预设对齐基准，其余预设实现、detect、展示与返工追溯不完整；
2. `aide hi/go/bye/flow` 没有完整体现 situations、草案冲突、正式 finish 闭环与详细回显要求；
3. `build-task`、`make-graphics`、`rework`、`finish`、`aide` 等 commands/skills 的当前语义没有完全收口；
4. `config` / `init` / 模板来源 / 解析指导路径仍残留旧阶段、旧命令、旧 fallback 文本与 `task-parser` 命名；
5. `aide init` 的源与分发逻辑、`make-memory` / `load-memory` 的缺失提示与边界行为尚未稳定落地。

与此同时，用户已经明确：当前项目里曾由 aide 生成或同步出的运行期数据（如 `aide-memory/`、宿主分发副本）不应再被当作本次 change 的修复对象。也就是说，本次设计要解决的是**程序、commands、skills 的实现缺口**，而不是为当前仓库补齐一份演示性运行数据。

## Goals / Non-Goals

### Goals

- 让 `task-optimized.md` 中与 aide 程序、commands、skills 相关的 current truth 拥有可直接落地的验收标准
- 先修复 flow 状态内核，再推进 CLI 行为与生命周期闭环，减少跨模块返工
- 统一 commands/skills 源实现、模板来源与旧术语清理，避免“程序修了但入口说明仍是旧体系”
- 让 `make-memory` / `load-memory` 在缺失或占位运行数据场景下给出正确提示与入口
- 通过夹具验证 `init`、CLI 与阶段行为，而不是依赖当前仓库里现成的运行数据

### Non-Goals

- 不恢复或补齐当前仓库中已删除的 `aide-memory/`、`.claude/`、`.agents/` 等运行/分发产物
- 不把当前仓库里的运行数据现状当作 current truth 的验收对象
- 不重新设计 capability 边界；边界沿用现有 six capabilities
- 不引入新的 workflow 阶段或新的宿主分发模型
- 不在本次 change 中扩展与当前缺口无关的新功能

## Decisions

### Decision 1：先修 flow 状态内核，再修 CLI 展现

`preset`、`loop`、`rework reason`、`flow-status` 持久化与 `todo.md` 阶段提取都属于状态内核。只有这些基础 truth 稳定后，`aide flow status/show` 与 `aide hi/go/bye` 的展示和决策才不会反复返工。

具体要求：
- 对齐 `full/standard/lite/docs/research` 的 preset 定义
- 让 `flow-status` 能记录返工原因，并兼容已有状态文件
- 在 `flow status/show` 中展示 preset、loop 与返工历史

### Decision 2：明确区分“暂停离场”与“正式结束”

`aide bye` 只负责安全离场；`finish/archive` 才承担正式结束、归档、分支收束与长期信息同步。这样才能避免“切回常驻分支”被误认为任务已经完成。

具体要求：
- 未完成任务执行 `bye` 时，只能视为暂停
- 已达到 finish 条件的任务，`bye` 必须引导进入正式收尾闭环，而不是静默代替 finish
- `archive` 需要与任务目录、分支映射、任务状态保持一致

### Decision 3：commands / skills / 模板只收口真相源，不修当前仓库运行产物

本次 change 关注的是 `aide-plugin`、程序源码、模板来源和相应测试。当前仓库里已经生成出的 `aide-memory/`、`.claude/`、`.agents/` 等文件，只是运行或分发结果，不应反向成为修复目标。

具体要求：
- `build-task` 优先消费程序输出的解析指导文档绝对路径
- `init` 不再回退到过时的旧体系 fallback 文案
- `aide-plugin` 作为 commands/skills 的真相源；如需验证分发逻辑，用夹具生成目标目录验证

### Decision 4：memory 相关工作只验证命令契约与缺失提示

`make-memory` / `load-memory` 的 current truth 仍然需要实现，但验收重点是：
- 能否生成最小可用成果集
- 在缺失或占位内容时能否正确提示
- 是否遵守配置/分支访问边界

而不是要求当前仓库必须内置一套完整 memory 成果作为本次修复前提。

### Decision 5：提案实施本身仍遵循“command 入口 + 总工程师 + 专家执行单元”

本提案要求的不只是“修哪些内容”，还要求“如何实施这些修复”。实施方式必须与 `task-optimized.50adab8.md` 中 commands 的定位保持一致：command 作为精简入口指引，总工程师负责统筹，专家执行单元负责分边界落地。

具体要求：
- 主 Agent 先通过 `hi` / `go` / `load-memory` 等入口判断状态与进入路径，而不是直接无上下文开工
- 主 Agent 负责把批次拆为可验证子任务，并决定哪些工作可以并行
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

### Batch 3：commands / skills 源实现与模板来源
- 清理旧阶段、旧命令与旧 fallback 文案
- 打通 `task.parse_guide` 绝对路径输出链路
- 更新 `build-task`、`make-graphics`、`rework`、`finish`、`aide` 等源实现语义
- 清理 `aide decide`、`aide env`、旧 flow step 命令、`.aide/` 与 `task-parser` 残留

### Batch 4：分发逻辑与回归验证
- 以 `aide-plugin` 为源验证 `init` / 分发逻辑
- 验证 `make-memory` / `load-memory` 在缺失或占位运行数据下的提示行为
- 运行 OpenSpec、Rust 测试与关键 CLI 冒烟验证

## Risks / Trade-offs

### Risk 1：把运行产物和源实现混淆，导致范围再次膨胀

缓解：在 proposal、tasks、测试策略里显式声明当前仓库运行数据不作为修复对象；需要验证生成行为时统一使用测试夹具。

### Risk 2：flow-status 字段扩展可能影响旧任务数据

缓解：新增字段采用可选字段并保持向后兼容读取；只有在状态更新时才回写新字段。

### Risk 3：commands / skills / init 源与分发逻辑容易再次漂移

缓解：以 `aide-plugin` 作为真相源，优先修源实现与自动化验证，不靠手工维护项目内副本来“看起来正确”。

## Validation Strategy

- `openspec validate fix-aide-baseline-implementation-gaps --strict --no-interactive`
- 针对 `aide flow`、`aide hi/go/bye`、`aide verify/confirm/archive` 的 Rust 测试
- 针对 `build-task`、`make-graphics`、`rework`、`finish`、`aide` 等源实现文档与旧术语的搜索校验
- 基于临时夹具验证 `init` / `make-memory` / `load-memory` 行为，而不是依赖当前仓库运行数据
