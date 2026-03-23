# /aide:go 命令设计文档

## 一、概述

`/aide:go` 是任务接续入口。它负责基于 `aide hi` 的状态判断和已载入的 memory，进入正确的任务分支并恢复当前阶段上下文。

## 二、职责

### 2.1 本命令负责

- 要求先理解 `aide-memory/aide-process-overview.md`
- 要求先理解 `aide-memory/AGENT.md`
- 要求先完成必要的 memory 载入
- 指导学习 `aide` skill
- 指导执行 `aide hi` 与 `aide go`
- 指导恢复任务摘要、todo 进度和阶段信息
- 指导路由到当前阶段对应的 skill

### 2.2 本命令不负责

- 代替阶段 skill 的具体实施方法
- 在任务不明确时擅自选择错误任务

## 三、执行流程

1. 读取流程总览与 Agent 说明
2. 检查并载入 memory
3. 执行 `aide hi`
4. 决定 `aide go` 的调用方式
5. 进入任务分支
6. 恢复任务文档与阶段上下文
7. 路由到对应 skill 继续推进

## 四、阶段路由

接续后根据阶段流程进入以下 skill 之一：
- `build-task`
- `make-graphics`
- `impl-verify`
- `integration`
- `review`
- `docs-update`
- `confirm`
- `finish`

返工场景由 `rework` skill 处理。

## 五、相关文档

- [执行文件](../../commands/go.md)
- [流程总览](../../../aide-memory/aide-process-overview.md)
- [Agent 说明](../../../aide-memory/AGENT.md)
- [aide skill](../../skills/aide/SKILL.md)
