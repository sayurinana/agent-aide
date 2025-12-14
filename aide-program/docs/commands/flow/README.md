# aide flow 详细设计（实现交接包）

本目录为 `aide flow` 子命令的**详细设计**。目标是让接手开发者在不阅读额外上下文的情况下，能够依据本文档集完成实现、联调与验证。

实现位置：
- 核心实现：`aide-program/aide/flow/`
- CLI 入口：`aide-program/aide/main.py` 的 `aide flow ...` 子命令树

上游/关联文档：
- 概览设计：[`aide-program/docs/commands/flow.md`](../flow.md)
- 数据格式规范（状态文件、提交信息）：[`aide-program/docs/formats/data.md`](../../formats/data.md)
- 配置格式规范（flow.phases）：[`aide-program/docs/formats/config.md`](../../formats/config.md)
- 插件侧调用契约：[`/aide:prep`](../../../../aide-marketplace/aide-plugin/docs/commands/prep.md)、[`/aide:exec`](../../../../aide-marketplace/aide-plugin/docs/commands/exec.md)

## 一、范围与目标

### 1.1 目标

- 以“静默即成功”为输出原则，提供统一的**进度追踪**与**Git 自动提交**能力
- 将任务执行过程结构化为“环节（phase）+ 步骤（step）+ 历史（history）”
- 对环节跳转进行校验，降低流程跳跃与遗漏
- 在特定环节提供可插拔的行为（Hooks），例如 PlantUML 校验/生成与 CHANGELOG 校验

### 1.2 非目标

- 不做任务分析/方案选择（这是 plugin 的职责）
- 不实现业务功能
- 不接管项目的 Git 工作流策略（仅提供最小、可配置的自动化提交）

## 二、关键约定（必须先统一）

1. **环节来源**：以 `flow.phases` 为准（见 `aide-program/docs/formats/config.md`），默认值覆盖 prep+exec 全流程。
2. **.aide 默认被 gitignore**：状态文件属于本地数据；Git 提交用于记录项目文件变化，状态历史中的 `git_commit` 仅做关联。
3. **“无变更可提交”不是错误**：当工作区无可提交变更时，`aide flow` 仍应完成状态记录，但 `git_commit` 为空。
4. **钩子顺序影响提交内容**：会产生/修改文件的钩子必须在 `git add/commit` 之前执行，才能进入同一次提交。
5. **失败边界**：校验失败/关键钩子失败应阻止状态推进；Git 失败的处理需要在 `git.md` 中按“必须/可选”策略定义。

## 三、文档索引（按实现模块拆分）

- CLI 与输出：[`aide-program/docs/commands/flow/cli.md`](cli.md)
- 状态与落盘：[`aide-program/docs/commands/flow/state-and-storage.md`](state-and-storage.md)
- 流程校验：[`aide-program/docs/commands/flow/validation.md`](validation.md)
- Git 集成：[`aide-program/docs/commands/flow/git.md`](git.md)
- Hooks 机制：[`aide-program/docs/commands/flow/hooks.md`](hooks.md)
- 验证清单：[`aide-program/docs/commands/flow/verification.md`](verification.md)

## 四、推荐实现模块划分（仅文件/职责约定）

实现位于 `aide-program/aide/flow/`，按职责拆分为：

- `tracker`：编排一次 flow 操作（校验 → hooks → git → 落盘 → 输出）
- `validator`：环节/动作校验（基于 phases 列表）
- `git`：Git 操作封装（add/commit/status/判定“无变更”）
- `hooks`：内置 hook 与 hook 调度
- `types`：状态结构（与 `aide-program/docs/formats/data.md` 一致）

> 注：本文档只约定职责与接口，不提供实现代码。

## 五、实现任务拆分（建议顺序）

1. 完成状态结构与读写：按 `state-and-storage.md` 约定实现原子写入与锁
2. 完成校验器：按 `validation.md` 对 start/next-part/back-part 等动作校验
3. 完成 Git 封装：按 `git.md` 处理 repo 探测、提交信息、无变更/失败策略
4. 完成 Hooks：按 `hooks.md` 实现 PlantUML/CHANGELOG 等内置钩子
5. 完成 CLI 路由：在 `aide-program/aide/main.py` 增加 `aide flow ...` 子命令树
6. 逐条对照 `verification.md` 做验证（至少覆盖“无变更”“非 git 仓库”“校验失败”）

## 六、风险与待定项（需要开发前确认）

- “每次 flow 操作都提交”会产生大量提交：是否需要通过配置提供降噪策略（如仅在 next-part 时提交）
- PlantUML/PNG 是否应纳入提交：默认建议纳入，但需要与团队仓库习惯一致
- CHANGELOG 校验规则的严格程度：硬失败还是警告（建议可配置）
