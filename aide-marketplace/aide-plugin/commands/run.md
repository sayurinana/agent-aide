# Aide 任务执行

你正在执行 Aide 任务执行流程。这是核心命令，整合了任务准备和任务执行。

## 参数

- `$ARGUMENTS`：任务文档路径（可选）
- 未传入时根据 flow 状态决定行为

---

## 前置准备

**首先触发 `aide` skill 学习 aide 命令的使用方法。**

---

## 开始

### 检查 flow 状态

```bash
aide flow status
```

根据状态决定行为：

- **无活跃任务** 或 **当前任务已 finish** → 进入**新任务流程**
- **当前任务未完成** → 进入**续接流程**

---

## 新任务流程

### 阶段 1：任务准备 (task-optimize)

#### 1.1 启动流程追踪

```bash
aide flow start task-optimize "开始任务准备: <任务简述>"
```

#### 1.2 确定任务文档

```bash
# 如果传入了参数
任务文档 = $ARGUMENTS

# 如果未传入参数，使用配置默认值
aide config get task.source
```

读取任务文档内容。如文档不存在，询问用户提供任务内容。

#### 1.3 任务分析

- 任务要解决什么问题？
- 最终交付物是什么？
- 成功的标准是什么？
- 涉及哪些模块/系统？

#### 1.4 复杂度评估

根据**任务复杂度评估指导原则**（见附录）评估任务复杂度：

- **简单/中等任务**：直接生成任务细则
- **复杂/超大任务**：拆分为多个子计划

#### 1.5 任务优化

- 识别歧义和不明确之处
- 识别隐含假设
- 明确任务边界
- 生成待定项（如有）

#### 1.6 待定项处理（如有）

```bash
aide decide submit .aide/pending-items.json
# 用户完成后
aide decide result
```

#### 1.7 生成任务细则

产出任务细则文档，保存到配置的路径。

```bash
aide flow next-step "任务准备完成"
```

### 阶段 2：流程设计 (flow-design)

```bash
aide flow next-part flow-design "进入流程设计环节"
```

#### 2.1 制定执行计划

- 具体的实现步骤
- 每个步骤的预期产出
- 潜在风险和应对方案

#### 2.2 创建流程图

在配置的流程图目录创建 PlantUML 源文件：

```bash
aide config get flow.diagram_path
```

**所有任务必须有流程图**，用于：
- 规范化思考
- 方便用户审阅
- 早期发现逻辑错误

```bash
aide flow next-step "流程图设计完成"
```

#### 2.3 进入实现环节

```bash
aide flow next-part impl "流程设计完成，进入实现环节"
```

> aide flow 会自动校验 PlantUML 并生成 PNG

### 阶段 3：迭代实现 (impl)

按计划执行，每完成一个步骤：

```bash
aide flow next-step "<完成内容简述>"
```

遇到问题时：

```bash
aide flow issue "<一般问题>"
aide flow error "<严重错误>"
```

需要回退时：

```bash
aide flow back-step "<原因>"
aide flow back-part <环节名> "<原因>"
```

### 阶段 4：验证交付 (verify)

```bash
aide flow next-part verify "实现完成，进入验证环节"
```

- 对照任务细则验证每个成功标准
- 执行测试（如适用）
- 验证失败则回退修复

```bash
aide flow next-step "验证完成: <验证结论>"
```

### 阶段 5：文档更新 (docs)

```bash
aide flow next-part docs "验证通过，进入文档环节"
```

更新相关文档：
- `README.md`（如有用户可见变更）
- `CHANGELOG.md`
- 其他相关文档

```bash
aide flow next-step "文档更新完成"
```

### 阶段 6：收尾 (finish)

```bash
aide flow next-part finish "文档更新完成，进入收尾"
```

- 清理临时文件
- 检查遗漏的 TODO
- 向用户汇报完成情况

```bash
aide flow next-step "任务完成"
```

---

## 续接流程

当检测到未完成的任务时：

### 1. 分析当前进度

```bash
aide flow status
aide flow show <task_id>
```

了解：
- 当前处于哪个环节
- 已完成哪些步骤
- 最后的操作是什么

### 2. 载入项目认知

调用 `/aide:load` 的逻辑，按需载入项目文档。

### 3. 读取任务细则

```bash
aide config get task.spec
```

读取任务细则，了解任务目标和计划。

### 4. 继续执行

根据当前环节，从中断处继续执行。

---

## 复杂任务的子计划执行

对于拆分为多个子计划的复杂任务：

### 执行模式

```
task-optimize → [flow-design → impl → verify → docs] × N → finish
```

### 流程

1. 完成 task-optimize，生成任务导览和所有子计划细则
2. 对每个子计划：
   - 进入 flow-design，为该子计划设计流程图
   - 进入 impl，实现该子计划
   - 进入 verify，验证该子计划
   - 进入 docs，更新该子计划相关文档
   - 标记子计划完成，更新任务导览
3. 所有子计划完成后，进入 finish

### 子计划切换

完成一个子计划的 docs 后：

```bash
aide flow next-step "子计划 N 完成，开始子计划 N+1"
aide flow back-part flow-design "开始下一个子计划的流程设计"
```

---

## 附录：任务复杂度评估指导原则

### 评估维度

1. **结构维度**：模块数量、文件数量、依赖关系
2. **逻辑维度**：业务复杂度、状态管理、边界条件
3. **集成维度**：外部依赖、数据格式、兼容性
4. **风险维度**：技术风险、影响范围、回滚难度

### 复杂度等级

| 等级 | 特征 | 处理方式 |
|------|------|----------|
| 简单 | 单文件或少量文件，逻辑清晰 | 直接执行 |
| 中等 | 2-4 个模块，有一定依赖 | 直接执行，注意顺序 |
| 复杂 | 5+ 模块，复杂依赖 | **拆分为子计划** |
| 超大 | 10+ 模块，全面重构 | 拆分为独立任务 |

### 拆分判断标准

满足以下任一条件时应拆分：
1. 涉及 3 个以上独立功能模块
2. 任务自然分为多个可独立交付的阶段
3. 存在高风险环节，需要阶段性验证
4. 存在明确的前后依赖关系
5. 单次对话可能无法完成
