# 验证清单（实现完成后的自检）

本清单用于在实现 `aide flow` 后进行验证，确保行为符合设计文档与 plugin 契约。

## 一、准备条件

- 当前目录为一个 git 仓库（用于验证 Git 集成相关条目）
- 已执行 `aide init`，确保 `.aide/` 与 `config.toml` 存在
- `flow.phases` 使用默认值或明确配置（见 `aide-program/docs/formats/config.md`）

## 二、核心用例

### 2.1 start 新任务（无历史）

步骤：

1. 运行 `aide flow start task-optimize "..."`

期望：

- `.aide/flow-status.json` 被创建且可解析
- `current_phase == "task-optimize"`
- `history` 至少 1 条记录，`action == "start"`
- 若仓库无变更：不视为失败，`git_commit` 为空

### 2.2 next-step 记录步骤（静默）

步骤：

1. 运行 `aide flow next-step "..."`

期望：

- 命令成功时无输出
- `history` 追加一条 `action == "next-step"`

### 2.3 next-part 正常前进

步骤：

1. 从 `task-optimize` 执行 `aide flow next-part flow-design "..."`

期望：

- 输出 `✓ 进入环节: flow-design`
- 状态 phase 更新为 `flow-design`

### 2.4 next-part 跳跃（应失败）

步骤：

1. 从 `flow-design` 执行 `aide flow next-part verify "..."`

期望：

- 输出 `✗` 错误，明确指出“不可跳过环节”
- 状态不应推进（仍停留在 `flow-design`）
- 不产生新提交（若 Git 为必须）

### 2.5 back-part 回退（允许回到任意之前环节）

步骤：

1. 从 `impl` 执行 `aide flow back-part flow-design "..."`

期望：

- 输出 `⚠ 回退到环节: flow-design`
- 状态 phase 更新为 `flow-design`

### 2.6 非法 back-part（向前/原地）

步骤：

1. 从 `flow-design` 执行 `aide flow back-part impl "..."`
2. 从 `verify` 执行 `aide flow back-part verify "..."`

期望：

- 均失败并输出 `✗`
- 状态不推进

## 三、Git 集成用例

### 3.1 有变更时自动提交

步骤：

1. 修改任意可提交文件（例如 README）
2. 执行 `aide flow next-step "..."` 或 `next-part ...`

期望：

- 产生一条新 commit，commit message 符合 `aide-program/docs/formats/data.md`
- 状态历史条目中 `git_commit` 记录该 hash

### 3.2 无变更时不报错

步骤：

1. 确保工作区干净
2. 执行 `aide flow next-step "..."`

期望：

- 成功（静默）
- `git_commit` 为空

### 3.3 Git 必须但失败（应阻止状态推进）

场景示例（任选其一）：

- 在非 git 目录执行 flow 命令
- 或刻意制造 git commit 失败（例如 hook 阻止/权限问题）

期望：

- 返回失败（退出码 1）
- 状态不推进（避免“状态走了但提交没走”）

## 四、Hook 用例

### 4.1 离开 flow-design：PlantUML 校验/生成

步骤（建议最小化）：

1. 在 `docs/` 或 `discuss/` 放置一个可解析的 `.puml` 文件
2. 确保当前 phase 为 `flow-design`
3. 执行 `aide flow next-part impl "..."`

期望：

- hook 被触发
- 若配置为生成 PNG：PNG 文件被生成，并进入同一次提交（有变更时）

异常用例：

- `.puml` 语法错误：应失败并阻止跳转

### 4.2 进入 docs：提醒更新 CHANGELOG

步骤：

1. 从 `verify` 执行 `aide flow next-part docs "..."`

期望：

- 输出一次 `→` 提醒（一句话）

### 4.3 离开 docs：校验 CHANGELOG 已更新

步骤：

1. 在 docs 阶段对 `CHANGELOG.md` 做一次修改并产生至少一次提交
2. 执行 `aide flow next-part finish "..."`

期望：

- 校验通过并允许进入 finish

异常用例：

- docs 阶段未提交任何包含 `CHANGELOG.md` 的变更：按配置应失败或警告（见 `hooks.md`）
