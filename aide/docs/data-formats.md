# 数据格式文档

本文档描述 Aide 使用的所有配置文件和状态数据文件的格式。

---

## config.toml

**路径：** `.aide/config.toml`
**格式：** TOML
**创建方式：** `aide init` 自动生成

### 完整结构

```toml
[general]
# 是否在 .gitignore 中忽略 .aide 目录
# true: 自动添加 .aide/ 到 .gitignore
# false（默认）: 不修改 .gitignore
gitignore_aide = false

[task]
# 任务原文档路径
source = "task-now.md"
# 任务细则文档路径
spec = "task-spec.md"
# 复杂任务计划文档目录
plans_path = ".aide/task-plans/"

[docs]
# 项目文档目录路径
path = ".aide/project-docs"

[flow]
# 环节名称列表（有序）
phases = ["task-optimize", "flow-design", "impl", "verify", "docs", "confirm", "finish"]
# 流程图目录路径
diagram_path = ".aide/diagrams"

[plantuml]
# 下载缓存目录（相对于 ~/.aide/）
download_cache_path = "download-buffer"
# 安装完成后是否删除下载的压缩包
clean_cache_after_install = true
# 工具程序安装目录（相对于 ~/.aide/）
install_path = "utils"
# PlantUML 程序包下载链接
download_url = "https://github.com/sayurinana/agent-aide/releases/download/resource-001/plantuml-1.2025.4-linux-x64.tar.gz"
# 默认字体名称
font_name = "Arial"
# DPI 值
dpi = 300
# 缩放系数
scale = 0.5

[decide]
# HTTP 服务起始端口
port = 3721
# 监听地址
bind = "127.0.0.1"
# 自定义访问地址（可选）
url = ""
# 超时时间（秒），0 = 不超时
timeout = 0
```

### 字段说明

| 键 | 类型 | 默认值 | 说明 |
|---|---|---|---|
| `general.gitignore_aide` | bool | `false` | 是否将 `.aide/` 添加到 `.gitignore` |
| `task.source` | string | `"task-now.md"` | 任务原文档路径 |
| `task.spec` | string | `"task-spec.md"` | 任务细则文档路径 |
| `task.plans_path` | string | `".aide/task-plans/"` | 复杂任务计划目录 |
| `docs.path` | string | `".aide/project-docs"` | 项目文档目录 |
| `flow.phases` | string[] | 7 个默认环节 | 有序环节列表 |
| `flow.diagram_path` | string | `".aide/diagrams"` | 流程图目录 |
| `plantuml.download_cache_path` | string | `"download-buffer"` | 下载缓存目录（相对于 ~/.aide/） |
| `plantuml.clean_cache_after_install` | bool | `true` | 安装后是否删除缓存 |
| `plantuml.install_path` | string | `"utils"` | 工具安装目录（相对于 ~/.aide/） |
| `plantuml.download_url` | string | GitHub 链接 | PlantUML 程序包下载链接 |
| `plantuml.font_name` | string | `"Arial"` | PlantUML 字体 |
| `plantuml.dpi` | int | `300` | PlantUML DPI |
| `plantuml.scale` | float | `0.5` | PlantUML 缩放系数 |
| `decide.port` | int | `3721` | HTTP 服务起始端口 |
| `decide.bind` | string | `"127.0.0.1"` | 监听地址 |
| `decide.url` | string | `""` | 自定义访问地址 |
| `decide.timeout` | int | `0` | 超时秒数（0=不超时） |

---

## flow-status.json

**路径：** `.aide/flow-status.json`
**格式：** JSON
**生命周期：** `aide flow start` 创建，任务完成后归档到 `.aide/logs/`

### 结构

```json
{
  "task_id": "2024-01-15T14-30-00",
  "current_phase": "impl",
  "current_step": 3,
  "started_at": "2024-01-15T14:30:00+08:00",
  "history": [
    {
      "timestamp": "2024-01-15T14:30:00+08:00",
      "action": "start",
      "phase": "task-optimize",
      "step": 1,
      "summary": "开始任务",
      "git_commit": "abc1234"
    }
  ],
  "source_branch": "main",
  "start_commit": "def5678",
  "task_branch": "aide/2024-01-15T14-30-00"
}
```

### 字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `task_id` | string | 是 | 唯一任务标识（时间戳格式） |
| `current_phase` | string | 是 | 当前环节名 |
| `current_step` | int | 是 | 当前步骤编号（从 1 开始） |
| `started_at` | string | 是 | 任务开始时间（ISO 8601） |
| `history` | array | 是 | 操作历史记录 |
| `source_branch` | string | 否 | 源分支名 |
| `start_commit` | string | 否 | 起始提交哈希 |
| `task_branch` | string | 否 | 任务分支名 |

### HistoryEntry 字段

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `timestamp` | string | 是 | 操作时间（ISO 8601） |
| `action` | string | 是 | 操作类型（start/next-step/back-step/next-part/back-part/issue/error） |
| `phase` | string | 是 | 操作时所在环节 |
| `step` | int | 是 | 操作时的步骤编号 |
| `summary` | string | 是 | 操作说明 |
| `git_commit` | string | 否 | 关联的 git 提交哈希 |

### 归档文件

任务完成后状态文件归档为 `.aide/logs/flow-status.<task_id>.json`，格式与 `flow-status.json` 相同。

---

## branches.json

**路径：** `.aide/branches.json`
**格式：** JSON
**说明：** 记录所有分支的编号和元数据

### 结构

```json
{
  "branches": {
    "aide/2024-01-15T14-30-00": {
      "number": 1,
      "source_branch": "main",
      "created_at": "2024-01-15T14:30:00+08:00"
    }
  },
  "next_number": 2
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `branches` | object | 分支名到分支信息的映射 |
| `branches.<name>.number` | int | 分支编号 |
| `branches.<name>.source_branch` | string | 源分支名 |
| `branches.<name>.created_at` | string | 创建时间 |
| `next_number` | int | 下一个分支的编号 |

---

## back-confirm-state.json

**路径：** `.aide/back-confirm-state.json`
**格式：** JSON
**生命周期：** `aide flow back-part` 创建，`aide flow back-confirm` 或 `aide flow clean` 删除

### 结构

```json
{
  "pending_key": "a1b2c3d4e5f6",
  "target_part": "task-optimize",
  "reason": "需要重新设计接口",
  "created_at": "2024-01-15T16:00:00+08:00"
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `pending_key` | string | 确认密钥（12 位十六进制） |
| `target_part` | string | 回退目标环节 |
| `reason` | string | 回退原因 |
| `created_at` | string | 创建时间（ISO 8601） |

---

## Decide JSON 格式

### 输入文件（DecideInput）

提交给 `aide decide submit` 的 JSON 文件格式。

```json
{
  "task": "选择技术方案",
  "source": "task-spec.md",
  "items": [
    {
      "id": 1,
      "title": "选择数据库",
      "options": [
        {
          "value": "postgres",
          "label": "PostgreSQL",
          "score": 85.0,
          "pros": ["成熟稳定", "功能丰富"],
          "cons": ["运维复杂度高"]
        },
        {
          "value": "sqlite",
          "label": "SQLite",
          "score": 70.0,
          "pros": ["零配置"],
          "cons": ["并发限制"]
        }
      ],
      "location": {
        "file": "src/db.rs",
        "start": 10,
        "end": 25
      },
      "context": "项目需要一个嵌入式数据库",
      "recommend": "postgres"
    }
  ]
}
```

### 输入字段校验规则

| 字段 | 类型 | 必填 | 校验规则 |
|------|------|------|---------|
| `task` | string | 是 | 非空 |
| `source` | string | 是 | 非空 |
| `items` | array | 是 | 至少 1 个元素 |
| `items[].id` | int | 是 | 正整数，不重复 |
| `items[].title` | string | 是 | 非空 |
| `items[].options` | array | 是 | 至少 2 个选项 |
| `items[].options[].value` | string | 是 | 非空，同一 item 内唯一 |
| `items[].options[].label` | string | 是 | 非空 |
| `items[].options[].score` | float | 否 | 0-100 |
| `items[].options[].pros` | string[] | 否 | - |
| `items[].options[].cons` | string[] | 否 | - |
| `items[].location` | object | 否 | - |
| `items[].location.file` | string | 条件必填 | location 存在时非空 |
| `items[].location.start` | int | 条件必填 | - |
| `items[].location.end` | int | 条件必填 | - |
| `items[].context` | string | 否 | - |
| `items[].recommend` | string | 否 | 必须是 options 中的有效 value |

### 输出格式（DecideOutput）

`aide decide result` 的输出格式。

```json
{
  "decisions": [
    {
      "id": 1,
      "chosen": "postgres",
      "note": "选择 PostgreSQL 因为更成熟"
    }
  ]
}
```

### 输出字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `decisions` | array | 是 | 与 items 一一对应 |
| `decisions[].id` | int | 是 | 对应 item 的 id |
| `decisions[].chosen` | string | 是 | 选择的 option value |
| `decisions[].note` | string | 否 | 用户备注 |

### 决策记录（DecisionRecord）

保存在 `.aide/decisions/<session_id>.json`，完整记录输入和输出。

```json
{
  "input": { "task": "...", "source": "...", "items": [...] },
  "output": { "decisions": [...] },
  "completed_at": "2024-01-15T17:00:00+08:00"
}
```

### server.json

**路径：** `.aide/decisions/server.json`
**说明：** 记录运行中的 decide 服务器信息

```json
{
  "pid": 12345,
  "port": 3721,
  "url": "http://127.0.0.1:3721",
  "started_at": "2024-01-15T16:30:00+08:00"
}
```

---

## 目录结构总览

```
.aide/
├── config.toml              # 核心配置文件
├── flow-status.json         # 当前任务状态（活跃时存在）
├── flow-status.lock         # 状态文件锁（操作期间存在）
├── back-confirm-state.json  # 回退待确认状态（待确认时存在）
├── branches.json            # 分支管理数据
├── decisions/
│   ├── pending.json         # 当前待定项数据
│   ├── server.json          # 服务器运行信息
│   └── <session_id>.json    # 决策记录
├── logs/
│   └── flow-status.<task_id>.json  # 归档的任务状态
├── task-plans/              # 任务计划目录
├── project-docs/            # 项目文档目录
└── diagrams/                # 流程图目录
```
