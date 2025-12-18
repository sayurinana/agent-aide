# 子计划 6：返工流程优化

## 目标

优化 Aide 工作流的返工流程，包括：
1. 创建 `rework` skill 提供返工指导
2. 修改 `/aide:run` 命令引用该 skill
3. 修改 aide-program 实现 back-confirm 机制

## 具体步骤

### 6.1 创建 rework skill

**位置**：`aide-marketplace/aide-plugin/skills/rework/SKILL.md`

**内容结构**：

```markdown
# 返工流程指南

## 返工类型判断

| 返工目标 | 触发条件 | 处理方式 |
|----------|----------|----------|
| task-optimize | 新需求、需求理解偏差 | 更新 task.source |
| flow-design | 架构/流程设计问题 | 更新细则 + new-requirements.md |
| impl | 实现问题 | 更新细则 + new-requirements.md |
| verify | 验证未通过 | 更新细则 + new-requirements.md |

## 返工到 task-optimize 阶段

### 流程

1. `aide flow issue "准备返工: [原因简述]"`
2. 更新 task.source 文档：
   - 插入用户新需求原文
   - 记录提出时机
   - 添加 LLM 建议（可选）
3. 提醒用户
4. 执行 `aide flow back-confirm --key [key]`
5. 执行 `aide flow back-part task-optimize "[原因]"`

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

## 返工到其他阶段

### 流程

1. `aide flow issue "准备返工前处理需求整合: [原因简述]"`
2. 创建/更新 new-requirements.md：
   - 记录用户新需求原文
   - 记录提出时机
3. 更新细则文档：
   - 在导览部分添加返工声明
   - 梳理：已完成项、未完成项、需重新处理项
   - 处理需求冲突
   - 融入新需求
   - 删除返工声明
4. 提醒用户
5. 执行 `aide flow back-confirm --key [key]`
6. 执行 `aide flow back-part [阶段] "[原因]"`

### new-requirements.md 格式

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

## 提醒用户

### 返工到 task-optimize

> 我将会对 task-now.md 进行更新，加入您的新需求和我的建议，然后更新流程状态返工到 task-optimize 阶段。建议您在流程状态返工后使用 `/exit` 结束本次对话，重新启动一个新的对话执行 load+run，我将会自动接续任务的处理。

### 返工到其他阶段

> 我将会对 new-requirements.md 进行更新，加入您的新需求和我的建议，然后处理好您的新需求和原细则的需求冲突整合，然后更新流程状态返工到 [阶段] 阶段。建议您在流程状态返工后使用 `/exit` 结束本次对话，重新启动一个新的对话执行 load+run，我将会自动接续任务的处理。

## 确认机制

返工前必须完成确认流程，详见 aide flow back-confirm 命令说明。
```

### 6.2 修改 /aide:run 命令

**位置**：`aide-marketplace/aide-plugin/commands/run.md`

**修改内容**：

在 confirm 阶段的返工流程部分添加：

```markdown
#### 6.3 返工流程

当用户发现问题或有新需求时：

**触发 rework skill**：
加载 `rework` skill 学习返工流程指南，按照指南完成返工处理。
```

### 6.3 修改 aide-program：back-confirm 机制

**位置**：`aide-program/aide/flow/` 模块

**新增命令**：`aide flow back-confirm --key <key>`

**实现逻辑**：

1. **back-part 命令修改**：
   - 执行前检查是否有待确认的 back 请求
   - 如无确认，输出提示并生成随机 key
   - 记录目标阶段和原因到状态文件
   - 要求 LLM 确认已完成准备工作后执行 back-confirm

2. **back-confirm 命令实现**：
   - 验证 key 是否匹配
   - 匹配成功后：
     - 读取状态文件中的目标阶段和原因
     - 直接执行 back-part 操作
     - 暂存所有更改：`git add .`
     - 创建清洁提交：`git commit -m "[aide] 返工前清洁提交"`
     - 输出警告：建议用户 `/exit` 重新对话
     - 清理状态文件

**数据流**：

```
LLM 请求 back-part flow-design "设计遗漏"
       │
       ↓
aide 检测未确认，记录目标和原因，生成 key "abc123"，输出提示
       │
       ↓
LLM 完成准备工作后，执行 back-confirm --key abc123
       │
       ↓
aide 验证 key 成功 → 执行 back-part → 创建清洁提交 → 输出警告
       │
       ↓
结束（LLM 无需第三次请求）
```

### 6.4 状态存储

**位置**：`.aide/back-confirm-state.json`

**格式**：

```json
{
  "pending_key": "abc123",
  "target_part": "flow-design",
  "reason": "设计遗漏，需要补充",
  "created_at": "2025-12-19T10:00:00+08:00"
}
```

> 注：验证成功后直接执行并清理状态文件，无需 confirmed 字段

## 验证标准

- [ ] rework skill 内容完整，指导清晰
- [ ] /aide:run 正确引用 rework skill
- [ ] `aide flow back-part` 在未确认时生成 key 并记录状态
- [ ] `aide flow back-confirm --key` 正确验证 key 并直接执行 back-part
- [ ] 执行后创建清洁提交
- [ ] 输出正确的警告信息
- [ ] 状态文件在完成后被清理

## 依赖

- 前置：无（独立任务）
- 后续：无

## 风险评估

- **风险等级**：中
- **潜在影响**：
  - 确认机制增加操作复杂度
  - 清洁提交可能与用户 git 工作流冲突
- **缓解措施**：
  - 提供清晰的操作提示
  - 清洁提交信息明确标注来源
  - 文档说明机制目的
