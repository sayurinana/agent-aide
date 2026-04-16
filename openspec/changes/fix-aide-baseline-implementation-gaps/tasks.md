# 实施任务清单

## 0. 实施方式约束

- [ ] 0.1 由主 Agent 先通过 command 入口确认当前状态与进入路径，再开始对应批次实施
- [ ] 0.2 由主 Agent 将修复工作拆分给专家执行单元，并按边界控制并行范围
- [ ] 0.3 每批完成后先由主 Agent 审阅与汇总，再决定进入下一批或触发返工

## 1. 修复 flow 状态内核

- [ ] 1.1 对齐 `full/standard/lite/docs/research` 的 preset 定义、detect 结果与 `flow status/show` 展示
- [ ] 1.2 为 `flow-status` 增加返工原因追溯，并保持旧状态文件兼容读取/写回
- [ ] 1.3 补齐 `aide flow back/status/show` 的单测与 CLI 集成测试

## 2. 修复核心 CLI 与任务生命周期闭环

- [ ] 2.1 完善 `aide hi` 的 situation 判断，覆盖草案残留、任务并存、其他分支与常驻分支检查
- [ ] 2.2 完善 `aide go` / `aide go -v` 的唯一任务、多任务、无任务与失败路径详细回显
- [ ] 2.3 区分 `aide bye` 的暂停离场与正式结束，补齐 `finish/archive` 的生命周期闭环
- [ ] 2.4 统一 CLI 的绝对路径、UTC+8 时间口径与 PlantUML 变更优先处理反馈

## 3. 收口配置、初始化与模板来源

- [ ] 3.1 清理 `config` / `init` 默认值中的旧阶段、旧命令与过时 fallback 文案
- [ ] 3.2 打通 `task.parse_guide` 与任务解析指导文档的程序输出链路，确保输出绝对路径
- [ ] 3.3 更新 `task-now.md`、模板、`AGENT.md` 与 `aide-process-overview.md`，使其与 current truth 一致

## 4. 收口插件、skills 与活跃文档

- [ ] 4.1 以 `aide-plugin` 为源同步 commands/skills，并收口 `.claude`、`.agents` 与 Codex 分发副本
- [ ] 4.2 修正 `build-task`、`make-graphics`、`rework`、`finish`、`aide` 等 skills/commands 的当前语义
- [ ] 4.3 清理 `aide decide`、`aide env`、旧 flow step 命令、`.aide/` 与 `task-parser` 在活跃文档中的残留

## 5. 补齐 memory 并完成回归验证

- [ ] 5.1 补齐 `overview.md`、`structure/`、`concepts/`、`diagram/` 的最小可用 memory 文档集
- [ ] 5.2 验证 `make-memory` / `load-memory` 对最小成果集的消费链路与提示行为
- [ ] 5.3 运行 `openspec validate fix-aide-baseline-implementation-gaps --strict --no-interactive`
- [ ] 5.4 运行 Rust 单测、CLI 集成测试与关键命令冒烟验证，确认 `report-summary.md` 中的问题全部闭环
