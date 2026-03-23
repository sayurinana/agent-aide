# /aide:bye 命令设计文档

## 一、概述

`/aide:bye` 是收尾离场入口。它先确认当前状态与是否存在待补的收尾动作，再执行 `aide bye` 完成本轮工作收束。

## 二、职责

### 2.1 本命令负责

- 要求先理解 `aide-memory/aide-process-overview.md`
- 要求先理解 `aide-memory/AGENT.md`
- 指导学习 `aide` skill 中与收尾相关的命令说明
- 指导执行 `aide hi` 识别当前状态
- 指导判断是否需先执行 verify / confirm / archive
- 指导执行 `aide bye`
- 指导给出下次接续建议

### 2.2 本命令不负责

- 代替完整的任务归档流程
- 跳过必要的状态检查直接离场

## 三、执行流程

1. 读取流程总览与 Agent 说明
2. 学习 `aide` skill 的相关命令
3. 执行 `aide hi`
4. 判断是否需要补做收尾动作
5. 执行 `aide bye`
6. 向用户说明结果与下次接续入口

## 四、关键判断点

- 当前是否仍在任务分支
- 当前成果是否已经审验和敲定
- 当前是否只结束本轮会话，还是结束整个任务

## 五、相关文档

- [执行文件](../../commands/bye.md)
- [流程总览](../../../aide-memory/aide-process-overview.md)
- [Agent 说明](../../../aide-memory/AGENT.md)
- [aide skill](../../skills/aide/SKILL.md)
