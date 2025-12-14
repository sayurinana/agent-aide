# 环节钩子（Hooks）设计

## 一、目标

Hooks 用于在特定环节触发特定行为，常见用途：

- 环节离开时做强校验（阻止流程跳转）
- 自动生成/更新辅助产物（如 PlantUML PNG）
- 给出必要提醒（如进入 docs 提示更新 CHANGELOG）

## 二、触发点与顺序（必须统一）

### 2.1 触发点

Hooks 只在以下场景触发：

- `start`：视为“进入某个 phase”
- `next-part` / `back-part`：同时发生“离开旧 phase”与“进入新 phase”

`next-step/back-step/issue/error` 默认不触发 phase hooks（除非后续明确新增需求）。

### 2.2 顺序（影响提交内容）

为保证“生成物进入同一次提交”，推荐顺序：

1. **流程校验**（见 `validation.md`）
2. **离开旧 phase 的 pre-commit hooks**（可能生成/修改文件，失败则阻止跳转）
3. **Git add/commit**（见 `git.md`）
4. **写入状态文件**（包含本次 `git_commit`）
5. **进入新 phase 的 post-commit hooks**（提醒类/信息类为主）

> 若团队选择“先落盘再提交”，必须重新评估：失败边界、生成物提交时机、以及 `git_commit` 的写入方式。

## 三、Hook 接口约定（伪代码原型）

```
HookContext:
    root: Path
    phases: list[str]
    from_phase: str | None
    to_phase: str
    action: str                  # start/next-part/back-part
    summary: str

HookResult:
    ok: bool
    level: "info" | "warn" | "error"
    message: str
```

约定：

- pre-commit hook 失败（ok=false 且 level=error）必须阻止流程跳转
- warn/info 只允许在“需要用户可见”时输出，且保持简短

## 四、内置 Hooks 规格

### 4.1 离开 flow-design：PlantUML 校验与 PNG 生成（pre-commit）

**触发**：

- `next-part` 且 `from_phase == "flow-design"`

**目标**：

- 校验 PlantUML 语法
- 生成 PNG（若配置启用）

**输入约定（建议可配置）**：

| 配置项（建议） | 默认值（建议） | 说明 |
|---|---|---|
| `flow.hooks.plantuml.enabled` | true | 是否启用该 hook |
| `flow.hooks.plantuml.glob` | `docs/**/*.puml;docs/**/*.plantuml;discuss/**/*.puml;discuss/**/*.plantuml` | 待处理文件范围（分号分隔或列表） |
| `flow.hooks.plantuml.on_missing_tool` | `"warn"` | 未安装 plantuml 的处理：warn/error |
| `flow.hooks.plantuml.generate_png` | true | 是否生成 PNG |

**行为约定**：

- 若未匹配到任何 PlantUML 文件：静默通过
- 若 plantuml 工具缺失：
  - `on_missing_tool=warn`：输出一次 `⚠` 提示并通过
  - `on_missing_tool=error`：失败并阻止跳转
- 若语法校验失败：失败并阻止跳转（输出指出具体文件）

### 4.2 进入 docs：提醒更新 CHANGELOG（post-commit）

**触发**：

- `next-part` 且 `to_phase == "docs"`

**行为约定**：

- 输出一次 `→ 请更新 CHANGELOG.md`（保持一句话）
- 不影响状态推进与提交

### 4.3 离开 docs：校验 CHANGELOG 已更新（pre-commit）

**触发**：

- `next-part` 且 `from_phase == "docs"`

**校验目标**：

- 确保在 docs 阶段，至少有一次提交包含 `CHANGELOG.md` 的改动

**推荐实现判定（不引入额外状态字段）**：

1. 从状态历史中找出 `phase == "docs"` 且 `git_commit` 非空的条目集合
2. 遍历这些提交，判断是否有任意一次提交包含 `CHANGELOG.md`

**降级处理**：

- 若 Git 集成未启用/不可用：至少校验 `CHANGELOG.md` 文件存在，否则失败或警告（建议可配置）

**配置项（建议）**：

| 配置项（建议） | 默认值（建议） | 说明 |
|---|---|---|
| `flow.hooks.changelog.path` | `CHANGELOG.md` | changelog 路径 |
| `flow.hooks.changelog.required` | true | 离开 docs 时是否硬失败 |

## 五、Hook 失败的对外表现

- 校验类 hook 失败：输出 `✗` + 简短原因 + 下一步建议（例如“请先修复 PlantUML 语法”）
- 返回退出码 1
- 不推进环节，不产生提交（保持原子性）
