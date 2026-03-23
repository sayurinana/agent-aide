# /aide:load-memory 命令设计文档

## 一、概述

`/aide:load-memory` 是项目记忆载入入口，用于在执行任务前或解释当前状态前，按需建立项目上下文。

## 二、职责

### 2.1 本命令负责

- 要求先理解 `aide-memory/aide-process-overview.md`
- 要求先理解 `aide-memory/AGENT.md`
- 检查 `aide-memory/memory/` 是否存在可用内容
- 指导学习 `load-memory` skill
- 指导按需载入 memory，而不是一次性全量读完

### 2.2 本命令不负责

- 代替 `/aide:make-memory` 创建缺失的 memory
- 代替具体任务阶段的实施

## 三、执行流程

1. 读取流程总览与 Agent 说明
2. 检查 `aide-memory/memory/` 是否可用
3. 学习 `load-memory` skill
4. 先载入 `overview.md`
5. 再按当前任务选择 `structure/`、`concepts/`、`diagram/` 文档
6. 汇报已建立的认知范围

## 四、失败处理

如果 memory 不存在或明显不完整：
- 明确告知用户
- 建议先执行 `/aide:make-memory`

## 五、相关文档

- [执行文件](../../commands/load-memory.md)
- [流程总览](../../../aide-memory/aide-process-overview.md)
- [Agent 说明](../../../aide-memory/AGENT.md)
