# 规范：核心子过程 Skills

## ADDED: skill: build-task

重命名自 task-parser，职责调整。

核心职责：
- 指导如何构建符合 aide 体系规范的任务
- 输出 information.md、design.md、todo.md、task-summary.md
- 在 todo.md 中根据任务特性确定适用的阶段流程
- 在 design.md 中标记是否需要图解
- 引导用户对任务内容进行打磨完善

关键变更：
- 任务解析的具体方法由用户指定的解析指导文档决定
- aide 程序在此阶段动态输出解析指导文档的绝对路径

## ADDED: skill: impl-verify

- 按 todo.md 中的任务点逐一实施
- 每完成一个任务点立即进行审验
- 审验通过则标记完成，更新 todo.md 和 task-summary.md
- 审验未通过则在当前阶段内修复
- 发现需求偏差时可发起返工流程

## ADDED: skill: confirm

- 向用户展示成果和变更摘要
- 收集用户反馈
- 不符合预期时触发返工决策

## ADDED: skill: finish

- 使用 aide 程序完成任务归档
- 将任务目录从 tasks/ 移至 archived-tasks/
- 合并任务分支回常驻分支
- 同步更新 memory 全局信息
