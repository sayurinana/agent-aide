# /aide:make-memory 命令设计文档

## 一、概述

`/aide:make-memory` 是项目记忆生成入口，用于引导总工程师代理调度合适的执行者，基于 `make-memory` skill 为项目建立或更新 `aide-memory/memory/` 文档集。

## 二、职责

### 2.1 本命令负责

- 要求先理解 `aide-memory/aide-process-overview.md`
- 要求先理解 `aide-memory/AGENT.md`
- 明确总工程师与 memory 专家之间的职责边界
- 指导创建子代理或等效的受限执行上下文
- 指导学习 `make-memory` skill
- 指导生成并验收 memory 文档

### 2.2 本命令不负责

- 直接展开 `make-memory` skill 的全部细节
- 直接替代后续任务实施流程
- 直接决定具体业务方案

## 三、执行流程

1. 读取 `aide-memory/aide-process-overview.md`
2. 读取 `aide-memory/AGENT.md`
3. 划定 memory 专家的工作边界
4. 学习 `make-memory` skill
5. 生成或更新 `aide-memory/memory/` 下文档
6. 由总工程师做覆盖性验收并向用户汇报

## 四、产出物

- `aide-memory/memory/overview.md`
- `aide-memory/memory/structure/`
- `aide-memory/memory/concepts/`
- `aide-memory/memory/diagram/`

## 五、相关文档

- [执行文件](../../commands/make-memory.md)
- [流程总览](../../../aide-memory/aide-process-overview.md)
- [Agent 说明](../../../aide-memory/AGENT.md)
