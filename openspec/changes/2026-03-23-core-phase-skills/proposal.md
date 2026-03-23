# Change: 核心子过程 Skills

## Why

当前插件体系已经把正式任务流程收敛为固定阶段，但仓库里仍缺少这些固定阶段对应的 skill 实现，导致命令文档与实际技能目录不一致，任务方法论也无法真正落地。

需要补齐所有正式任务都必须经过的 4 个核心阶段 skill，并把旧的 `task-parser` 迁移为新的 `build-task` 阶段能力。

## What Changes

- 将 `task-parser` 重命名并重写为 `build-task` skill
- 新增 `impl-verify` skill，定义逐项实施与即时审验方法
- 新增 `confirm` skill，定义成果展示与反馈收集流程
- 新增 `finish` skill，定义任务收尾、归档与长期信息同步流程
- 修正活跃命令文档中仍指向旧 skill 名称的引用

## Impact

- Affected specs: `core-phase-skills`
- Affected code:
  - `aide-plugin/skills/build-task/SKILL.md`
  - `aide-plugin/skills/impl-verify/SKILL.md`
  - `aide-plugin/skills/confirm/SKILL.md`
  - `aide-plugin/skills/finish/SKILL.md`
  - `aide-plugin/commands/run.md`
- Dependency context: 依赖前序提案中已引入的任务文档格式、阶段流程与任务管理命令
