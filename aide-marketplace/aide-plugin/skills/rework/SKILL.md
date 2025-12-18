# 返工流程指南

本指南帮助你在 Aide 工作流中处理返工（rework）场景。返工是指用户在 confirm 阶段发现问题或有新需求，需要回退到之前的阶段重新处理。

## 返工类型判断

根据问题类型选择返工目标阶段：

| 返工目标 | 触发条件 | 处理方式 |
|----------|----------|----------|
| task-optimize | 新需求、需求理解偏差 | 更新 task.source |
| flow-design | 架构/流程设计问题 | 更新细则 + new-requirements.md |
| impl | 实现问题 | 更新细则 + new-requirements.md |
| verify | 验证未通过 | 更新细则 + new-requirements.md |

---

## 返工到 task-optimize 阶段

当需要重新理解需求或有重大新需求时使用。

### 流程

1. 记录问题：
   ```bash
   aide flow issue "准备返工: [原因简述]"
   ```

2. 更新 task.source 文档（见下方格式）

3. 向用户发出提醒（见下方模板）

4. 执行确认命令：
   ```bash
   aide flow back-confirm --key [key]
   ```
   > 注：key 由 `aide flow back-part` 生成

5. 执行返工命令：
   ```bash
   aide flow back-part task-optimize "[原因]"
   ```

### task.source 更新格式

在文档末尾添加：

```markdown
---

## 返工记录 [时间戳]

### 用户反馈
[用户原文]

### 提出时机
[在哪个阶段/步骤提出]

### LLM 建议
[如有]
```

---

## 返工到其他阶段

返工到 flow-design、impl、verify 阶段时使用。

### 流程

1. 记录问题：
   ```bash
   aide flow issue "准备返工前处理需求整合: [原因简述]"
   ```

2. 创建/更新 new-requirements.md（见下方格式）

3. 更新细则文档：
   - 在导览部分添加返工声明
   - 梳理：已完成项、未完成项、需重新处理项
   - 处理需求冲突
   - 融入新需求
   - 删除返工声明

4. 向用户发出提醒（见下方模板）

5. 执行确认命令：
   ```bash
   aide flow back-confirm --key [key]
   ```

6. 执行返工命令：
   ```bash
   aide flow back-part [阶段] "[原因]"
   ```

### new-requirements.md 格式

位置：`.aide/new-requirements.md`

```markdown
# 新需求记录

## 返工 [时间戳]

### 用户反馈
[原文]

### 提出时机
[阶段/步骤]

### 影响分析
- 已完成项：[列表]
- 未完成项：[列表]
- 需重新处理：[列表]

### 冲突处理
[如何处理与原细则的冲突]
```

---

## 提醒用户模板

### 返工到 task-optimize

> 我将会对 task-now.md 进行更新，加入您的新需求和我的建议，然后更新流程状态返工到 task-optimize 阶段。建议您在流程状态返工后使用 `/exit` 结束本次对话，重新启动一个新的对话执行 load+run，我将会自动接续任务的处理。

### 返工到其他阶段

> 我将会对 new-requirements.md 进行更新，加入您的新需求和我的建议，然后处理好您的新需求和原细则的需求冲突整合，然后更新流程状态返工到 [阶段] 阶段。建议您在流程状态返工后使用 `/exit` 结束本次对话，重新启动一个新的对话执行 load+run，我将会自动接续任务的处理。

---

## 确认机制

返工前必须完成确认流程：

1. 当执行 `aide flow back-part` 时，系统会检测是否已确认
2. 如未确认，系统会生成一个随机 key 并输出提示
3. 完成准备工作后，执行 `aide flow back-confirm --key <key>` 确认
4. 确认成功后，系统会自动执行返工操作并创建清洁提交

### 命令说明

```bash
# 首次调用：生成 key，记录目标阶段
aide flow back-part flow-design "设计遗漏"
# 输出: ⚠ 返工需要确认。请先完成准备工作，然后执行:
#       aide flow back-confirm --key abc123

# 完成准备后：确认并执行
aide flow back-confirm --key abc123
# 输出: ✓ 返工确认成功，回退到环节: flow-design
#       ⚠ 建议执行 /exit 重新开始对话
```

---

## 注意事项

1. **文档优先**：返工前必须更新相关文档，确保上下文不丢失
2. **清洁提交**：确认后系统会自动创建 git 提交，保留当前状态
3. **重启对话**：建议返工后重启对话，避免上下文混乱
4. **状态文件**：确认状态存储在 `.aide/back-confirm-state.json`
