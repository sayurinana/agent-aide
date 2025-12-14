# aide flow CLI 规格

## 一、命令一览

`aide flow` 由若干动作型子命令组成（每次调用记录一次历史，并尝试执行一次 Git 自动提交）：

| 子命令 | 语法（API 约定） | 成功输出 | 主要用途 |
|---|---|---|---|
| start | `aide flow start <phase> "<summary>"` | 输出 `✓ 任务开始: <phase>` | 开始新任务（创建/重置状态） |
| next-step | `aide flow next-step "<summary>"` | 静默 | 记录一个小步骤完成 |
| back-step | `aide flow back-step "<reason>"` | 静默 | 记录一次小步骤回退 |
| next-part | `aide flow next-part <phase> "<summary>"` | 输出 `✓ 进入环节: <phase>` | 前进进入下一环节 |
| back-part | `aide flow back-part <phase> "<reason>"` | 输出 `⚠ 回退到环节: <phase>` | 回退到之前环节 |
| issue | `aide flow issue "<description>"` | 静默 | 记录一般问题（不阻塞继续） |
| error | `aide flow error "<description>"` | 输出 `✗ 错误已记录: <description>` | 记录严重错误（需要用户关注） |

> 说明：`<summary>/<reason>/<description>` 建议由调用方负责加引号，避免空格导致参数截断。

## 二、参数校验规则

### 2.1 phase 校验

- `<phase>` 必须是 `flow.phases` 列表中的一个元素
- `next-part` 的 `<phase>` 必须是当前环节的**相邻下一环节**
- `back-part` 的 `<phase>` 必须出现在当前环节之前（允许回退到任意之前环节）

细则见：`aide-program/docs/commands/flow/validation.md`

### 2.2 文本参数校验

- `<summary>/<reason>/<description>` 不能为空字符串
- 建议限制最大长度（例如 200~500 字符），超出时返回错误并提示调用方缩短（防止 commit message 过长）

## 三、输出规范（与现有 core/output 保持一致）

### 3.1 静默原则

- **无输出 = 正常完成**
- 仅在需要“阶段确认/回退提醒/错误可见化”时输出

### 3.2 固定前缀

沿用 `aide-program/docs/README.md` 的输出规范：

- 成功：`✓`
- 警告：`⚠`
- 失败：`✗`
- 信息：`→`（仅在需要解释下一步时使用）

### 3.3 错误信息要求

错误输出需要包含：

1. 失败原因（尽量具体到参数/文件/外部命令）
2. 建议下一步（如“请先运行 aide init”“请检查 flow.phases 配置”）

## 四、退出码（与主程序一致）

| 退出码 | 含义 |
|---:|---|
| 0 | 成功（含“无变更可提交”的情况） |
| 1 | 失败（参数校验失败、流程校验失败、关键 Hook 失败、Git 必须但执行失败等） |

> 说明：`aide flow error` 的 “✗” 属于**业务语义提示**；只要本次记录与（可选的）Git 操作完成，仍应返回 0。

## 五、典型调用序列（契约）

### 5.1 prep 阶段（/aide:prep）

- `aide flow start task-optimize "开始任务准备: <任务简述>"`
- 若干次 `aide flow next-step "<...>"`

### 5.2 exec 阶段（/aide:exec）

- `aide flow next-part flow-design "进入流程设计环节"`
- 若干次 `aide flow next-step "<...>"`
- `aide flow next-part impl "..."`
- `aide flow next-part verify "..."`
- `aide flow next-part docs "..."`
- `aide flow next-part finish "..."`

> 注意：exec 使用 `next-part` 进入 `flow-design`，意味着 prep 与 exec 共享同一个任务状态文件。
