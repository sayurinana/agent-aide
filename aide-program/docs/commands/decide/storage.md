# aide decide 数据存储设计

## 一、概述

aide decide 的数据存储负责管理待定项数据和决策记录的持久化。

### 1.1 存储位置

所有数据存储在项目根目录的 `.aide/decisions/` 下：

```
.aide/
└── decisions/
    ├── pending.json              # 当前待处理的待定项
    └── 2025-01-15T10-30-00.json  # 历史决策记录
```

### 1.2 数据格式规范

数据格式以 `aide-program/docs/formats/data.md` 为准，本文档补充存储相关的实现细节。

## 二、文件说明

### 2.1 pending.json

**用途**：存储当前待处理的待定项数据

**生命周期**：
- 创建：`aide decide '<json>'` 执行时
- 读取：Web 前端通过 API 获取
- 保留：决策完成后保留，用于 `aide decide result` 验证匹配性
- 覆盖：下次 `aide decide '<json>'` 执行时覆盖

**内容格式**：与输入数据格式相同（DecideInput）

```json
{
  "task": "实现用户认证模块",
  "source": "task-now.md",
  "items": [...],
  "_meta": {
    "created_at": "2025-01-15T10:30:00+08:00",
    "session_id": "2025-01-15T10-30-00"
  }
}
```

**元数据字段**（`_meta`）：
- `created_at`：创建时间（ISO 8601 格式）
- `session_id`：会话标识（用于匹配决策记录）

### 2.2 历史决策记录

**文件名格式**：`{session_id}.json`

**示例**：`2025-01-15T10-30-00.json`

**内容格式**：

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

## 三、存储操作

### 3.1 保存待定项数据

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:接收 DecideInput 数据;

:生成 session_id;
note right: 格式: YYYY-MM-DDTHH-mm-ss

:添加 _meta 字段;

:确保 .aide/decisions/ 目录存在;

:写入 pending.json;
note right: 原子写入

stop
@enduml
```

### 3.2 保存决策结果

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:接收 DecideOutput 数据;

:读取 pending.json;

:提取 session_id;

:构造 DecisionRecord;
note right: input + output + completed_at

:写入 {session_id}.json;
note right: 原子写入

stop
@enduml
```

### 3.3 读取决策结果

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:检查 pending.json 是否存在;
if (存在?) then (是)
else (否)
  :返回错误: 未找到待定项数据;
  stop
endif

:读取 pending.json;

:提取 session_id;

:构造历史记录文件名;
note right: {session_id}.json

:检查历史记录是否存在;
if (存在?) then (是)
else (否)
  :返回错误: 尚无决策结果;
  stop
endif

:读取历史记录;

:返回 output.decisions;

stop
@enduml
```

## 四、并发与原子性

### 4.1 原子写入

为避免写入过程中断导致文件损坏，必须使用原子写入：

1. 写入临时文件：`{filename}.tmp`
2. 确保写入完成（fsync，如实现选择支持）
3. 原子重命名为目标文件

```
save_atomic(path: Path, data: dict) -> None:
    """
    原子写入 JSON 文件

    1. 序列化为 JSON 字符串
    2. 写入 {path}.tmp
    3. 重命名为 {path}
    """
```

### 4.2 并发控制

由于 aide decide 是单用户场景，且同一时间只有一个会话，通常不需要复杂的并发控制。

但为了安全，建议：
- 写入前检查文件是否被其他进程占用
- 使用文件锁（可选）

### 4.3 编码规范

- 文件编码：UTF-8
- 换行符：`\n`（Unix 风格）
- JSON 格式：缩进 2 空格，便于人工阅读

## 五、生命周期管理

### 5.1 文件生命周期

| 文件 | 创建时机 | 删除时机 |
|------|----------|----------|
| pending.json | `aide decide '<json>'` | 不自动删除，下次覆盖 |
| {session_id}.json | 用户提交决策 | 不自动删除 |

### 5.2 历史记录清理

历史记录不自动清理，由用户手动管理。

建议的清理策略（可作为后续功能）：
- 保留最近 N 条记录
- 保留最近 N 天的记录
- 提供 `aide decide clean` 命令

### 5.3 目录初始化

首次使用时，需要确保目录存在：

```
ensure_decisions_dir(root: Path) -> Path:
    """
    确保 .aide/decisions/ 目录存在

    1. 检查 .aide/ 是否存在
    2. 若不存在，返回错误（需要先执行 aide init）
    3. 创建 decisions/ 子目录（如不存在）
    4. 返回目录路径
    """
```

## 六、容错与恢复

### 6.1 JSON 解析失败

当文件存在但无法解析时：

```
✗ 无法解析 pending.json: <具体错误>
  建议: 文件可能已损坏，请重新执行 aide decide '<json>'
```

### 6.2 文件缺失

| 场景 | 错误信息 |
|------|----------|
| .aide/ 不存在 | `✗ .aide 目录不存在，请先执行 aide init` |
| pending.json 不存在 | `✗ 未找到待定项数据，请先执行 aide decide '<json>'` |
| 历史记录不存在 | `✗ 尚无决策结果，请等待用户完成操作` |

### 6.3 数据不一致

当 pending.json 和历史记录的 session_id 不匹配时：

```
✗ 决策结果已过期
  建议: pending.json 已被更新，请重新执行 aide decide '<json>'
```

## 七、方法签名原型

```
class DecideStorage:
    """决策数据存储管理"""

    root: Path                    # 项目根目录
    decisions_dir: Path           # .aide/decisions/ 目录
    pending_path: Path            # pending.json 路径

    def __init__(self, root: Path) -> None:
        """
        初始化存储管理器

        1. 设置路径
        2. 确保目录存在
        """

    def save_pending(self, data: DecideInput) -> str:
        """
        保存待定项数据

        1. 生成 session_id
        2. 添加 _meta 字段
        3. 原子写入 pending.json
        4. 返回 session_id
        """

    def load_pending(self) -> DecideInput | None:
        """
        加载待定项数据

        返回 DecideInput 或 None（文件不存在）
        """

    def get_session_id(self) -> str | None:
        """
        获取当前会话 ID

        从 pending.json 的 _meta.session_id 读取
        """

    def save_result(self, output: DecideOutput) -> None:
        """
        保存决策结果

        1. 读取 pending.json 获取 input 和 session_id
        2. 构造 DecisionRecord
        3. 原子写入 {session_id}.json
        """

    def load_result(self) -> DecideOutput | None:
        """
        加载决策结果

        1. 获取 session_id
        2. 读取 {session_id}.json
        3. 返回 output 部分
        """

    def has_pending(self) -> bool:
        """检查是否有待处理的待定项"""

    def has_result(self) -> bool:
        """检查是否有决策结果"""

    def _ensure_dir(self) -> None:
        """确保目录存在"""

    def _save_atomic(self, path: Path, data: dict) -> None:
        """原子写入 JSON 文件"""

    def _load_json(self, path: Path) -> dict | None:
        """加载 JSON 文件"""


# 数据类型定义（与 types.py 一致）

DecideInput:
    task: str
    source: str
    items: list[DecideItem]
    _meta: MetaInfo | None

MetaInfo:
    created_at: str
    session_id: str

DecideOutput:
    decisions: list[Decision]

DecisionRecord:
    input: DecideInput
    output: DecideOutput
    completed_at: str
```

## 八、与其他模块的关系

### 8.1 被调用方

| 调用方 | 调用的方法 |
|--------|------------|
| CLI (cmd_decide_submit) | save_pending() |
| CLI (cmd_decide_result) | load_result(), has_pending(), has_result() |
| Server (handle_get_items) | load_pending() |
| Server (handle_submit) | save_result() |

### 8.2 依赖

| 依赖项 | 用途 |
|--------|------|
| core/config.py | 读取项目根目录 |
| json | JSON 序列化/反序列化 |
| pathlib | 路径操作 |
| datetime | 时间戳生成 |

## 九、相关文档

- [decide 详细设计入口](README.md)
- [CLI 规格](cli.md)
- [HTTP 服务设计](server.md)
- [数据格式文档](../../formats/data.md)
