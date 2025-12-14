# Git 集成设计（GitIntegration）

## 一、目标与原则

### 1.1 目标

- 让调用方无需手动执行 `git add/commit` 也能得到规范提交
- 将“进度节点”与“提交记录”关联：在状态历史中记录 `git_commit`（可为空）

### 1.2 原则

- **静默即成功**：Git 正常执行时尽量不输出额外信息
- **.aide 默认不进提交**：Git 记录的是代码/文档等实际产出，状态文件仅做本地追踪
- **无变更可提交 ≠ 失败**：当工作区没有可提交变更时，本次记录仍然成功，`git_commit = null`

## 二、提交信息生成规则（必须一致）

提交信息格式以 `aide-program/docs/formats/data.md` 的“Git 提交信息格式”为准。

核心模板：

- 普通：`[aide] <phase>: <summary>`
- issue：`[aide] <phase> issue: <description>`
- error：`[aide] <phase> error: <description>`
- back-step：`[aide] <phase> back-step: <reason>`
- back-part：`[aide] <phase> back-part: <reason>`

> 说明：`phase` 使用“动作完成后的 current_phase”，以保证 status 与 commit message 一致。

## 三、暂存（add）策略

默认策略（与现有文档保持一致）：

- 执行 `git add .`（由 `.gitignore` 控制忽略项）

可选增强（建议实现时评估，避免误提交）：

- 仅暂存已跟踪文件：等价于“只更新 tracked 的变更”
- 在提交前检查 `git status`，对明显异常（如大量生成文件）给出 `⚠` 提示

## 四、事务边界（状态与 Git 的一致性）

### 4.1 推荐边界

一次 `aide flow` 调用视为一个“原子动作”：

- 校验失败/关键 Hook 失败：不应推进状态，也不应提交
- Git 必须但执行失败：不应推进状态（避免“状态已前进但提交缺失”）
- 无变更可提交：允许推进状态，`git_commit = null`

### 4.2 “无变更可提交”的判定

当出现“nothing to commit / working tree clean”等语义时：

- 视为成功，不返回错误
- 仍写入状态历史，但 `git_commit` 为空

## 五、失败与降级策略

### 5.1 非 git 仓库

若当前目录不在 git 仓库内：

- **默认建议：失败**（提示用户初始化 git 或切换到正确目录）
- 可配置降级：仅记录状态，不做 git（见 5.3）

### 5.2 git 命令不可用/异常

若 git 不存在或执行报错：

- 默认失败，并输出具体错误与建议

### 5.3 可配置项（建议加入 flow 配置）

为满足不同团队习惯，建议在 `flow` 下增加可选配置（若不实现则使用默认行为）：

| 配置键（建议） | 类型 | 默认值（建议） | 说明 |
|---|---:|---:|---|
| `flow.git.enabled` | bool | true | 是否启用 Git 集成 |
| `flow.git.required` | bool | true | 启用后是否“必须成功”，否则降级为仅记录 |
| `flow.git.allow_empty_commit` | bool | false | 是否允许创建空提交（用于记录纯流程节点） |
| `flow.git.add_strategy` | string | `"dot"` | 暂存策略：`dot`（git add .）/ 其它策略（可扩展） |

> 若 `allow_empty_commit = true`，则“无变更”场景也会产生 commit；需要评估对仓库历史的影响。

## 六、与 Hooks 的顺序关系

会产生/修改文件的钩子（如 PlantUML 生成 PNG）必须在 Git add/commit 之前执行，否则生成物不会进入本次提交。

详细触发点见：`aide-program/docs/commands/flow/hooks.md`
