---
name: load-memory
description: 按需载入 `aide-memory/memory/` 文档并建立项目认知。当需要理解项目、接续任务、或 `/aide:load-memory` 要求先读取 `overview.md`、`structure/`、`concepts/`、`diagram/` 时使用。
---

# 项目 Memory 载入指南

本 skill 用于在进入任务前，以最小必要成本建立可靠的项目上下文。

## 目标

1. 基于已有 `aide-memory/memory/` 建立当前任务所需认知。
2. 明确哪些信息已经足够，哪些部分仍需结合代码现状补充确认。
3. 避免“为了保险一次性全读全部 memory”造成上下文浪费。

## 前置准备

1. 先完整阅读：
   - `aide-memory/aide-process-overview.md`
   - `aide-memory/AGENT.md`
2. 学习 `aide` skill，必要时用来确认真实状态。
3. 若当前任务涉及流程图或架构图，再按需学习 `plantuml` skill。

## 载入前检查

优先确认以下路径是否存在且内容有效：

- `aide-memory/memory/overview.md`
- `aide-memory/memory/structure/index.md`
- `aide-memory/memory/concepts/term.md`
- `aide-memory/memory/concepts/arch.md`

如果 memory 缺失、明显过时或内容严重不完整：

- 明确告知用户当前无法完成可靠的 memory 载入
- 建议先执行 `/aide:make-memory`
- 不要假装已经建立了完整项目认知

## 默认载入顺序

### 1. 先读 `overview.md`

目标：

- 了解项目用途
- 了解技术栈
- 了解主要目录和推荐阅读顺序

### 2. 再读 `structure/index.md`

目标：

- 获取完整目录结构
- 确认关键区块名称
- 决定下一步该读哪些 `structure/*.md`

### 3. 按任务选择相关结构文档

根据当前任务、当前阶段或用户提问，挑选最相关的 `structure/*.md`。

选择原则：

- 任务涉及某个模块，就优先读取该模块区块文档
- 涉及跨模块问题，补读依赖区块和入口区块
- 只在需要时扩展阅读范围

### 4. 补读 `concepts/term.md`

当出现项目特有术语、缩写或口头表达时，优先用 `term.md` 对齐语义。

### 5. 补读 `concepts/arch.md`

当需要理解系统边界、模块协作、关键流程或高层设计时，再读取 `arch.md`。

### 6. 按需查看 `diagram/*.puml`

适用于以下场景：

- 仅靠文字难以快速理解流程
- 当前任务涉及复杂状态流转
- `arch.md` 已引用某张图解

## 常见载入模式

### 模式一：首次进入项目

建议顺序：

1. `overview.md`
2. `structure/index.md`
3. `concepts/arch.md`
4. 最核心的 1-3 篇 `structure/*.md`

### 模式二：接续某个具体任务

建议顺序：

1. `overview.md`
2. 与当前任务直接相关的 `structure/*.md`
3. 相关术语和架构文档
4. 必要时补读图解

### 模式三：回答局部问题

建议顺序：

1. `structure/index.md`
2. 命中的区块文档
3. 若出现术语歧义，再读 `term.md`

## 载入后的输出要求

向总工程师或用户汇报时，至少说明：

- 本次实际读取了哪些 memory 文档
- 当前已建立的认知范围
- 哪些部分仍未覆盖
- 哪些结论来自 memory，哪些仍需回到代码核实

## 风险控制

1. **不要全量机械读取**：memory 的价值是“按需建立上下文”，不是一次性灌满。
2. **不要盲信旧 memory**：如果任务高风险、代码近期变化大或 memory 明显陈旧，必须回到代码验证。
3. **不要跳过缺口说明**：已知未知同样要汇报清楚。

## 退化策略

仅当用户明确要求“先快速了解项目”，且当前无法立即补建 memory 时，才允许退化为快速认知：

1. 读取 `README.md`
2. 读取 `CLAUDE.md` 或其他总览性说明
3. 读取关键配置文件和入口文件
4. 浏览顶层目录结构
5. 明确说明这只是临时认知，不等同于完成 `load-memory`

## 完成标准

满足以下条件才算完成：

- 已基于 `aide-memory/memory/` 建立与当前任务匹配的上下文
- 已说明本次读取了哪些文档
- 已指出当前认知的覆盖范围和缺口
- 若 memory 不足，已明确建议下一步而非伪装完成
