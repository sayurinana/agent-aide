# 数据格式规范

## 一、概述

本文档定义 aide 系统中使用的各种数据格式，包括：
- 待定项数据格式（aide decide）
- 流程状态格式（aide flow）
- 决策记录格式

---

## 二、待定项数据格式

### 2.1 输入格式（LLM → aide decide）

LLM 调用 `aide decide '<json>'` 时传入的数据格式。

```
DecideInput:
    task: string              # 任务简述
    source: string            # 来源文档路径
    items: DecideItem[]       # 待定项列表

DecideItem:
    id: number                # 待定项 ID（唯一标识）
    title: string             # 问题标题
    location?: Location       # 原文位置（可选）
    context?: string          # 详细说明（可选）
    options: Option[]         # 选项列表（至少2个）
    recommend?: string        # 推荐选项的 value（可选）

Location:
    file: string              # 文件路径
    start: number             # 起始行号
    end: number               # 结束行号

Option:
    value: string             # 选项标识（用于返回结果）
    label: string             # 选项描述（显示给用户）
    score?: number            # 评分 0-100（可选）
    pros?: string[]           # 优点列表（可选）
    cons?: string[]           # 缺点列表（可选）
```

**示例**：

```json
{
  "task": "实现用户认证模块",
  "source": "task-now.md",
  "items": [
    {
      "id": 1,
      "title": "认证方式选择",
      "location": {
        "file": "task-now.md",
        "start": 5,
        "end": 7
      },
      "context": "任务描述中未明确指定认证方式，需要确认",
      "options": [
        {
          "value": "jwt",
          "label": "JWT Token 认证",
          "score": 85,
          "pros": ["无状态", "易于扩展", "跨域友好"],
          "cons": ["Token 无法主动失效", "需要处理刷新"]
        },
        {
          "value": "session",
          "label": "Session 认证",
          "score": 70,
          "pros": ["实现简单", "可主动失效"],
          "cons": ["需要存储", "扩展性差"]
        }
      ],
      "recommend": "jwt"
    },
    {
      "id": 2,
      "title": "密码加密算法",
      "context": "选择密码存储的加密算法",
      "options": [
        {
          "value": "bcrypt",
          "label": "bcrypt",
          "score": 90,
          "pros": ["安全性高", "自带盐值"],
          "cons": ["计算较慢"]
        },
        {
          "value": "argon2",
          "label": "Argon2",
          "score": 95,
          "pros": ["最新标准", "抗GPU攻击"],
          "cons": ["库支持较少"]
        }
      ],
      "recommend": "bcrypt"
    }
  ]
}
```

### 2.2 输出格式（aide decide result → LLM）

LLM 调用 `aide decide result` 时返回的数据格式。

```
DecideOutput:
    decisions: Decision[]     # 决策列表

Decision:
    id: number                # 待定项 ID
    chosen: string            # 选中的选项 value
    note?: string             # 用户备注（可选，仅在用户填写时出现）
```

**示例**：

```json
{
  "decisions": [
    {"id": 1, "chosen": "jwt"},
    {"id": 2, "chosen": "bcrypt", "note": "团队更熟悉 bcrypt"}
  ]
}
```

---

## 三、流程状态格式

### 3.1 状态文件格式

位置：`.aide/flow-status.json`

```
FlowStatus:
    task_id: string           # 任务标识（时间戳格式）
    current_phase: string     # 当前环节名
    current_step: number      # 当前步骤序号
    started_at: string        # 开始时间（ISO 8601 格式）
    history: HistoryEntry[]   # 历史记录

HistoryEntry:
    timestamp: string         # 时间戳（ISO 8601 格式）
    action: string            # 操作类型
    phase: string             # 环节名
    step: number              # 步骤序号
    summary: string           # 总结/原因
    git_commit?: string       # git commit hash（可选）
```

**操作类型（action）**：
- `start` - 开始任务
- `next-step` - 步骤前进
- `back-step` - 步骤回退
- `next-part` - 环节前进
- `back-part` - 环节回退
- `issue` - 记录问题
- `error` - 记录错误

**示例**：

```json
{
  "task_id": "2025-01-15T10-30-00",
  "current_phase": "impl",
  "current_step": 5,
  "started_at": "2025-01-15T10:30:00+08:00",
  "history": [
    {
      "timestamp": "2025-01-15T10:30:00+08:00",
      "action": "start",
      "phase": "flow-design",
      "step": 1,
      "summary": "开始任务: 实现用户认证模块",
      "git_commit": "a1b2c3d"
    },
    {
      "timestamp": "2025-01-15T10:45:00+08:00",
      "action": "next-step",
      "phase": "flow-design",
      "step": 2,
      "summary": "流程图设计完成",
      "git_commit": "e4f5g6h"
    },
    {
      "timestamp": "2025-01-15T11:00:00+08:00",
      "action": "next-part",
      "phase": "impl",
      "step": 3,
      "summary": "流程设计完成，开始实现",
      "git_commit": "i7j8k9l"
    },
    {
      "timestamp": "2025-01-15T11:30:00+08:00",
      "action": "issue",
      "phase": "impl",
      "step": 4,
      "summary": "测试覆盖率较低，后续需要补充",
      "git_commit": "m0n1o2p"
    },
    {
      "timestamp": "2025-01-15T12:00:00+08:00",
      "action": "next-step",
      "phase": "impl",
      "step": 5,
      "summary": "完成用户模型定义",
      "git_commit": "q3r4s5t"
    }
  ]
}
```

---

## 四、决策记录格式

### 4.1 存储位置

```
.aide/
└── decisions/
    ├── pending.json          # 当前待处理的待定项
    └── 2025-01-15T10-30-00.json  # 历史决策记录
```

### 4.2 待处理文件格式

位置：`.aide/decisions/pending.json`

内容：与输入格式相同（DecideInput）

### 4.3 历史记录格式

位置：`.aide/decisions/{timestamp}.json`

```
DecisionRecord:
    input: DecideInput        # 原始输入
    output: DecideOutput      # 决策结果
    completed_at: string      # 完成时间（ISO 8601 格式）
```

**示例**：

```json
{
  "input": {
    "task": "实现用户认证模块",
    "source": "task-now.md",
    "items": [...]
  },
  "output": {
    "decisions": [
      {"id": 1, "chosen": "jwt"},
      {"id": 2, "chosen": "bcrypt", "note": "团队更熟悉 bcrypt"}
    ]
  },
  "completed_at": "2025-01-15T10:35:00+08:00"
}
```

---

## 五、Git 提交信息格式

### 5.1 自动提交格式

aide flow 自动生成的提交信息格式：

```
[aide] <环节>: <总结>
```

**示例**：
```
[aide] flow-design: 开始任务: 实现用户认证模块
[aide] flow-design: 流程图设计完成
[aide] impl: 流程设计完成，开始实现
[aide] impl: 完成用户模型定义
[aide] verify: 验证通过
[aide] docs: 更新 CHANGELOG
[aide] finish: 任务完成
```

### 5.2 问题/错误记录

```
[aide] <环节> issue: <描述>
[aide] <环节> error: <描述>
```

### 5.3 回退记录

```
[aide] <环节> back-step: <原因>
[aide] <环节> back-part: <原因>
```

---

## 六、时间格式

所有时间字段使用 **ISO 8601** 格式：

```
YYYY-MM-DDTHH:mm:ss+HH:00
```

**示例**：
```
2025-01-15T10:30:00+08:00
```

文件名中的时间戳使用简化格式（避免特殊字符）：

```
YYYY-MM-DDTHH-mm-ss
```

**示例**：
```
2025-01-15T10-30-00
```

---

## 七、修改指南

### 7.1 修改待定项格式

1. 更新本文档的"待定项数据格式"章节
2. 修改 `aide/decide/` 相关代码
3. 同步更新 [aide decide 设计](../commands/decide.md)
4. 同步更新 [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)

### 7.2 修改流程状态格式

1. 更新本文档的"流程状态格式"章节
2. 修改 `aide/flow/` 相关代码
3. 同步更新 [aide flow 设计](../commands/flow.md)

### 7.3 添加新的数据格式

1. 在本文档添加新章节
2. 定义数据结构
3. 提供示例
4. 更新相关设计文档

---

## 八、相关文档

- [program 导览](../README.md)
- [aide flow 设计](../commands/flow.md)
- [aide decide 设计](../commands/decide.md)
- [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)
