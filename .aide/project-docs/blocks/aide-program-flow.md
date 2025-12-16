# aide-program-flow

> 路径：aide-program/aide/flow/
> 最后更新：2025-12-17

## 概述

进度追踪和流程控制模块，负责管理任务执行流程、Git 集成和环节钩子。采用状态机模式，支持环节跳转校验、自动 Git 提交和 PlantUML 流程图验证。

## 目录结构

```
aide-program/aide/flow/
├── __init__.py              模块初始化
├── types.py                 数据结构定义
├── errors.py                错误类型
├── tracker.py               流程追踪器
├── storage.py               状态文件读写
├── git.py                   Git 操作封装
├── hooks.py                 环节钩子
├── validator.py             流程校验
└── utils.py                 工具函数
```

## 文件清单

| 文件 | 类型 | 说明 |
|------|------|------|
| __init__.py | 源码 | 模块初始化 |
| types.py | 源码 | FlowStatus、HistoryEntry 数据类 |
| errors.py | 源码 | FlowError 异常类 |
| tracker.py | 源码 | FlowTracker 核心类，编排流程动作 |
| storage.py | 源码 | FlowStorage 类，状态文件读写和归档 |
| git.py | 源码 | GitIntegration 类，Git 操作封装 |
| hooks.py | 源码 | 环节钩子（PlantUML、CHANGELOG） |
| validator.py | 源码 | FlowValidator 类，流程校验逻辑 |
| utils.py | 源码 | 时间戳和文本处理工具 |

## 核心组件

### FlowTracker 类

- **职责**：编排一次 flow 动作（校验 → hooks → 落盘 → git → 输出）
- **位置**：`aide/flow/tracker.py:20`
- **关键方法**：
  - `start(phase, summary)` - 开始新任务
  - `next_step(summary)` - 记录步骤前进
  - `back_step(reason)` - 记录步骤回退
  - `next_part(phase, summary)` - 进入下一环节
  - `back_part(phase, reason)` - 回退到之前环节
  - `issue(description)` - 记录一般问题
  - `error(description)` - 记录严重错误
  - `_run()` - 核心执行逻辑
  - `_apply_action()` - 应用动作，更新状态
  - `_do_git_commit()` - 执行 Git 提交

### FlowStorage 类

- **职责**：状态文件的读写、锁和归档
- **位置**：`aide/flow/storage.py:16`
- **关键方法**：
  - `ensure_ready()` - 确保 .aide 目录存在
  - `lock()` - 文件锁上下文管理器
  - `load_status()` - 加载当前状态
  - `save_status()` - 保存状态（原子写入）
  - `archive_existing_status()` - 归档旧状态
  - `list_all_tasks()` - 列出所有任务
  - `load_task_by_id()` - 根据 ID 加载任务

### GitIntegration 类

- **职责**：封装 Git 操作
- **位置**：`aide/flow/git.py:12`
- **关键方法**：
  - `ensure_available()` - 检查 git 命令可用
  - `ensure_repo()` - 检查是否在 git 仓库中
  - `add_all()` - git add .
  - `commit(message)` - git commit
  - `rev_parse_head()` - 获取 HEAD commit hash
  - `status_porcelain(path)` - 检查文件状态
  - `commit_touches_path()` - 检查提交是否修改指定文件

### FlowValidator 类

- **职责**：流程校验，验证环节跳转合法性
- **位置**：`aide/flow/validator.py:8`
- **关键方法**：
  - `validate_phase_exists(phase)` - 验证环节存在
  - `validate_start(phase)` - 验证开始环节
  - `validate_next_part(from, to)` - 验证前进跳转（只能相邻）
  - `validate_back_part(from, to)` - 验证回退跳转（只能向前）

### FlowStatus 数据类

- **职责**：流程状态封装
- **位置**：`aide/flow/types.py:50`
- **字段**：
  - `task_id: str` - 任务 ID（时间戳格式）
  - `current_phase: str` - 当前环节
  - `current_step: int` - 当前步骤号
  - `started_at: str` - 开始时间（ISO 格式）
  - `history: list[HistoryEntry]` - 历史记录

### HistoryEntry 数据类

- **职责**：历史条目封装
- **位置**：`aide/flow/types.py:10`
- **字段**：
  - `timestamp: str` - 时间戳
  - `action: str` - 动作类型
  - `phase: str` - 环节名
  - `step: int` - 步骤号
  - `summary: str` - 摘要
  - `git_commit: str | None` - Git 提交 hash

## 环节钩子

### PlantUML 钩子

- **触发时机**：离开 flow-design 环节时（next-part/back-part）
- **位置**：`aide/flow/hooks.py:61`
- **功能**：
  1. 校验 .puml/.plantuml 文件语法
  2. 生成 PNG 图片
  3. 检查目录：.aide/diagrams、docs、discuss

### CHANGELOG 钩子

- **触发时机**：离开 docs 环节时
- **位置**：`aide/flow/hooks.py:126`
- **功能**：验证 CHANGELOG.md 已更新

## 接口说明

### 流程追踪 API

```python
from aide.flow.tracker import FlowTracker
from aide.core.config import ConfigManager

cfg = ConfigManager(Path.cwd())
tracker = FlowTracker(Path.cwd(), cfg)

# 开始新任务
tracker.start("task-optimize", "开始任务准备")

# 步骤前进
tracker.next_step("完成数据库设计")

# 进入下一环节
tracker.next_part("impl", "进入实现环节")

# 回退环节
tracker.back_part("flow-design", "发现设计遗漏")
```

### 默认环节列表

```python
DEFAULT_PHASES = ["task-optimize", "flow-design", "impl", "verify", "docs", "finish"]
```

## 依赖关系

- 依赖：aide/core（ConfigManager、output）
- 被依赖：aide/main.py

## 注意事项

- 每次 flow 操作都会自动执行 git add + commit
- 状态文件使用文件锁防止并发冲突
- next-part 只能前进到相邻环节
- back-part 可以回退到任意之前的环节
- 离开 flow-design 时会自动校验和生成 PlantUML 图
- 离开 docs 时会验证 CHANGELOG.md 已更新
