# aide-program-flow

> 路径：aide-program/aide/flow/
> 最后更新：2025-12-16

## 概述

进度追踪模块，提供任务流程管理、Git 自动提交、环节校验和 Hooks 支持。

## 文件清单

| 文件 | 说明 |
|------|------|
| `__init__.py` | 模块初始化 |
| `tracker.py` | FlowTracker 主逻辑（~220 行） |
| `storage.py` | 状态文件读写（~147 行） |
| `types.py` | 数据结构定义（~103 行） |
| `validator.py` | 环节校验器（~50 行） |
| `git.py` | Git 集成（~75 行） |
| `hooks.py` | pre/post commit 钩子（~125 行） |
| `errors.py` | 自定义异常 |
| `utils.py` | 工具函数 |

## 核心组件

### FlowTracker

- **职责**：编排一次 flow 动作（校验 → hooks → 落盘 → git → 输出）
- **位置**：`tracker.py:20`
- **关键方法**：
  - `start(phase, summary)` - 开始新任务
  - `next_step(summary)` - 记录步骤前进
  - `back_step(reason)` - 记录步骤回退
  - `next_part(phase, summary)` - 进入下一环节
  - `back_part(phase, reason)` - 回退到之前环节
  - `issue(description)` - 记录一般问题
  - `error(description)` - 记录严重错误
  - `_apply_action()` - 应用动作，生成新状态和 commit 消息
  - `_do_git_commit()` - 执行 git 操作并更新 commit hash

### FlowStorage

- **职责**：状态文件的读写、锁定和归档
- **位置**：`storage.py:16`
- **关键方法**：
  - `lock()` - 上下文管理器，获取文件锁
  - `load_status()` - 加载当前任务状态
  - `save_status(status)` - 保存状态（原子写入）
  - `archive_existing_status()` - 归档旧状态到 logs/
  - `list_all_tasks()` - 列出所有任务
  - `load_task_by_id(task_id)` - 按 ID 加载任务

### FlowValidator

- **职责**：校验环节跳转合法性
- **位置**：`validator.py`
- **校验规则**：
  - `next_part`: 只能跳转到相邻的下一环节
  - `back_part`: 可以回退到任意之前的环节
  - `start`: 必须从有效环节开始

### 数据结构

- `FlowStatus` - 任务状态（task_id, current_phase, current_step, history）
- `HistoryEntry` - 历史条目（timestamp, action, phase, step, summary, git_commit）

## 接口说明

```python
# CLI 入口
aide flow start <phase> "<summary>"    # 开始任务
aide flow next-step "<summary>"        # 步骤前进
aide flow back-step "<reason>"         # 步骤回退
aide flow next-part <phase> "<summary>" # 进入下一环节
aide flow back-part <phase> "<reason>"  # 回退环节
aide flow issue "<description>"         # 记录问题
aide flow error "<description>"         # 记录错误
aide flow status                        # 查看当前状态
aide flow list                          # 列出所有任务
aide flow show <task_id>                # 查看任务详情
```

## Git 集成

**执行顺序**（已优化）：
1. 运行 pre_commit_hooks
2. 更新 FlowStatus（内存）
3. 保存状态到磁盘（flow-status.json）
4. `git add .`
5. `git commit -m "[aide] <phase>: <summary>"`
6. 更新 commit hash 到状态文件

> **关键改进**：状态文件先保存再执行 git 操作，确保 flow-status.json 的更新包含在 commit 中

提交信息格式：
- 正常操作：`[aide] impl: 完成数据库模型设计`
- 问题记录：`[aide] impl issue: 测试覆盖率低`
- 错误记录：`[aide] impl error: 数据库连接失败`

## 依赖关系

- 依赖：core（output, config）
- 被依赖：main.py

## 注意事项

- 状态文件使用文件锁防止并发写入
- 归档文件保存在 `.aide/logs/` 目录
- Hooks 支持 PlantUML 自动校验和构建
- Git 提交在状态保存之后执行，确保 .aide 目录变更被包含
