---
name: build-task
description: 任务构建与打磨指南。在新建正式任务或当前阶段为 build-task 时使用，用于读取任务解析指导文档，澄清需求，产出 `information.md`、`design.md`、`todo.md`、`task-summary.md` 与 `flow-graphics/` 目录，并为后续 `aide verify` / `aide confirm` 做准备。
---

# build-task 指南

## 目标

把原始需求打磨成可验证、可确认、可实施的任务草案。

解析细节以用户指定或 aide 程序给出的任务解析指导文档为准；本 skill 负责约束产物、阶段选择和验收口径，不重复定义解析方法本身。

## 输入

- 用户当前需求、对话记录或 `task-now.md`
- 任务解析指导文档绝对路径
- 已载入的项目 memory、相关代码上下文

如果 aide 程序已经输出了解析指导文档路径，优先使用该绝对路径；只有在没有明确路径时，才回退到 `aide-memory/templates/任务解析指导.md`。

## 必须产物

在 `aide-memory/tasks/task-now/` 下准备好以下内容：

- `information.md`
- `design.md`
- `todo.md`
- `task-summary.md`
- `flow-graphics/` 目录

这些文件需要满足 `aide verify` 的基本校验要求。

## 工作流程

### 1. 先理解，再落盘

1. 阅读任务解析指导文档，按该文档的方法理解和重组需求。
2. 结合项目 memory、现有代码和用户反馈，补齐关键上下文。
3. 遇到关键约束缺失、验收标准不清或依赖不明确时，先和用户确认，不要伪造结论。

### 2. 产出任务文档

按下述约束分别写入 4 个文件。

#### information.md

- 必须是合法 Markdown，且第一行是一级标题
- 正文至少说明：
  - 任务背景或动机
  - 目标与范围
  - 具体要求
  - 约束条件
  - 验收标准
- 未解决的问题可以单列，但不要把关键前提留空不说明

#### design.md

- 必须是合法 Markdown，且第一行是一级标题
- 必须包含图解标记，二选一：
  - `<!-- GRAPHICS: required -->`
  - `<!-- GRAPHICS: skip: 原因 -->`
- 正文至少说明：
  - 实现思路
  - 涉及模块或文件范围
  - 风险、边界和取舍
- 如果需要图解，应明确图解要表达什么；具体绘制工作交给 `make-graphics`

#### todo.md

- 必须是合法 Markdown，且第一行是一级标题
- 必须包含阶段声明，格式示例：

```markdown
<!-- PHASES: build-task, impl-verify:loop, confirm, finish -->
```

- 阶段声明必须以 `build-task` 开始
- 任务点必须使用复选框格式，例如：

```markdown
- [ ] 完成核心接口实现
- [ ] 补齐回归测试
```

- 每个任务点都应可验证、可完成、可在阶段内独立推进

#### task-summary.md

- 必须是合法 Markdown，且第一行是一级标题
- 一级标题本身就是任务摘要标题，应简短明确
- 正文尽量控制在 10 行以内、每行 30 字以内，便于 `aide hi` / `aide go` 展示
- 只保留范围、当前结果、关键提醒，不要写成长文

#### flow-graphics/

- 目录必须存在
- 即使 `design.md` 标记为无需图解，也应保留空目录
- 若标记为 `required`，后续阶段需要在此目录中补齐可编译的 PlantUML 文件

## 阶段选择规则

固定阶段始终包含：

- `build-task`
- `confirm`
- `finish`

实施阶段默认使用 `impl-verify`，通常建议启用循环模式：

```markdown
<!-- PHASES: build-task, impl-verify:loop, confirm, finish -->
```

按任务特性决定是否插入可选阶段：

- 需要流程图、结构图或交互图时，加入 `make-graphics`
- 需要跨模块联调、环境联测或整体串联时，加入 `integration`
- 需要专门评审质量、安全、架构或方案时，加入 `review`
- 需要同步项目文档、操作手册或 memory 时，加入 `docs-update`

不要为了“看起来完整”而加入无意义阶段。

## 输出原则

- 解析方法来自外部指导文档，本 skill 只负责把结果变成任务文档
- 文档必须面向后续实施，而不是停留在口头需求转述
- 阶段流程、图解标记、复选框任务点必须真实反映任务特性
- 在你认为已经可以进入下一步时，应明确告诉用户：
  - 补齐或澄清了哪些信息
  - 选用了哪些阶段
  - 是否可以执行 `aide verify` 与 `aide confirm`

## 完成条件

满足以下条件后，`build-task` 才算完成：

- 4 个任务文档和 `flow-graphics/` 目录已准备好
- `design.md` 已包含合法图解标记
- `todo.md` 已包含合法阶段声明和复选框任务点
- 可选阶段的启用理由已经在任务文档中体现
- 当前草案已经达到可执行 `aide verify` 的质量
