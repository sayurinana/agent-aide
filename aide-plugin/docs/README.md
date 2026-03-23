# Aide Plugin 设计文档

## 一、概述

`aide-plugin` 是 Aide 体系在 Claude Code 中的入口层，负责把“现在该做什么”与“具体怎么做”拆开。

- `command`：入口导航，只给出当前情境下的起点、约束和下一步
- `skill`：方法手册，说明具体的执行方式
- `aide` 程序：提供状态、分支、阶段、任务管理等确定性能力
- `aide-memory/`：提供项目长期记忆和任务上下文

新体系强调简短入口、按需学习、分工明确。

## 二、组件关系

```text
Commands
  make-memory / load-memory / hi / go / bye
        |
        v
Skills
  基础: make-memory / load-memory
  固定阶段: build-task / impl-verify / confirm / finish
  可选阶段: make-graphics / integration / review / docs-update / rework
  参考: aide / plantuml
        |
        v
aide 程序 + aide-memory/
```

其中：
- command 决定入口
- skill 决定方法
- 程序与 memory 提供真实状态和长期上下文

## 三、Commands 索引

| Command | 设计文档 | 执行文件 | 职责 |
|---------|----------|----------|------|
| `/aide:make-memory` | [commands/make-memory.md](commands/make-memory.md) | [../commands/make-memory.md](../commands/make-memory.md) | 生成或更新项目 memory |
| `/aide:load-memory` | [commands/load-memory.md](commands/load-memory.md) | [../commands/load-memory.md](../commands/load-memory.md) | 按需载入项目 memory |
| `/aide:hi` | [commands/hi.md](commands/hi.md) | [../commands/hi.md](../commands/hi.md) | 查看当前状态并给出建议 |
| `/aide:go` | [commands/go.md](commands/go.md) | [../commands/go.md](../commands/go.md) | 进入任务分支并接续当前阶段 |
| `/aide:bye` | [commands/bye.md](commands/bye.md) | [../commands/bye.md](../commands/bye.md) | 收尾并离场 |

> 说明：仓库中旧版 command 文档目前仍保留作迁移参考，但新入口以本表为准。

## 四、入口设计原则

### 4.1 command 保持精简

command 只负责：
- 判断当前属于哪类情境
- 指出需要先读哪些总览文档
- 指出需要学习哪个 skill
- 指出需要调用哪些 `aide` 子命令
- 指出预期行为和汇报方式

command 不负责完整展开所有方法细节。

### 4.2 先总览，后技能，最后执行

在新体系中，进入任何 command 前都默认优先理解：
- `aide-memory/aide-process-overview.md`
- `aide-memory/AGENT.md`

之后再进入具体 skill 和执行动作。

### 4.3 真实状态由程序给出

涉及任务状态、当前分支、阶段进度、待确认或待归档状态时，以 `aide` 程序输出为准，不依赖对话记忆猜测。

## 五、典型路径

### 5.1 初次进入项目

```text
/aide:hi
  -> 若缺少 memory，则 /aide:make-memory
  -> 若已有 memory，则 /aide:load-memory
```

### 5.2 接续任务

```text
/aide:hi
  -> /aide:load-memory
  -> /aide:go
  -> 当前阶段对应的 skill
```

### 5.3 结束本轮工作

```text
/aide:hi
  -> 必要时 verify / confirm / archive
  -> /aide:bye
```

完整阶段说明见：
- [aide-memory/aide-process-overview.md](../../aide-memory/aide-process-overview.md)
- [aide-memory/AGENT.md](../../aide-memory/AGENT.md)

## 六、Skills 视角

当前 command 体系依赖以下 skill 分类：

- 基础：`make-memory`、`load-memory`
- 固定阶段：`build-task`、`impl-verify`、`confirm`、`finish`
- 可选阶段：`make-graphics`、`integration`、`review`、`docs-update`、`rework`
- 参考：`aide`、`plantuml`

其中 `aide` 与 `plantuml` 属于可被多个流程复用的参考技能。

## 七、修改指南

### 7.1 修改 command

1. 先读对应设计文档
2. 再改执行文件
3. 如职责或入口发生变化，同步更新本 README
4. 如影响全局流程，同步更新 `aide-memory/aide-process-overview.md`

### 7.2 修改全局规则

涉及角色边界或协作方式变化时：

1. 更新 `aide-memory/AGENT.md`
2. 更新 `aide-memory/aide-process-overview.md`
3. 再回查各 command 是否仍然成立

## 八、相关文档

- [流程总览](../../aide-memory/aide-process-overview.md)
- [Agent 说明](../../aide-memory/AGENT.md)
- [aide skill](../skills/aide/SKILL.md)
- [plantuml skill](../skills/plantuml/SKILL.md)

## 九、版本信息

- 当前版本：3.0.0
- 更新日期：2026-03-23
- 主要变更：将 commands 重写为入口导航，围绕 `aide-memory/`、`hi/go/bye` 和阶段 skills 组织新流程
