---
name: aide
description: Aide 工作流工具集。提供环境管理、进度追踪、待定项确认等功能。当需要执行 aide 命令、管理任务进度、处理待定项确认时使用。
---

# Aide 工具使用指南

Aide 是一套命令行工具，用于支持 Aide 工作流体系。所有 aide 命令的输出遵循精简原则。

## 输出格式

| 前缀 | 含义 |
|------|------|
| `✓` | 成功 |
| `⚠` | 警告（可继续） |
| `✗` | 错误（需处理） |
| `→` | 进行中/信息 |

**静默原则**：无输出 = 正常完成

---

## 核心子命令

### aide hi [-v]

状态查询与展示。默认精简输出，`-v` 显示详细信息。

**在常驻分支时**：
- 显示项目目录绝对路径、当前 git 分支
- 扫描 `tasks/` 目录获取未归档任务 ID
- 从 `branches.json` 查询对应分支名
- 通过 git 读取各任务分支中的 `task-summary.md` 内容
- 显示各任务最后一次提交时间（UTC+8 完整时间 + 相对时间）
- 提示用户可使用 `aide go {n}` 进入最近活跃的任务
- 若无未归档任务，检查关键文件状态

**在任务分支时**：
- 显示当前任务工作分支的状态信息
- 显示任务摘要
- 根据 `todo.md` 显示任务点统计（总计/已完成/未完成）
- 显示最后一次提交时间

**在其他分支时**：
- 查找常驻分支是否存在，输出相应提示

**所有路径输出使用绝对路径。**

**输出示例**：

```
# 常驻分支，有未归档任务
→ 项目目录: /path/to/project
→ 当前分支: dev
→ 未归档任务:
  *[1] task-1 (impl-verify) 最后提交: 2025-03-23 18:30:00 (2小时前)
   [2] task-2 (confirm) 最后提交: 2025-03-22 10:00:00 (1天前)
→ 提示: 使用 aide go 1 进入最近活跃任务

# 任务分支
→ 任务分支: task-1
→ 任务摘要: 实现用户认证模块
→ 任务点: 5/8 完成
→ 最后提交: 2025-03-23 18:30:00
```

### aide go [n] [-v]

进入任务分支。

- 未传入任务编号时：
  - 仅有一个未归档任务 → 自动跳转并提示
  - 多个未归档任务 → 输出提示和帮助信息
  - `-v` 且未传入编号 → 额外执行 `aide hi -v`
- 传入有效任务编号时：
  - 仓库状态干净 → 直接切换分支
  - 仓库状态不干净 → 根据配置决定行为（默认：自动 `git add .` + commit）
  - `-v` → 切换后执行 `aide hi -v`

**示例**：

```bash
# 自动进入唯一任务
aide go
# 输出: ✓ 已切换到分支: task-1

# 指定任务编号
aide go 1
# 输出: ✓ 已切换到分支: task-1

# 详细模式
aide go 1 -v
# 输出: ✓ 已切换到分支: task-1
#       (然后执行 aide hi -v 的输出)
```

### aide bye

清理并回到常驻分支。

- 在常驻分支：检查仓库状态，不干净则自动暂存提交
- 在任务分支：暂存提交（如需），切换到常驻分支，输出 bye 消息
- 在其他分支：仅输出提示信息

**示例**：

```bash
aide bye
# 输出: ✓ 已暂存并提交
#       ✓ 已切换到常驻分支: dev
#       → Bye! 下次继续请使用 aide go
```

---

## 任务管理子命令

### aide verify

审验 `task-now/` 目录的规范性：
- 检查 information.md、design.md、todo.md、task-summary.md 是否存在且内容符合规范
- 检查 flow-graphics/ 目录及 .puml 文件（或 design.md 中无图解标记）
- 编译 plantuml 文件并检查是否通过
- 输出审验结果和反馈信息

**示例**：

```bash
aide verify
# 输出:
# ✓ information.md 存在且格式正确
# ✓ design.md 存在且格式正确
# ✓ todo.md 存在且格式正确
# ✓ task-summary.md 存在且格式正确
# ✓ flow-graphics/main.puml 编译通过
# ✓ 审验通过，可执行 aide confirm 敲定任务
```

### aide confirm

敲定任务（审验通过后执行）：
- 重置 task-now.md 为模板内容
- 分配下一个任务编号（自增计数器）
- 重命名 `task-now/` → `task-{n}/`
- 根据配置的分支名前缀和格式规范确定工作分支名
- 更新 `branches.json` 和 `branches.md`
- `git add .` + 创建规范的 git 提交
- 从该提交创建任务工作分支（不切换）

**示例**：

```bash
aide confirm
# 输出:
# ✓ 已重置 task-now.md
# ✓ 已分配任务编号: 3
# ✓ 已重命名目录: task-now/ → task-3/
# ✓ 已更新分支映射
# ✓ 已创建提交: [aide] confirm: 任务敲定
# ✓ 已创建工作分支: task-3
# → 提示: 使用 aide go 3 进入任务分支
```

### aide archive [n]

归档任务：
- 将 `tasks/task-{n}/` 移至 `archived-tasks/task-{n}/`
- 更新 `branches.json` 和 `branches.md`

**示例**：

```bash
aide archive 3
# 输出:
# ✓ 已归档任务: task-3
# ✓ 已更新分支映射
# → 提示: 任务分支 task-3 可手动删除
```

---

## 阶段管理

### aide flow

阶段级别追踪（不追踪 step）。

```bash
# 查看当前阶段状态
aide flow status

# 进入下一阶段
aide flow next <阶段名> "<摘要>"

# 返回之前阶段（返工）
aide flow back <阶段名> "<原因>"
```

**阶段名称**：

| 阶段 | 类型 | 说明 |
|------|------|------|
| build-task | 固定 | 任务构建与需求完善 |
| make-graphics | 可选 | 图解绘制 |
| impl-verify | 固定 | 实施与验证循环 |
| integration | 可选 | 集成测试 |
| review | 可选 | 代码审查/文档审校 |
| docs-update | 可选 | 文档更新 |
| confirm | 固定 | 用户确认 |
| finish | 固定 | 收尾归档 |

**输出示例**：

```bash
aide flow status
# 输出:
# → 任务: task-1
# → 当前阶段: impl-verify
# → 任务点: 5/8 完成
# → 最后更新: 2025-03-23 18:30:00

aide flow next review "impl-verify 完成，进入审查"
# 输出:
# ✓ 已进入阶段: review
# ✓ 已创建提交: [aide] flow: impl-verify → review
```

---

## 配置项

配置文件位于 `aide-memory/config.toml`。

| 配置项 | 说明 | 默认值 |
|--------|------|--------|
| `task_description_file` | 任务描述文档路径（相对于项目根目录） | `task-now.md` |
| `task_template` | 任务描述文档模板路径（相对于 templates/） | `任务口述模板.md` |
| `task_parse_guide` | 任务解析指导文档路径（相对于 templates/） | `任务解析指导.md` |
| `branch_prefix` | 任务分支名前缀 | 空 |
| `branch_format` | 任务分支名格式（支持变量如 {n}） | `task-{n}` |
| `resident_branch` | 常驻分支名（不等价于主分支） | `dev` |
| `auto_commit_on_switch` | 切换分支时自动暂存提交 | `true` |
| `auto_commit_message` | 自动提交的默认消息 | `暂存：清理仓库状态以切换分支` |
| `bye_commit_message` | bye 操作的默认提交消息 | `暂存：清理仓库状态` |

**配置说明**：
- `resident_branch`：常驻工作分支，通常为 `dev` 或 `user-name_dev`，不建议使用主分支（master/main）

---

## 数据存储

所有 aide 数据文件存放在项目目录的 `aide-memory/` 下：

```
aide-memory/
├── memory/                  # 项目认知记忆
│   ├── structure/           # 文件结构概述
│   ├── concepts/            # 概念和术语
│   ├── diagram/             # 概念图解
│   └── overview.md          # 导览文档
├── tasks/                   # 未归档任务目录
│   ├── task-now/            # 起草中的任务
│   └── task-{n}/            # 已分配编号的任务
├── archived-tasks/          # 已归档任务目录
│   └── task-{n}/
├── config.toml              # aide 程序配置
├── config.md                # 配置项详细文档
├── branches.json            # 任务分支映射数据
├── branches.md              # 分支信息可读版本
├── templates/               # 模板文件
└── AGENT.md                 # Agent 身份与行为准则
```

---

## 常见用法示例

### 查看状态

```bash
aide hi
aide hi -v  # 详细模式
```

### 开始新任务

```bash
# 1. 编辑 task-now.md 描述任务
# 2. 执行 aide verify 审验
# 3. 执行 aide confirm 敲定
# 4. 执行 aide go 进入任务分支
```

### 进入任务

```bash
aide go        # 自动进入唯一任务
aide go 1      # 进入指定任务
aide go 1 -v   # 进入并显示详细状态
```

### 管理阶段

```bash
aide flow status              # 查看当前阶段
aide flow next impl-verify "开始实施"  # 进入下一阶段
aide flow back build-task "需求偏差"   # 返工
```

### 结束任务

```bash
aide bye       # 暂存并回到常驻分支
aide archive 1 # 归档已完成任务
```

---

## 与 Commands 的配合

| Command | 调用的 aide 命令 |
|---------|------------------|
| `/hi` | `aide hi` |
| `/go` | `aide hi`, `aide go` |
| `/bye` | `aide hi`, `aide bye` |
| build-task skill | `aide verify`, `aide confirm` |
| finish skill | `aide archive` |

---

## 注意事项

1. **所有路径输出使用绝对路径**
2. 输出格式沿用符号规范（✓ 成功、⚠ 警告、✗ 错误、→ 进行中）
3. plantuml 变更检测与自动编译集成在 hi 子命令中
4. 配置文件 Agent 不可直接读取 config.toml，可通过 config.md 了解配置项