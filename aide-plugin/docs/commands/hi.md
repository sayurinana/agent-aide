# /aide:hi 命令设计文档

## 一、概述

`/aide:hi` 是状态判断入口。它先通过 `aide hi` 读取项目真实状态，再结合项目 memory 给出下一步行动建议。

## 二、职责

### 2.1 本命令负责

- 要求先理解 `aide-memory/aide-process-overview.md`
- 要求先理解 `aide-memory/AGENT.md`
- 指导学习 `aide` skill 中与状态查看和阶段推进相关的部分
- 指导执行 `aide hi`
- 指导解释输出并判断是否需要载入 memory
- 指导形成下一步建议

### 2.2 本命令不负责

- 直接切换任务分支
- 直接展开某个阶段的实施细节

## 三、执行流程

1. 读取流程总览与 Agent 说明
2. 学习 `aide` skill 的相关命令说明
3. 执行 `aide hi`
4. 必要时执行 `aide hi -v`
5. 判断是否需要 `load-memory`
6. 基于状态与 memory 提出下一步建议

## 四、关键判断点

- 当前分支类型是什么
- 是否存在未归档任务
- 当前任务是否可继续、待确认或待归档
- 当前是否需要额外载入 memory

## 五、相关文档

- [执行文件](../../commands/hi.md)
- [流程总览](../../../aide-memory/aide-process-overview.md)
- [Agent 说明](../../../aide-memory/AGENT.md)
- [aide skill](../../skills/aide/SKILL.md)
