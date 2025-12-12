---
description: 任务执行流程。基于任务细则执行任务，包含流程设计、迭代实现、验证交付、文档更新。
argument-hint: [任务细则文档路径]
---

# Aide 任务执行

你正在执行 Aide 任务执行流程。基于任务细则完成实际的任务实施。

## 参数

- `$ARGUMENTS`：任务细则文档路径（可选）
- 未传入时使用配置中的默认路径

---

## 开始

### 确定任务细则

```bash
# 如果传入了参数
细则文档 = $ARGUMENTS

# 如果未传入参数，使用配置默认值
aide config get task.spec
```

读取任务细则内容。如文档不存在，提示用户先执行 `/aide:prep` 或指定文档路径。

### 进入流程设计环节

```bash
aide flow next-part flow-design "进入流程设计环节"
```

---

## 环节 1：流程设计 (flow-design)

### 理解任务细则

- 明确任务目标和成功标准
- 理解执行步骤和依赖关系
- 识别技术决策和约束

### 分析项目环境

根据需要阅读相关代码/文档。

### 制定执行计划

- 具体的实现步骤
- 每个步骤的预期产出
- 潜在风险和应对方案

> 复杂任务建议使用 sequential-thinking 进行结构化规划

### 创建流程图（如需要）

在配置指定的流程图目录创建 PlantUML 源文件。

```bash
aide flow next-step "流程图设计完成"
```

> aide flow 会在此环节自动校验 PlantUML 语法

### 进入下一环节

```bash
aide flow next-part main "流程设计完成，进入实现环节"
```

> aide flow 会自动生成 PNG 流程图

---

## 环节 2：迭代实现 (impl)

### 按计划执行

对于每个实现步骤，完成后记录：

```bash
aide flow next-step "<完成内容简述>"
```

### 处理问题

```bash
# 一般问题（可继续）
aide flow issue "<问题描述>"

# 严重错误（需解决）
aide flow error "<错误描述>"
```

严重错误处理：
1. 尝试自行解决（最多3次）
2. 成功：在 `discuss/` 创建分析文档
3. 失败：停止并告知用户

### 需要回退时

```bash
# 回退到上一步
aide flow back-step "<原因>"

# 回退到之前环节
aide flow back-part <环节名> "<原因>"
```

### 进入下一环节

```bash
aide flow next-part verify "实现完成，进入验证环节"
```

---

## 环节 3：验证交付 (verify)

### 对照任务细则验证

- 每个成功标准是否满足
- 每个交付物是否完成
- 功能是否正常工作

### 执行测试

根据项目情况运行测试或手动验证。

### 记录结果

```bash
aide flow next-step "验证完成: <验证结论>"
```

验证失败时回退修复：

```bash
aide flow back-part main "验证失败: <原因>"
```

### 进入下一环节

```bash
aide flow next-part docs "验证通过，进入文档环节"
```

---

## 环节 4：文档更新 (docs)

> aide flow 会在进入此环节时提示更新 CHANGELOG

### 更新相关文档

- `README.md`（如有用户可见变更）
- `CHANGELOG.md`
- 其他相关文档

### 记录完成

```bash
aide flow next-step "文档更新完成"
```

### 进入下一环节

```bash
aide flow next-part finish "文档更新完成，进入收尾"
```

> aide flow 会校验 CHANGELOG 是否已更新

---

## 环节 5：收尾 (finish)

### 清理工作

- 删除临时文件和调试代码
- 确保代码格式规范
- 检查遗漏的 TODO

### 最终检查

确认所有计划步骤已完成。

### 记录完成

```bash
aide flow next-step "任务完成"
```

### 向用户汇报

总结本次任务：
- 完成了什么
- 主要变更点
- 遗留问题（如有）
