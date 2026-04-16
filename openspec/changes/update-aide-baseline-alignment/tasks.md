# 实施任务清单

## 1. 能力边界与当前规范收口

- [x] 1.1 复核 `cli` 与 `plugin` 当前 requirement，确定保留边界与需要替换的旧口径
- [x] 1.2 建立 50adab8 基准条目到 capability 的映射，确保每类缺口都有唯一承载位置
- [x] 1.3 识别 archive change 中可复用的 requirement 语义，并重写为 current truth

## 2. CLI 与插件规范补齐

- [x] 2.1 扩展 `cli` spec，补齐 `aide hi/go/bye` 的可见行为、输出和错误处理要求
- [x] 2.2 扩展 `cli` spec，补齐 `aide verify/confirm/archive/flow` 的命令行为与一致性要求
- [x] 2.3 修订 `plugin` spec，明确插件仓库同步、项目级分发与 Claude/Codex 双宿主部署边界

## 3. 新增 workflow domain capabilities

- [x] 3.1 新增 `track-workflow-state` spec，定义 situations、phases、presets 与返工规则
- [x] 3.2 新增 `manage-task-lifecycle` spec，定义草案、敲定、归档、任务分支与收尾闭环
- [x] 3.3 新增 `manage-project-memory` spec，定义 `aide-memory` 结构、memory 产物、模板来源与访问边界
- [x] 3.4 新增 `provide-workflow-guidance` spec，定义 commands/skills 契约、总工程师角色与命名收口规则

## 4. 规范一致性与清理要求

- [x] 4.1 在各 capability 中明确旧体系的收口约束，包括 `aide decide`、`aide env`、旧 flow 与 `task-parser`
- [x] 4.2 统一 `build-task`、图解标记、preset、resident branch、task-now 等关键术语口径
- [x] 4.3 检查 capability 之间是否存在 requirement 重复、职责交叉或缺口

## 5. 验证与实施准备

- [x] 5.1 运行 `openspec validate update-aide-baseline-alignment --strict --no-interactive`
- [x] 5.2 修正所有校验错误，确保 proposal、design、tasks 与 spec deltas 一致
- [x] 5.3 在 apply 阶段由主 Agent 作为总监 / 总工程师先拆分实施批次与子任务，而不是直接单体式开工
- [x] 5.4 按 capability、阶段或主题创建并管理子代理 / 专家执行单元，交由其分段完成实现、验证与记录
- [x] 5.5 主 Agent 对每轮子任务结果执行审阅、汇总与跨模块一致性检查，再决定下一轮实施
- [x] 5.6 当宿主支持并行子代理时并行推进独立任务；当宿主不支持时，在单上下文中显式保持同等职责边界与分工纪律
- [x] 5.7 全部修复完成后，由主 Agent 统一验证完成度并确认实现结果满足本提案与 50adab8 基准
