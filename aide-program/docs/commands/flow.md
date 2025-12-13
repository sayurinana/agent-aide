# aide flow 子命令设计文档

## 一、背景

### 1.1 解决的问题

| 问题 | 影响 |
|------|------|
| 状态记录分散 | 难以追踪任务进度 |
| git 提交不规范 | 提交信息不一致，难以追溯 |
| 流程跳跃 | 遗漏重要环节 |
| 返工无记录 | 问题原因和解决方案丢失 |

### 1.2 设计目标

提供**统一的进度追踪与版本控制**：
- 自动记录状态变化
- 自动执行 git add + commit
- 校验流程跳转合理性
- 环节特定行为（如 PlantUML 校验）

---

## 二、职责

### 2.1 做什么

1. 记录任务执行状态（环节、步骤）
2. 自动执行 git add . && git commit
3. 校验环节跳转是否符合预期流程
4. 在特定环节执行特定行为（如校验 PlantUML）

### 2.2 不做什么

- 不执行业务逻辑
- 不进行任务分析
- 不修改业务代码

---

## 三、接口规格

### 3.1 aide flow start

**用途**：开始新任务

**语法**：
```
aide flow start <环节名> "<总结>"
```

**参数**：

| 参数 | 说明 |
|------|------|
| `<环节名>` | task-optimize / flow-design |
| `<总结>` | 本次操作的简要说明 |

**输出**：
```
✓ 任务开始: <环节名>
```

### 3.2 aide flow next-step

**用途**：记录小步骤前进

**语法**：
```
aide flow next-step "<总结>"
```

**输出**：静默（成功无输出）

### 3.3 aide flow back-step

**用途**：记录小步骤回退

**语法**：
```
aide flow back-step "<原因>"
```

**输出**：静默（成功无输出）

### 3.4 aide flow next-part

**用途**：进入下一个大环节

**语法**：
```
aide flow next-part <环节名> "<总结>"
```

**输出**：
```
✓ 进入环节: <环节名>
```

**特殊行为**：
- 离开 `flow-design` 时：校验 PlantUML 语法，生成 PNG
- 进入 `docs` 时：提示更新 CHANGELOG
- 离开 `docs` 时：校验 CHANGELOG 是否已更新

### 3.5 aide flow back-part

**用途**：回退到之前的大环节

**语法**：
```
aide flow back-part <环节名> "<原因>"
```

**输出**：
```
⚠ 回退到环节: <环节名>
```

### 3.6 aide flow issue

**用途**：记录一般问题（不阻塞继续）

**语法**：
```
aide flow issue "<描述>"
```

**输出**：静默（成功无输出）

### 3.7 aide flow error

**用途**：记录严重错误（需要解决）

**语法**：
```
aide flow error "<描述>"
```

**输出**：
```
✗ 错误已记录: <描述>
```

---

## 四、业务流程

### 4.1 状态记录流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:接收命令和参数;

:读取当前状态;
note right: .aide/flow-status.json

:校验操作合法性;
if (合法?) then (是)
else (否)
  :输出错误信息;
  stop
endif

:更新状态文件;

:执行 git add .;

:生成提交信息;
note right: [aide] <环节>: <总结>

:执行 git commit;

if (环节特定行为?) then (是)
  :执行特定行为;
endif

:输出结果;

stop
@enduml
```

### 4.2 流程校验规则

```
@startuml
skinparam defaultFontName "PingFang SC"

[*] --> task_optimize : start (prep阶段)
[*] --> flow_design : start (exec阶段)

task_optimize --> task_optimize : next-step
task_optimize --> [*] : 完成prep

flow_design --> flow_design : next-step
flow_design --> impl : next-part

impl --> impl : next-step
impl --> flow_design : back-part
impl --> verify : next-part

verify --> verify : next-step
verify --> impl : back-part
verify --> docs : next-part

docs --> docs : next-step
docs --> verify : back-part
docs --> finish : next-part

finish --> finish : next-step
finish --> [*] : 完成exec

@enduml
```

**校验规则**：
- `next-part` 只能前进到相邻环节或回退
- 不允许跳过环节（如 flow-design → finish）
- `back-part` 可以回退到任意之前的环节

---

## 五、数据结构

### 5.1 状态文件格式

位置：`.aide/flow-status.json`

```
FlowStatus:
    task_id: str              # 任务标识（时间戳）
    current_phase: str        # 当前环节名
    current_step: int         # 当前步骤序号
    started_at: str           # 开始时间（ISO格式）
    history: list[HistoryEntry]  # 历史记录

HistoryEntry:
    timestamp: str            # 时间戳
    action: str               # 操作类型（start/next-step/next-part/...）
    phase: str                # 环节名
    step: int                 # 步骤序号
    summary: str              # 总结/原因
    git_commit: str | None    # git commit hash
```

### 5.2 方法签名原型

```
class FlowTracker:
    root: Path
    status_path: Path         # .aide/flow-status.json

    start(phase: str, summary: str) -> bool
        # 开始新任务

    next_step(summary: str) -> bool
        # 记录步骤前进

    back_step(reason: str) -> bool
        # 记录步骤回退

    next_part(phase: str, summary: str) -> bool
        # 进入下一环节

    back_part(phase: str, reason: str) -> bool
        # 回退到之前环节

    issue(description: str) -> bool
        # 记录一般问题

    error(description: str) -> bool
        # 记录严重错误

    _load_status() -> FlowStatus | None
        # 加载状态文件

    _save_status(status: FlowStatus) -> None
        # 保存状态文件

    _validate_transition(from_phase: str, to_phase: str) -> bool
        # 校验环节跳转

    _git_commit(message: str) -> str | None
        # 执行 git add + commit，返回 commit hash

    _run_phase_hooks(phase: str, entering: bool) -> None
        # 执行环节特定行为
```

### 5.3 Git 集成

```
class GitIntegration:
    root: Path

    add_all() -> bool
        # git add .

    commit(message: str) -> str | None
        # git commit -m "..."，返回 commit hash

    get_status() -> str
        # git status -sb
```

### 5.4 流程校验

```
class FlowValidator:
    PHASE_ORDER: list[str]    # 环节顺序定义
    VALID_TRANSITIONS: dict   # 有效跳转映射

    validate_start(phase: str) -> bool
        # 校验 start 操作

    validate_next_part(from_phase: str, to_phase: str) -> bool
        # 校验 next-part 操作

    validate_back_part(from_phase: str, to_phase: str) -> bool
        # 校验 back-part 操作
```

---

## 六、环节特定行为

| 触发时机 | 行为 |
|----------|------|
| 离开 flow-design | 校验 PlantUML 语法，生成 PNG |
| 进入 docs | 输出提示：请更新 CHANGELOG |
| 离开 docs | 校验 CHANGELOG 是否已更新 |

---

## 七、依赖

| 依赖项 | 类型 | 说明 |
|--------|------|------|
| ConfigManager | 内部模块 | 读取配置 |
| output | 内部模块 | 输出格式化 |
| git | 外部工具 | 版本控制 |
| plantuml | 外部工具 | 流程图生成（可选） |

---

## 八、被依赖

| 依赖方 | 说明 |
|--------|------|
| /aide:prep | 调用 flow start、next-step |
| /aide:exec | 调用 flow next-part、next-step、issue、error |

---

## 九、修改指南

### 9.1 修改环节定义

1. 更新本文档的流程校验规则图
2. 修改 `FlowValidator.PHASE_ORDER` 和 `VALID_TRANSITIONS`
3. 同步更新 [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)

### 9.2 添加环节特定行为

1. 在本文档添加行为说明
2. 在 `_run_phase_hooks()` 中实现
3. 更新相关 Command 设计文档

### 9.3 修改状态文件格式

1. 更新本文档的数据结构章节
2. 修改代码实现
3. 同步更新 [数据格式文档](../formats/data.md)

---

## 十、相关文档

- [program 导览](../README.md)
- [数据格式文档](../formats/data.md)
- [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)
- [/aide:prep 命令设计](../../../aide-marketplace/aide-plugin/docs/commands/prep.md)
- [/aide:exec 命令设计](../../../aide-marketplace/aide-plugin/docs/commands/exec.md)
