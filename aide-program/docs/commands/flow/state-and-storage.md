# 状态模型与存储设计

## 一、状态文件（源规范）

状态文件的数据结构以 `aide-program/docs/formats/data.md` 的“流程状态格式”为准：

- 位置：`.aide/flow-status.json`
- 顶层结构：`FlowStatus`
- 历史条目：`HistoryEntry`

本文档补充**字段语义**与**落盘/并发/容错**约定。

## 二、字段语义（实现必须遵守）

### 2.1 task_id

- 生成时机：`aide flow start` 调用时生成
- 语义：标识一次“任务”的全生命周期（覆盖 prep + exec）
- 格式：与 `aide-program/docs/formats/data.md` 的时间戳约定一致

### 2.2 current_phase

- 语义：当前所处环节名称
- 取值：必须在 `flow.phases` 中
- 变更来源：`start`、`next-part`、`back-part`

### 2.3 current_step

为避免历史回溯混乱，建议将 `current_step` 定义为**单调递增的记录序号**：

- `start` 产生第一条记录，`current_step = 1`
- 每次成功记录一次动作（start/next-step/back-step/next-part/back-part/issue/error），`current_step += 1`
- 不做递减（回退动作使用 action=back-step/back-part 表达）

> 如果团队坚持“回退需要 step-1”，需先在全体文档中统一，并评估历史序号重复/倒序对调试的影响。

### 2.4 history

每次 flow 调用都会追加一个 HistoryEntry：

- `timestamp`：本次记录时间（ISO 8601）
- `action`：动作类型（与 data.md 枚举一致）
- `phase`：本次动作完成后的 current_phase
- `step`：本次动作完成后的 current_step
- `summary`：摘要/原因/描述
- `git_commit`：若本次产生了 commit，则为 commit hash，否则为空

约束（建议实现中校验）：

- `history[-1].phase == current_phase`
- `history[-1].step == current_step`
- `history` 按追加顺序即时间顺序

## 三、存储生命周期

### 3.1 start 行为（归档旧状态）

当执行 `aide flow start` 时：

- 若 `.aide/flow-status.json` 不存在：直接创建新状态
- 若已存在：应先归档旧文件，避免无提示覆盖导致历史丢失

推荐归档位置（本地数据，不纳入 git）：

- `.aide/logs/flow-status.<old_task_id>.json`

### 3.2 其它动作行为

除 `start` 外的命令要求：

- 状态文件必须存在，否则返回错误并建议先运行 `aide flow start ...` 或执行 `/aide:prep`

## 四、并发与原子性（落盘必须可靠）

### 4.1 互斥（锁）

为避免并发调用导致状态文件损坏，`aide flow` 在读写前必须获得独占锁。

推荐方案（跨平台、实现简单）：

- 锁文件：`.aide/flow-status.lock`
- 加锁：以“创建新文件（独占）”方式获取
- 释放：删除锁文件

约定：

- 获取锁失败时：短暂重试（例如总计 2~3 秒），仍失败则返回错误 `✗ 状态文件被占用`
- 异常退出时：应尽力清理锁（并允许人工删除）

### 4.2 原子写入

必须避免写一半导致 JSON 损坏：

1. 写入临时文件：`.aide/flow-status.json.tmp`
2. fsync（如实现选择支持）
3. 原子替换为目标文件：`.aide/flow-status.json`

### 4.3 编码与换行

- UTF-8 编码
- 换行统一 `\n`
- JSON 建议使用稳定排序/缩进策略（便于人工阅读与 diff），但不要依赖其格式做解析

## 五、容错与恢复策略

### 5.1 JSON 解析失败

当状态文件存在但无法解析：

- 输出错误并提示用户手动处理（例如备份后删除该文件，再重新 start）
- 不建议在未确认的情况下自动“修复”或丢弃字段

### 5.2 目录缺失

若 `.aide/` 不存在：

- 返回错误并提示先执行 `aide init`

## 六、与 Git 的关系（重要）

`.aide/` 默认在仓库 `.gitignore` 内：

- 状态文件用于本地追踪，不要求进入提交
- Git 提交用于记录真实产出（代码/文档等）
- 若团队希望将状态文件纳入版本库，应显式调整 `.gitignore`，并在 Git 策略上达成一致
