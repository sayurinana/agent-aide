# /aide:prep 命令设计文档

## 一、背景

### 1.1 解决的问题

用户提供的任务描述通常存在以下问题：

| 问题 | 影响 |
|------|------|
| 表述模糊 | 执行方向不明确 |
| 存在歧义 | 可能产生错误理解 |
| 缺少细节 | 执行时需要频繁确认 |
| 多种方案 | 需要用户决策 |

### 1.2 设计目标

将任务描述转化为**清晰、可执行的任务细则**：
- 消除歧义和模糊
- 明确执行步骤和验证标准
- 处理待定项，获取用户决策
- 产出 task-spec.md 供执行阶段使用

---

## 二、职责

### 2.1 做什么

1. 启动流程追踪（task-optimize 环节）
2. 深度分析任务内容
3. 优化任务表述（准确性、简洁性、可执行性）
4. 处理待定项，获取用户确认
5. 生成任务细则（task-spec.md）

### 2.2 不做什么

- 不执行实际的任务实现
- 不编写业务代码
- 不主动关注 git 操作和状态记录（由 aide flow 自动处理）

---

## 三、参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `$ARGUMENTS` | 可选 | 任务原文档路径 |

**未传入参数时**：使用 `aide config get task.source` 获取默认路径（通常为 task-now.md）

---

## 四、执行流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:aide flow start task-optimize "开始任务准备: <任务简述>";

:确定任务文档路径;
note right: 使用参数或配置默认值

:读取任务文档;

if (文档存在?) then (是)
else (否)
  :询问用户提供任务内容;
  stop
endif

partition "阶段1: 任务分析" {
  :深度理解任务;
  note right: 核心目标\n交付物\n成功标准\n涉及模块\n技术难点

  :分析项目环境;
  note right: 阅读相关代码/文档\n理解与现有结构的关系

  :aide flow next-step "任务分析完成";
}

partition "阶段2: 任务优化" {
  :准确性优化;
  note right: 识别歧义\n识别隐含假设\n明确边界

  :简洁性优化;
  note right: 识别冗余\n区分真冗余与必要强调

  :可执行性优化;
  note right: 抽象→具体步骤\n明确输入/输出/验证标准

  :生成待定项;

  :aide flow next-step "任务优化完成，生成待定项";
}

partition "阶段3: 待定项处理" {
  if (有待定项?) then (是)
    :aide decide '<json>';
    :告知用户访问链接;
    :aide decide result;
    :aide flow next-step "用户完成待定项确认";
  else (否)
  endif
}

partition "阶段4: 结果生成" {
  :整合生成任务细则;
  :aide flow next-step "生成任务细则，等待用户确认";

  :展示给用户确认;

  if (用户确认?) then (是)
    :保存为 task-spec.md;
    :aide flow next-step "用户确认任务细则";
  else (否)
    :根据反馈调整;
    note right: 返回相应阶段
  endif
}

:aide flow next-step "任务准备完成";

:提示用户执行 /aide:exec;

stop
@enduml
```

---

## 五、阶段详解

### 5.1 阶段1：任务分析

**核心问题**：
- 任务要解决什么问题？
- 最终交付物是什么？
- 成功的标准是什么？
- 涉及哪些模块/系统？
- 是否有技术难点？

**复杂任务处理**：
- 多子目标、多方案对比时，建议使用 sequential-thinking 进行结构化分析

### 5.2 阶段2：任务优化

| 优化维度 | 关注点 |
|----------|--------|
| 准确性 | 歧义、不明确之处、隐含假设、任务边界 |
| 简洁性 | 冗余表述、区分真冗余与必要强调 |
| 可执行性 | 抽象→具体步骤、输入/输出/验证标准、替代方案 |

### 5.3 阶段3：待定项处理

**待定项类型**：
- 存在多种可行方案
- 有歧义需要澄清
- 需要用户确认的决策

**处理流程**：
1. 准备待定项 JSON 数据
2. 调用 `aide decide '<json>'` 启动 Web 服务
3. 告知用户访问链接进行确认
4. 调用 `aide decide result` 获取决策结果

### 5.4 阶段4：结果生成

**任务细则结构**：
```markdown
# 任务细则

## 任务目标
[清晰描述任务要达成的目标]

## 成功标准
[明确的、可验证的成功标准]

## 执行步骤
1. [步骤1]
2. [步骤2]
...

## 技术决策
[已确认的技术选型]

## 约束与边界
[任务范围边界]
```

---

## 六、与 aide 程序的交互

### 6.1 aide flow start

**调用时机**：命令开始时

**命令**：
```bash
aide flow start task-optimize "开始任务准备: <任务简述>"
```

### 6.2 aide flow next-step

**调用时机**：每个阶段完成时

**命令**：
```bash
aide flow next-step "<完成内容简述>"
```

### 6.3 aide config get

**调用时机**：未传入参数时

**命令**：
```bash
aide config get task.source
```

### 6.4 aide decide

**调用时机**：有待定项需要用户确认时

**命令**：
```bash
aide decide '<json数据>'
aide decide result
```

---

## 七、注意事项

1. **不要主动提及 git 操作**：由 aide flow 自动处理
2. **不要主动提及状态记录**：由 aide flow 自动处理
3. **专注于任务分析和优化**：这是 prep 的核心价值

---

## 八、依赖

| 依赖项 | 类型 | 说明 |
|--------|------|------|
| /aide:init | Command | 需要先完成环境初始化 |
| aide flow | aide 子命令 | 流程追踪 |
| aide decide | aide 子命令 | 待定项处理 |
| aide config | aide 子命令 | 读取配置 |

---

## 九、被依赖

| 依赖方 | 说明 |
|--------|------|
| /aide:exec | 使用 prep 产出的 task-spec.md |

---

## 十、修改指南

### 10.1 修改分析/优化流程

1. 更新本文档的阶段详解
2. 修改执行文件 `../../commands/prep.md`

### 10.2 修改任务细则格式

1. 更新本文档的"结果生成"章节
2. 修改执行文件中的模板

### 10.3 修改待定项处理

1. 更新本文档的"待定项处理"章节
2. 如涉及数据格式变更，同步更新 [数据格式文档](../../../../aide-program/docs/formats/data.md)

---

## 十一、相关文档

- [执行文件](../../commands/prep.md)
- [aide flow 设计](../../../../aide-program/docs/commands/flow.md)
- [aide decide 设计](../../../../aide-program/docs/commands/decide.md)
- [数据格式规范](../../../../aide-program/docs/formats/data.md)
- [plugin 导览](../README.md)
