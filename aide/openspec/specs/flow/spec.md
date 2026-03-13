# flow Specification

## Purpose
TBD - created by archiving change rewrite-in-rust. Update Purpose after archive.
## Requirements
### Requirement: 流程状态数据结构

系统 SHALL 使用以下 JSON 结构存储流程状态（`.aide/flow-status.json`）：

```
FlowStatus {
  task_id: string          // 格式 YYYY-MM-DDTHH-MM-SS
  current_phase: string    // 当前环节名称
  current_step: number     // 步骤计数
  started_at: string       // ISO 8601 带时区
  history: HistoryEntry[]
  source_branch?: string   // 任务开始时的 git 分支
  start_commit?: string    // 任务开始时的 git 提交
  task_branch?: string     // 创建的任务分支名称
}

HistoryEntry {
  timestamp: string        // ISO 8601
  action: string           // start|next-step|back-step|next-part|back-part|issue|error
  phase: string
  step: number
  summary: string
  git_commit?: string      // 提交哈希（无变更时为 null）
}
```

#### Scenario: 任务 ID 格式
- **WHEN** 创建新任务
- **THEN** task_id 格式为 `YYYY-MM-DDTHH-MM-SS`（本地时间，时间分隔符用 `-`）

#### Scenario: 时间戳格式
- **WHEN** 记录历史条目
- **THEN** timestamp 为 ISO 8601 格式带时区（如 `2025-01-15T10:30:00+08:00`）

### Requirement: 开始任务

`aide flow start <phase> "<summary>"` SHALL：
1. 验证 phase 存在于 `flow.phases` 配置中
2. 归档现有活跃任务（如有）
3. 创建 git 分支 `aide/NNN`（零补齐3位）
4. 在 `.aide/branches.json` 中记录分支信息
5. 创建 FlowStatus 写入 `.aide/flow-status.json`
6. 执行 `git add -A --exclude="*.lock"` + `git commit`

输出 `✓ 任务开始: <phase> (分支: aide/NNN)`。

#### Scenario: 正常开始
- **WHEN** 运行 `aide flow start impl "实现核心功能"`
- **AND** `impl` 存在于 flow.phases 中
- **THEN** 创建分支 `aide/001`（首次）
- **AND** 创建 flow-status.json
- **AND** 输出 `✓ 任务开始: impl (分支: aide/001)`

#### Scenario: 无效环节
- **WHEN** 运行 `aide flow start nonexistent "test"`
- **AND** `nonexistent` 不在 flow.phases 中
- **THEN** 输出错误信息

#### Scenario: 覆盖现有任务
- **WHEN** 已存在活跃任务
- **AND** 运行 `aide flow start impl "新任务"`
- **THEN** 将现有任务归档到 `.aide/logs/flow-status.{task_id}.json`
- **AND** 创建新任务

### Requirement: 步骤前进

`aide flow next-step "<summary>"` SHALL：
1. 递增 current_step
2. 添加 action=next-step 的历史条目
3. 执行 git add + commit，消息格式 `[aide] <phase>: <summary>`

#### Scenario: 正常步骤前进
- **WHEN** 运行 `aide flow next-step "完成数据模型"`
- **AND** 当前环节为 impl，步骤为 3
- **THEN** 步骤变为 4
- **AND** git 提交消息为 `[aide] impl: 完成数据模型`

#### Scenario: 无活跃任务
- **WHEN** 没有活跃任务时运行 `aide flow next-step "test"`
- **THEN** 输出错误信息

### Requirement: 步骤回退

`aide flow back-step "<reason>"` SHALL：
1. 递增 current_step
2. 添加 action=back-step 的历史条目
3. 执行 git commit，消息格式 `[aide] <phase> back-step: <reason>`

#### Scenario: 正常步骤回退
- **WHEN** 运行 `aide flow back-step "发现接口设计有问题"`
- **THEN** git 提交消息为 `[aide] impl back-step: 发现接口设计有问题`

### Requirement: 环节前进

`aide flow next-part <phase> "<summary>"` SHALL：
1. 验证目标环节是当前环节的下一个相邻环节（不可跳跃）
2. 执行 pre-commit hooks
3. 更新 current_phase、递增 current_step
4. 执行 git add + commit
5. 执行 post-commit hooks
6. 如果进入 `finish` 环节，执行分支合并和清理

输出 `✓ 进入环节: <phase>`。

#### Scenario: 正常环节前进
- **WHEN** 当前环节为 impl，运行 `aide flow next-part verify "实现完成"`
- **AND** verify 是 impl 的下一个环节
- **THEN** 当前环节变为 verify
- **AND** 输出 `✓ 进入环节: verify`

#### Scenario: 跳跃环节被拒绝
- **WHEN** 当前环节为 impl，运行 `aide flow next-part docs "跳过验证"`
- **AND** docs 不是 impl 的下一个相邻环节
- **THEN** 输出错误信息

#### Scenario: 进入 finish 环节
- **WHEN** 运行 `aide flow next-part finish "完成"`
- **THEN** 执行分支合并流程
- **AND** 清理任务临时文件

### Requirement: 环节回退

`aide flow back-part <phase> "<reason>"` SHALL 实现两阶段确认：
1. 第一阶段：验证目标环节在当前环节之前，生成 12 字符十六进制确认 key，保存到 `.aide/back-confirm-state.json`，输出确认指令
2. 第二阶段：`aide flow back-confirm --key <key>` 验证 key 匹配后执行实际回退

#### Scenario: 请求回退
- **WHEN** 当前环节为 verify，运行 `aide flow back-part impl "需要修改实现"`
- **THEN** 生成确认 key（12 字符十六进制）
- **AND** 保存 back-confirm-state.json
- **AND** 输出确认指令

#### Scenario: 确认回退
- **WHEN** 运行 `aide flow back-confirm --key abc123def456`
- **AND** key 与 back-confirm-state.json 中的 pending_key 匹配
- **THEN** 当前环节变为目标环节
- **AND** 删除 back-confirm-state.json

#### Scenario: 错误的确认 key
- **WHEN** 运行 `aide flow back-confirm --key wrong_key`
- **THEN** 输出错误信息

### Requirement: 记录问题和错误

`aide flow issue "<description>"` SHALL 记录非阻塞问题。
`aide flow error "<description>"` SHALL 记录阻塞错误。

两者都递增 step 并执行 git commit。

issue 提交消息格式：`[aide] <phase> issue: <description>`
error 提交消息格式：`[aide] <phase> error: <description>`

error 额外输出 `✗ 错误已记录: <description>`。

#### Scenario: 记录问题
- **WHEN** 运行 `aide flow issue "性能有待优化"`
- **THEN** git 提交消息为 `[aide] impl issue: 性能有待优化`
- **AND** 无标准输出

#### Scenario: 记录错误
- **WHEN** 运行 `aide flow error "编译失败"`
- **THEN** git 提交消息为 `[aide] impl error: 编译失败`
- **AND** 输出 `✗ 错误已记录: 编译失败`

### Requirement: 查看状态

`aide flow status` SHALL 显示当前活跃任务信息：

```
→ 任务 ID: <task_id>
→ 环节: <current_phase>
→ 步骤: <current_step>
→ 开始时间: <started_at>
→ 最新操作: <summary>
→ 操作时间: <timestamp>
→ Git 提交: <commit_short>
```

无活跃任务时输出 `→ 当前无活跃任务`。

#### Scenario: 有活跃任务
- **WHEN** 存在活跃任务
- **THEN** 输出任务详情

#### Scenario: 无活跃任务
- **WHEN** 不存在活跃任务
- **THEN** 输出 `→ 当前无活跃任务`

### Requirement: 列出任务

`aide flow list` SHALL 按时间倒序列出所有任务（当前 + 已归档），格式：

```
→ 任务列表:
  *[1] <task_id> (<phase>) <summary>
   [2] <task_id> (<phase>) <summary>
→ 提示: 使用 aide flow show <task_id> 查看详细状态
```

`*` 标记当前活跃任务。

#### Scenario: 列出多个任务
- **WHEN** 存在一个活跃任务和两个归档任务
- **THEN** 列出三个任务，活跃任务带 `*` 标记

### Requirement: 查看任务详情

`aide flow show <task_id>` SHALL 显示指定任务的完整历史记录：

```
→ 任务 ID: <task_id>
→ 当前环节: <phase>
→ 当前步骤: <step>
→ 开始时间: <started_at>
→
→ 历史记录:
  [phase] <summary> [commit_short]
         <timestamp> (<action>)
```

#### Scenario: 查看活跃任务
- **WHEN** 运行 `aide flow show <当前任务ID>`
- **THEN** 显示完整任务信息和历史记录

#### Scenario: 查看归档任务
- **WHEN** 运行 `aide flow show <归档任务ID>`
- **THEN** 从 `.aide/logs/` 加载并显示任务信息

### Requirement: 强制清理

`aide flow clean` SHALL 强制清理当前任务：
1. 如果工作目录有未提交变更，自动提交
2. 执行分支合并（正常或临时分支策略）
3. 切换回源分支
4. 清理任务临时文件

#### Scenario: 正常清理
- **WHEN** 运行 `aide flow clean`
- **AND** 源分支无新提交
- **THEN** 正常合并并清理

#### Scenario: 源分支有新提交
- **WHEN** 运行 `aide flow clean`
- **AND** 源分支在任务创建后有新提交
- **THEN** 使用临时分支策略合并

### Requirement: 环节校验规则

系统 SHALL 强制执行以下环节校验规则：
- phases 列表不可为空
- 不允许重复的环节名称
- `next-part` 只能前进到下一个相邻环节
- `back-part` 可以回退到任何更早的环节
- 环节名称去除首尾空白后不可为空

#### Scenario: phases 列表验证
- **WHEN** flow.phases 配置为空数组
- **THEN** 操作失败并提示配置错误

### Requirement: Git 集成

系统 SHALL 在 flow 操作中执行以下 Git 命令：
- `git add -A --exclude="*.lock"` — 暂存变更（排除锁文件）
- `git commit -m "<message>"` — 提交
- `git rev-parse HEAD` — 获取提交哈希
- `git status --porcelain` — 检查变更状态
- `git rev-parse --abbrev-ref HEAD` — 获取当前分支名

提交消息格式：
- start: `[aide] <phase>: <summary>`
- next-step: `[aide] <phase>: <summary>`
- back-step: `[aide] <phase> back-step: <reason>`
- next-part: `[aide] <phase>: <summary>`
- back-part: `[aide] <phase> back-part: <reason>`
- issue: `[aide] <phase> issue: <description>`
- error: `[aide] <phase> error: <description>`

如果 `git add` 后无实际变更，git commit 可静默跳过（不报错）。

#### Scenario: 有变更时提交
- **WHEN** 执行 flow 操作且工作目录有变更
- **THEN** 执行 git add + commit

#### Scenario: 无变更时跳过
- **WHEN** 执行 flow 操作但工作目录无变更
- **THEN** 跳过 git commit，不报错
- **AND** 历史条目的 git_commit 为 null

### Requirement: 分支管理

系统 SHALL 管理任务分支的完整生命周期：

**创建**：
- 分支命名格式 `aide/NNN`（3 位零补齐）
- 编号从 `.aide/branches.json` 的 `next_number` 获取并递增
- 记录 source_branch、start_commit、task_id、task_summary

**合并（正常）**：当源分支无新提交时
1. 记录 end_commit 和 finished_at
2. 清理任务文件
3. 切换到源分支
4. `git merge --squash` 合并任务分支
5. 提交合并，消息格式 `完成：<branch> - <summary>`

**合并（临时分支）**：当源分支有新提交时
1. 创建临时分支 `<task_branch>-merge` 从 start_commit
2. 在临时分支上 squash merge 任务分支
3. 标记状态为 `merged-to-temp`
4. 输出警告提示用户手动处理

**状态**：`active` | `finished` | `merged-to-temp` | `force-cleaned` | `force-cleaned-to-temp`

#### Scenario: 首次创建分支
- **WHEN** branches.json 不存在或 next_number 为 1
- **THEN** 创建分支 `aide/001`

#### Scenario: 正常合并
- **WHEN** 进入 finish 环节
- **AND** 源分支自 start_commit 后无新提交
- **THEN** squash merge 到源分支
- **AND** 分支状态变为 `finished`

#### Scenario: 临时分支合并
- **WHEN** 进入 finish 环节
- **AND** 源分支有新提交
- **THEN** 创建临时分支并合并
- **AND** 分支状态变为 `merged-to-temp`
- **AND** 输出用户操作提示

### Requirement: 状态文件锁

系统 SHALL 使用文件锁保护 `.aide/flow-status.json` 的读写：
- 锁文件路径：`.aide/flow-status.lock`
- 锁超时时间：3 秒，0.2 秒轮询间隔
- 锁内容：当前进程 PID
- 操作完成后在 finally/drop 中释放锁

#### Scenario: 并发访问
- **WHEN** 两个 aide 进程同时尝试更新 flow-status.json
- **THEN** 后到的进程等待锁释放（最多 3 秒）

#### Scenario: 锁超时
- **WHEN** 锁在 3 秒内未能获取
- **THEN** 操作失败并输出错误信息

### Requirement: PlantUML Hook

系统 SHALL 在离开 `flow-design` 环节时执行 PlantUML 验证 hook：
1. 收集 `.puml` 和 `.plantuml` 文件（从 `flow.diagram_path` 配置目录及 `docs/`、`discuss/` 目录）
2. 解析 PlantUML 命令路径，按以下优先级查找：
   a. 全局配置的自包含可执行程序：`{global_aide_dir}/{plantuml.install_path}/plantuml/bin/plantuml`
   b. 系统 PATH 中的 `plantuml` 命令
3. 对每个文件执行语法检查：`plantuml -checkonly <file>`
4. 对每个文件生成 PNG：`plantuml -tpng <file>`
5. 检查失败时阻止环节跳转

#### Scenario: PlantUML 验证通过
- **WHEN** 离开 flow-design 环节
- **AND** 所有 .puml 文件语法正确
- **THEN** 生成 PNG 文件
- **AND** 允许环节跳转

#### Scenario: PlantUML 语法错误
- **WHEN** 离开 flow-design 环节
- **AND** 某个 .puml 文件有语法错误
- **THEN** 输出错误信息
- **AND** 阻止环节跳转

#### Scenario: PlantUML 不可用
- **WHEN** 系统未安装 PlantUML（自包含程序和系统命令均不可用）
- **THEN** 输出警告信息但不阻止操作

### Requirement: CHANGELOG Hook

系统 SHALL 在 `docs` 环节执行 CHANGELOG 相关 hooks：

**进入 docs 环节（post-commit）**：输出 `→ 请更新 CHANGELOG.md`

**离开 docs 环节（pre-commit）**：
1. 检查 CHANGELOG.md 文件存在
2. 检查 CHANGELOG.md 在 docs 环节中被修改过（通过 git 历史检查）
3. 如果未修改，阻止环节跳转

#### Scenario: CHANGELOG 已更新
- **WHEN** 离开 docs 环节
- **AND** CHANGELOG.md 在 docs 环节中有提交记录
- **THEN** 允许环节跳转

#### Scenario: CHANGELOG 未更新
- **WHEN** 离开 docs 环节
- **AND** CHANGELOG.md 未被修改
- **THEN** 输出错误信息
- **AND** 阻止环节跳转

### Requirement: Finish 清理 Hook

系统 SHALL 在进入 `finish` 环节时执行清理 hook：
- 删除 `task.plans_path` 目录（默认 `.aide/task-plans/`）下的所有文件

#### Scenario: 清理任务计划文件
- **WHEN** 进入 finish 环节
- **THEN** 删除 `.aide/task-plans/` 目录下的所有文件

### Requirement: 任务完成清理

系统 SHALL 在任务完成时清理以下文件：
1. `.aide/` 下的所有 `.lock` 文件
2. 任务 spec 文件（`config.task.spec`）
3. 待定项文件（`.aide/decisions/` 下的文件）
4. `pending-items.json`
5. 图表文件（`.puml`、`.plantuml`、`.png`）
6. flow-status.json 备份到 logs 目录

#### Scenario: 完成任务后清理
- **WHEN** 任务合并完成后
- **THEN** 上述临时文件被清理
- **AND** flow-status.json 归档到 `.aide/logs/`

