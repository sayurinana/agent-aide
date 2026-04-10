---
name: make-memory
description: 为项目生成或更新 `aide-memory/memory/` 文档集。当需要建立项目长期认知、补齐 memory、或 `/aide:make-memory` 要求产出 `overview.md`、`structure/`、`concepts/`、`diagram/` 时使用。
---

# 项目 Memory 构建指南

本 skill 用于把项目源码、目录结构和关键概念整理为可长期复用的 `aide-memory/memory/` 文档集。

## 目标产物

| 路径 | 作用 |
|------|------|
| `aide-memory/memory/overview.md` | 全局导览，说明项目用途、技术栈、主要模块和阅读路径 |
| `aide-memory/memory/structure/index.md` | 完整目录结构索引和总体统计 |
| `aide-memory/memory/structure/*.md` | 按区块拆分的内容概述文档 |
| `aide-memory/memory/concepts/term.md` | 项目专用术语表 |
| `aide-memory/memory/concepts/arch.md` | 抽象架构、模块关系和关键机制说明 |
| `aide-memory/memory/diagram/*.puml` | 服务于概念理解的图解源码 |

## 核心原则

1. **完整覆盖**：递归扫描项目目录下的文件和目录，不遗漏空目录；仅排除 `.git` 和 `.gitignore` 忽略项。
2. **结构与概念分离**：`structure/` 讲“项目里有什么”，`concepts/` 讲“这些内容为什么存在、如何协作”。
3. **先事实后抽象**：先基于源码和文档建立结构认知，再沉淀术语、架构和图解。
4. **图解服务理解**：`diagram/` 只绘制真正有助于理解的关键机制，不为了“有图而画图”。
5. **总工程师验收**：可以拆分专家代理做探索，但最终由总工程师统一检查覆盖度、一致性和过期风险。
6. **旧配置已废弃**：不要再使用旧版 `docs.path`、`docs.block_plan_path`、`docs.steps_path` 和 `.aide/project-docs/` 流程；新产物统一写入 `aide-memory/memory/`。

## 前置准备

1. 先完整阅读：
   - `aide-memory/aide-process-overview.md`
   - `aide-memory/AGENT.md`
2. 学习 `aide` skill，了解如何查询配置和状态。
3. 需要编写图解时，额外学习 `plantuml` skill。
4. 若宿主支持子代理，优先由总工程师拆出“项目认知专家”负责深度扫描和初稿产出。

## 执行流程

### 1. 确认输出骨架

目标目录固定为：

```text
aide-memory/memory/
├── overview.md
├── structure/
│   ├── index.md
│   └── *.md
├── concepts/
│   ├── term.md
│   └── arch.md
└── diagram/
    └── *.puml
```

如果 `aide-memory/` 尚不存在，先让总工程师执行初始化动作，确保基础目录已就绪。

### 2. 建立完整扫描视图

1. 读取项目根目录 `.gitignore`。
2. 递归遍历所有目录和文件。
3. 对每个条目至少识别：
   - 路径
   - 类型（目录 / 源码 / 配置 / 文档 / 二进制 / 其他）
   - 是否被忽略
   - 与哪个模块或职责相关
4. 保留“完整目录树”视图，供后续写入 `structure/index.md`。

### 3. 划分结构区块

按以下原则划分 `structure/*.md`：

- 优先按顶层目录或明确功能模块划分
- 同一功能链路尽量放在同一篇区块文档中
- 空目录和被忽略目录也要在索引中体现
- 区块名应稳定、可复用、便于后续按需载入

区块文档建议使用 kebab-case 命名，例如：

- `root.md`
- `src-core.md`
- `aide-plugin-commands.md`

### 4. 编写结构文档

先写各区块文档，再汇总 `structure/index.md`。

每篇 `structure/*.md` 至少包含：

- 区块职责概述
- 该区块的局部目录树
- 文件清单或关键文件表
- 核心组件 / 入口 / 数据流
- 与其他区块的依赖关系

可参考以下骨架：

````markdown
# [区块名]

> 路径：xxx/
> 最后更新：YYYY-MM-DD

## 概述

[该区块的主要职责]

## 目录结构

```text
xxx/
├── ...
```

## 关键文件

| 路径 | 类型 | 说明 |
|------|------|------|
| `...` | 源码 | ... |

## 核心组件

### [组件名]

- 职责：...
- 位置：`路径`
- 依赖：...
````

### 5. 汇总 `structure/index.md`

`index.md` 要承担“完整目录结构索引 + 总览统计”的职责，至少包含：

- 完整目录树
- 被忽略项标记
- 空目录标记
- 主要区块索引和链接
- 文件数 / 目录数 / 忽略项等统计

`index.md` 是 `load-memory` 的基础入口之一，必须保证可快速定位模块。

### 6. 编写 `concepts/term.md`

记录项目专用术语、简称和常用口头表达与真实代码位置的对应关系。

只记录满足以下条件的术语：

- 在项目或用户交流中高频出现
- 可以明显缩短沟通成本
- 已经有足够证据支撑，不是臆测

建议格式：

```markdown
# 项目术语表

## [术语]

- 含义：...
- 对应位置：`路径` / `模块`
- 使用场景：...
- 备注：...
```

### 7. 编写 `concepts/arch.md`

把结构层事实抽象为更高层的架构叙述，重点说明：

- 项目的核心目标
- 主要模块与职责边界
- 关键数据流 / 调用链 / 生命周期
- 对外依赖和内部扩展点
- 需要配合图解理解的复杂机制

必要时可在文中引用 `diagram/*.puml`，例如“可参考 `bootstrap.puml`”。

### 8. 编写 `diagram/*.puml`

只为真正复杂或跨模块的机制绘图，例如：

- 系统启动流程
- 任务状态流转
- 核心模块协作关系

要求：

- 源文件保存在 `aide-memory/memory/diagram/`
- 使用 PlantUML 语法
- 文件名应表达主题，如 `bootstrap.puml`、`task-flow.puml`
- 图与 `arch.md` 的叙述要互相对应

### 9. 编写 `overview.md`

`overview.md` 是 memory 的总入口，建议包含：

- 项目用途和定位
- 技术栈
- 顶层目录说明
- 推荐阅读顺序
- 常见任务应先看哪些 memory 文档

推荐阅读顺序通常为：

1. `overview.md`
2. `structure/index.md`
3. 相关 `structure/*.md`
4. `concepts/term.md`
5. `concepts/arch.md`
6. 相关 `diagram/*.puml`

## 大项目处理策略

如果项目规模较大，不要恢复旧版 `block-plan.md` / `steps/` 机制，而应：

1. 由总工程师先划分模块边界。
2. 让专家代理按模块并行探索。
3. 统一回收产出，写入同一套 `memory/` 文档。
4. 最终由总工程师做去重、统一命名和一致性验收。

## 完成标准

满足以下条件才算完成：

- `overview.md`、`structure/index.md`、`concepts/term.md`、`concepts/arch.md` 已存在且内容有效
- 至少覆盖项目关键目录和核心模块
- 结构文档与概念文档之间没有明显冲突
- 图解文件与 `arch.md` 引用关系一致
- 能让后续 `load-memory` 在不重扫全仓库的情况下快速建立项目认知
