# 实施任务清单

## 0. 实施方式约束

- [x] 0.1 由主 Agent 先通过 command 入口确认当前状态与进入路径，再开始对应批次实施
- [x] 0.2 由主 Agent 将修复工作拆分给专家执行单元，并按边界控制并行范围
- [x] 0.3 每批完成后先由主 Agent 审阅与汇总，再决定进入下一批或触发返工
- [x] 0.4 不把当前仓库 `aide-memory/`、`.claude/`、`.agents/` 等已生成运行/分发产物作为修复目标，相关行为以源实现、夹具验证与 `/repo/test-aide` 运行验证为准

## 1. 修复 flow 状态内核

- [x] 1.1 对齐 `full/standard/lite/docs/research` 的 preset 定义、detect 结果与 `flow status/show` 展示
- [x] 1.2 为 `flow-status` 增加返工原因追溯，并保持旧状态文件兼容读取/写回
- [x] 1.3 补齐 `aide flow back/status/show` 的单测与 CLI 集成测试

## 2. 修复核心 CLI 与任务生命周期闭环

- [x] 2.1 完善 `aide hi` 的 situation 判断，覆盖草案残留、任务并存、其他分支与常驻分支检查
- [x] 2.2 完善 `aide go` / `aide go -v` 的唯一任务、多任务、无任务与失败路径详细回显
- [x] 2.3 区分 `aide bye` 的暂停离场与正式结束，补齐 `finish/archive` 的生命周期闭环
- [x] 2.4 统一 CLI 的绝对路径、UTC+8 时间口径与 PlantUML 变更优先处理反馈

## 3. 收口 aide 程序、commands 与 skills 源实现

- [ ] 3.1 清理 `config` / `init` 默认值中的旧阶段、旧命令与过时 fallback 文案
- [ ] 3.2 打通 `task.parse_guide` 与任务解析指导文档的程序输出链路，确保 `build-task` 优先消费绝对路径
- [ ] 3.3 修正 `aide-plugin/commands/*` 与 `aide-plugin/skills/*` 中 `build-task`、`make-graphics`、`rework`、`finish`、`aide` 等当前语义
- [ ] 3.4 清理 `aide decide`、`aide env`、旧 flow step 命令、`.aide/` 与 `task-parser` 在源实现和活跃源文档中的残留

## 4. 收口分发逻辑并完成回归验证

- [ ] 4.1 以 `aide-plugin` 为真相源修正 `aide init` / 分发逻辑或相关测试，不依赖本仓库已生成副本
- [ ] 4.2 验证 `make-memory` / `load-memory` 在缺失或占位 runtime data 下的提示与入口行为
- [ ] 4.3 运行 `openspec validate fix-aide-baseline-implementation-gaps --strict --no-interactive`
- [ ] 4.4 运行 Rust 单测、CLI 集成测试与关键命令冒烟验证；其中 aide 程序运行效果在 `/repo/test-aide` 覆盖 `hi/go/bye/flow/verify/confirm/archive` 等路径，必要时再配合临时夹具验证 `init` 等生成行为
