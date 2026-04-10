---
name: make-graphics
description: 任务图解绘制指南。在 build-task 阶段选定本阶段时使用，用于根据 design.md 的标记判断是否需要绘制图解，并编写 PlantUML 图解保存至 flow-graphics/ 目录。
---

# make-graphics 指南

## 目标

为任务编写 PlantUML 图解，使任务流程、架构设计、数据关系等关键信息可视化。

## 进入前先读

- `information.md` — 任务目标与边界
- `design.md` — 方案细节及是否需要图解的标记
- `todo.md` — 任务拆分与阶段流程

## 判断是否需要图解

### design.md 中的标记格式

```markdown
## 图解需求

[标记] [原因]

### 标记类型

- `必要`：必须绘制图解
- `可选`：建议绘制，但可跳过
- `无需`：不需要图解
- `跳过`：暂不绘制，后续补充

### 示例

- `必要` 涉及多模块协作，需要流程图说明
- `无需` 简单配置调整，无需图解
- `跳过` 时间紧迫，先实现后补图解
```

### 处理规则

| 标记 | 行为 |
|------|------|
| `必要` | 必须绘制，完成后进入下一阶段 |
| `可选` | 绘制或向用户确认跳过 |
| `无需` | 直接跳过，进入下一阶段 |
| `跳过` | 暂不绘制，记录待办后进入下一阶段 |

## 图解类型选择

根据 design.md 内容选择合适类型：

| 类型 | 适用场景 | PlantUML 关键字 |
|------|----------|----------------|
| 流程图 | 业务流程、操作步骤 | `@startuml flow` |
| 序列图 | 交互流程、API调用 | `@startuml sequence` |
| 组件图 | 系统架构、模块关系 | `@startuml component` |
| 类图 | 数据模型、类关系 | `@startuml class` |
| 状态图 | 状态流转、生命周期 | `@startuml state` |
| 用例图 | 功能场景、用户行为 | `@startuml usecase` |

## 编写流程

### 1. 规划图解

- 明确图解要表达的核心信息
- 确定需要展示的元素和关系
- 选择合适的图解类型

### 2. 编写 PlantUML

遵循 plantuml skill 的规范：

```plantuml
@startuml main
skinparam defaultFontName "Noto Sans CJK SC"
skinparam dpi 200
scale 1

title 任务流程图

' 图解内容
...

@enduml
```

### 3. 保存位置

图解文件保存至任务目录的 `flow-graphics/` 下：

```
aide-memory/tasks/task-{n}/flow-graphics/
  main.puml        # 主流程图
  architecture.puml # 架构图（如有）
  data-flow.puml   # 数据流图（如有）
```

### 4. 编译验证

执行 plantuml 编译确认图解正确：

```bash
plantuml flow-graphics/*.puml
```

编译成功后生成对应的 PNG 文件。

## 与 build-task 的衔接

build-task 阶段在 `design.md` 中标记图解需求后，make-graphics 阶段：

1. 读取标记判断是否需要绘制
2. 若需要，绘制并编译验证
3. 若不需要，记录原因后跳过
4. 完成后进入 impl-verify 阶段

## 跳过时的记录

当标记为 `跳过` 时，在 `todo.md` 中添加：

```markdown
## 待办

- [ ] 补充流程图解（标记：跳过，原因：时间紧迫）
```

## 完成条件

make-graphics 阌段完成时，应满足：

- design.md 中的图解标记已处理
- 必要图解已绘制并编译通过
- PNG 文件已生成在 flow-graphics/ 目录
- todo.md 已更新（如有跳过项）