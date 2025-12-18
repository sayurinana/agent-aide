# CHANGELOG

本文件记录 Aide 项目对使用者可见的重要变更。

## 2025-12-19

### 新增功能

**PlantUML 渲染配置**

- 新增 `plantuml.font_name` 配置项：默认字体 "Arial"
- 新增 `plantuml.dpi` 配置项：默认 DPI 300
- 新增 `plantuml.scale` 配置项：默认缩放 0.5
- run.md 流程图示例更新：添加配置获取说明和 skinparam/scale 示例

**finish 环节自动清理任务计划文件**

- `aide flow next-part finish` 时自动清理 `.aide/task-plans/` 目录下的所有文件
- 功能由 `hooks.py` 中的 `_hook_clean_task_plans` 实现
- 从 run.md 中删除了手动清理指令

**返工流程警告增强**

- `aide flow back-part` 输出确认 key 前，新增 rework skill 学习提示
- 明确要求完成准备工作后才能确认返工

### 移除

**移除安装命令和离线安装程序**

- 删除 `/aide:install-linux` 命令（`aide-marketplace/aide-plugin/commands/install-linux.md`）
- 删除 `/aide:install-win` 命令（`aide-marketplace/aide-plugin/commands/install-win.md`）
- 删除 `aide-program/offline-installer/` 目录及其全部内容
- 清理项目文档中的相关引用

### 新增功能

**返工流程优化**

- 新增 `rework` skill：提供返工流程指南，包含返工类型判断、文档更新格式、用户提醒模板
- 新增 `aide flow back-confirm --key <key>` 命令：确认返工请求并执行
- 修改 `aide flow back-part`：执行时生成确认 key，需先完成准备工作后执行 back-confirm 确认
- 返工确认机制防止误操作，确保返工前完成文档更新
- 状态文件 `.aide/back-confirm-state.json` 用于存储待确认的返工请求

**面向用户的文档系统**

- 新增 `/aide:readme` 命令：生成面向用户的 README.md
- 新增 `/aide:user-docs` 命令：生成面向用户的文档
- 新增 `/aide:user-graph` 命令：生成面向用户的流程图
- 新增 `readme-templates` skill：提供 README 编写模板和模块

**基础配置修改**

- `gitignore_aide` 配置项默认值改为 `false`
- `aide init` 支持在任意目录执行初始化

## 2025-12-18

### 修复

**修复 end_commit 记录逻辑**
- `end_commit` 现在记录 tracker 创建的 finish 提交哈希（`[aide] finish: {summary}`）
- `finished_at` 使用 finish 提交的时间戳
- tracker.py：移除多余的 `[aide] finish: 更新状态文件` 提交，改为传递 end_commit 和 finished_at 给 branch_mgr
- branch.py：finish_branch_merge 接收 end_commit 和 finished_at 参数
- 正常 finish 流程不再创建额外的结束提交，直接使用 tracker 传入的值
- 强制清理（clean）仍会创建自己的结束提交
- flow-status.json 和 branches.json/md 的更新合并到同一个提交 `[aide] finish: 更新状态`
- 新增 `git.amend()` 方法（备用）

**修复 find_project_root 在子项目中定位错误的问题**
- 原因：从复制的测试目录运行时，因没有 `flow-status.json` 而向上查找到父项目
- 解决：添加"步骤 0"，如果当前目录已有 `.aide` 目录，直接使用不向上查找
- 新的查找策略：
  0. 当前目录有 `.aide` → 直接使用
  1. 向上查找有 `flow-status.json` 的目录（活跃任务）
  2. 向上查找有 `config.toml` 的目录

### 新增功能

**`aide flow clean` 强制清理命令**
- 用于在任务中途强制终止流程
- 如果工作区不干净，自动创建一个提交
- 执行流程与 finish 类似，但：
  - 提交消息格式为 `{start_commit[:7]}的强制清理`
  - 分支状态标记为 `force-cleaned`
- 保留任务分支供后续参考

### 修复

**修复 finish 流程中起始/结束提交哈希相同的 bug**
- 原因：切回源分支后才调用 `record_branch_finish`，导致 `rev_parse_head()` 获取的是源分支 HEAD
- 解决：在任务分支上先创建"结束提交"并记录哈希，在最终保存时直接使用保存的变量
- 新增 `record_end_commit` 方法：仅记录结束提交和时间，不更新状态
- 在 `_merge_normal` 和 `_merge_with_temp_branch` 中，直接使用保存的 `end_commit` 变量，确保不会丢失

### 变更

**finish/clean 流程重构**
- 新流程：结束提交 → 记录哈希 → 清理 → 清理提交 → 切回源分支 → squash 合并 → 更新状态(使用保存的end_commit) → 收尾提交
- `_merge_normal` 方法新增 `is_force_clean` 参数，不再调用 `update_branch_status`，直接内联更新
- `_merge_with_temp_branch` 方法也同步更新

**decisions 清理逻辑简化**
- 清理时不再备份 `decisions/*.json`，直接删除
- 移除备份目录创建逻辑

**branches.md 时间格式调整**
- 起始时间和结束时间分别单独列出
- 原格式：`- **时间**: 2025-12-18 10:30 ~ 11:45`
- 新格式：
  - `- **起始时间**: 2025-12-18 10:30`
  - `- **结束时间**: 2025-12-18 11:45`

### 修改的文件
- `aide-program/aide/core/config.py`
  - `find_project_root` 函数：添加步骤 0，当前目录有 .aide 时不向上查找
- `aide-program/aide/flow/git.py`
  - 新增 `amend` 方法：将暂存区内容追加到上一次提交
- `aide-program/aide/flow/branch.py`
  - `_cleanup_task_files` 方法：decisions 直接删除不备份
  - `record_branch_finish` 方法：保持兼容
  - 新增 `record_end_commit` 方法
  - `clean_branch_merge` 方法：脏工作区时自动创建提交
  - `_merge_normal` 方法：重构流程，结束提交后立即更新状态再清理
  - `_merge_with_temp_branch` 方法：同步重构
  - `_generate_markdown` 方法：时间格式分开显示
- `aide-program/aide/flow/tracker.py`
  - 新增 `clean` 方法
- `aide-program/aide/main.py`
  - 新增 `flow clean` 子命令
  - 新增 `handle_flow_clean` 处理函数

---

### 改进

**收尾清理逻辑增强**
- 新增删除 `pending-items.json` 文件的清理步骤
- 新增删除流程图目录（`.aide/diagrams/`）下所有文件的清理步骤
  - 支持删除 `.puml`、`.plantuml`、`.png` 文件
  - 保留目录本身

### 修改的文件
- `aide-program/aide/flow/branch.py` - `_cleanup_task_files` 方法增加两个清理步骤

### 改进

**aide 命令递归查找项目根目录**
- 新增 `find_project_root()` 函数，支持从子目录运行 aide 命令
- 类似 git 查找 `.git` 目录的逻辑：从当前目录向上递归查找
- 查找策略（两遍遍历）：
  1. 优先查找包含 `flow-status.json` 的目录（活跃任务）
  2. 其次查找包含 `config.toml` 的目录
- 解决了会话重启后工作目录变更导致任务状态"丢失"的问题

### 修改的文件
- `aide-program/aide/core/config.py` - 新增 `find_project_root()` 函数
- `aide-program/aide/main.py` - 所有 handler 函数改用 `find_project_root()` 替代 `Path.cwd()`

---

### 新增功能

**全自动任务执行命令 `/aide:auto-run`**
- 基于 `/aide:run` 的全自动化版本
- 去除所有用户交互环节：
  - 待定项自动决策（选择 recommend 或最高分选项）
  - 任务细则自动确认（无需用户确认）
  - 跳过 confirm 阶段（docs → finish）
- 新增错误自动处理机制：
  - 优先委托 `general-purpose` 子代理解决
  - 无子代理时自行解决（需先记录并更新计划）
- 新增计划外情况处理机制：
  - 必须先记录遇到的情况
  - 更新任务计划文档后才能继续
- 适用场景：批量处理、后台执行、无需人工干预的任务

**用户确认阶段 (confirm)**
- 在 docs 和 finish 之间新增 `confirm` 阶段
- 用于用户审阅和测试已完成的工作
- 支持返工机制：根据问题类型返回相应阶段
- 返工前强制更新任务文档，便于上下文恢复

**复杂任务子计划文档规范**
- 新增 `task.plans_path` 配置项（默认 `.aide/task-plans/`）
- 规范化文档结构：
  - `guide.md` - 任务计划总导览
  - `spec-01.md`, `spec-02.md`, ... - 各子计划细则
- 更新复杂任务执行模式：`task-optimize → [flow-design → impl → verify → docs → confirm] × N → finish`
- 子计划状态管理：在 guide.md 中维护状态表

### 变更

- 流程阶段更新为 7 个：task-optimize → flow-design → impl → verify → docs → confirm → finish
- run.md 中"阶段 6：收尾"更新为"阶段 7：收尾"
- 更新 aide skill 文档，添加 confirm 环节和 task-plans 目录说明

### 修改的文件
- `aide-program/aide/core/config.py` - 添加 plans_path 配置和 confirm 阶段
- `aide-marketplace/aide-plugin/commands/run.md` - 任务细则规范、子计划执行、confirm 阶段
- `aide-marketplace/aide-plugin/skills/aide/SKILL.md` - 环节列表、数据存储、流程示例
- `.aide/config.toml` - 项目配置更新

---

## 2025-12-18 (earlier)

### 变更

**aide flow finish 流程重构**
- 简化 git 历史：finish 后原分支只保留 2 个提交（起始 + 收尾）
- 收尾提交信息格式改为：`{起始哈希}的任务收尾`
- 分支记录不再保存 `end_commit`（简化数据结构）

**finish 时自动清理任务文件**
- 删除 `.aide/*.lock` 文件
- 删除任务细则文件（`task.spec` 配置路径）
- 清空任务原文件（`task.source` 配置路径，保留文件本身）
- 备份并删除 `flow-status.json` 到 `.aide/logs/{task_id}-status.json`
- 备份并删除 `decisions/*.json` 到 `.aide/logs/{task_id}-decisions/`

### 修改的文件
- `aide-program/aide/flow/branch.py` - 重构 `_merge_normal()` 方法，新增 `_cleanup_task_files()` 清理函数
- `aide-program/aide/flow/tracker.py` - 传递 `ConfigManager` 给 `BranchManager`

## 2025-12-17

### 修复

**aide flow 分支切换问题修复**
- 修复 finish 阶段分支切换时 lock 文件冲突的问题
- 修复 finish 阶段状态文件未提交导致切换失败的问题
- 在 `BranchManager` 添加 `_cleanup_lock_file` 方法
- 调整 `FlowTracker` 中分支合并的执行顺序（先提交再合并）
- **修复 `_merge_normal` 方法使用错误的 reset_soft 逻辑，改用 merge_squash**
- 在分支合并前额外提交状态文件（解决 git_commit hash 更新后未提交的问题）
- **修复 finish 后工作目录不干净的问题：在 `record_branch_finish` 后添加收尾提交**
- **修复 `git add_all` 改用 `-A` 参数，确保删除的文件也被正确暂存**

### 新增功能

**环境安装 Commands**
- 新增 `/aide:install-win` - Windows 环境安装命令
- 新增 `/aide:install-linux` - Linux 环境安装命令
- 支持两种安装模式：自动安装（报告→确认→执行）和手动指南（markdown + 脚本）
- 自动检测并安装 uv、Python（via uv）、Java JRE
- 包含 aide PATH 配置指导

**离线安装程序**
- 新增 `aide-program/offline-installer/windows/` - Windows 离线安装程序
- 新增 `aide-program/offline-installer/linux/` - Linux 离线安装程序
- 提供资源清单（resources.json）、安装脚本和使用说明
- 支持在无网络环境下安装 aide 所需的环境依赖

**aide flow 自动 Git 分支管理**
- 任务开始时自动创建 `aide/NNN` 分支（三位递增编号）
- 任务结束时自动合并并压缩提交
- 分支概况文档（JSON + MD 双格式）
- 安全合并策略（检测源分支变更，必要时创建临时分支）
- 新增 `BranchManager` 类管理分支编号和概况
- 扩展 `GitIntegration` 类支持分支操作

### 变更

**command/run 流程更新**
- 智能续接判断逻辑：检查任务细则文档，判断是否符合当前任务
- 待定项处理强制执行
- 任务细则生成后强制确认（使用 AskUserQuestion）

### 优化

**/aide:docs 命令强化完整性覆盖**
- 新增"完整覆盖"规则：根目录下每个非忽略的子目录都必须归入某个区块
- 新增强制完整性检查步骤：区块划分后必须验证所有顶层目录都已覆盖
- 新增非代码区块示例：展示如何处理 docs/、discuss/、assets/ 等目录
- 更新目录树生成规则：明确要求包含所有顶层目录（包括非代码目录）
- 更新完成检查清单：新增目录完整性最终检查项
- 修复了之前执行 /aide:docs 时遗漏 docs/、discuss/ 等非代码目录的问题

**/aide:docs 命令增强**（早期更新）
- 新增"完全深度探索"核心原则：不考虑效率，对每个文件/目录完全覆盖
- 新增完整目录树结构展示（类似 tree 命令输出）
- 支持混合模式：总导览使用简化版（前两层），区块文档使用完整 tree
- 新增空目录处理：记录并标注 `[空目录]`
- 新增被忽略文件处理：标注 `[ignored]`
- 新增二进制文件处理：根据上下文推断概括
- 新增文件类型列和统计信息
- 移除"单个区块不超过 5000 行代码"的限制

### 文档更新 (docs 阶段)

- 更新 commands/docs.md 命令定义
- 更新 aide-plugin-commands.md 区块文档
- 更新 CHANGELOG.md 记录变更详情

---

## 2025-12-16

### 新增功能

**新增 task-parser skill**
- 口语化任务内容解析器
- 支持语义解析、批判性分析、建设性建议
- 自动识别口语化特征并进行规范化转换

**流程图规范强化**
- 明确区分两种流程图类型：
  - 任务执行流程图（所有任务必需）
  - 程序逻辑流图（程序类任务必需）
- 程序逻辑流图规范：从入口函数开始，支持模块化和层次化表示

### 优化

**aide flow git 提交顺序调整**
- 状态文件先保存再执行 git 操作
- 确保 flow-status.json 的更新包含在 commit 中

### 文档

- 更新项目导览和区块文档
- 新增 task-parser skill 说明

---

## 2025-12-15 (v2.0.3)

### 文档

- 重写 README.md，提供面向用户的快速上手指南
- 原 README.md 移至 `docs/project-details.md` 作为项目详细说明

## 2025-12-15 (v2.0.0)

### 新增功能

**Commands 重组**
- 新增 `/aide:setup` - 独立的环境配置命令
- 新增 `/aide:load` - 项目文档按需载入命令
- 新增 `/aide:docs` - 项目文档创建和维护命令
- 新增 `/aide:run` - 整合任务准备和执行的核心命令（替代原 prep + exec）

**aide flow 增强**
- 新增 `aide flow status` - 查看当前任务状态
- 新增 `aide flow list` - 列出所有任务（当前 + 归档）
- 新增 `aide flow show <task_id>` - 查看指定任务的详细状态历史

**配置系统增强**
- 配置文件完全自文档化（所有配置项含详细注释说明）
- 新增 `[general]` 节：`gitignore_aide` 配置是否忽略 .aide 目录
- 新增 `[docs]` 节：项目文档路径配置
- 新增 `[flow]` 节：`diagram_path` 流程图目录配置
- 新增 `[plantuml]` 节：PlantUML jar 路径配置

**PlantUML 集成**
- 内置 plantuml.jar，支持本地流程图校验和构建
- flow-design 阶段自动校验 PlantUML 语法
- 进入 impl 阶段时自动生成 PNG 图片

**项目文档体系**
- 设计面向 LLM 的区块化文档结构
- 支持总导览 + 多子区块的文档组织
- 支持增量更新和多对话续接

**任务分析增强**
- 制定任务复杂度评估指导原则
- 支持复杂任务拆分为多个子计划
- 子计划循环执行机制

### 变更

- 原 `/aide:init` 重命名为 `/aide:setup`
- 原 `/aide:prep` 和 `/aide:exec` 合并为 `/aide:run`
- 旧命令文件保留为 `_deprecated_*.md` 供参考

## 2025-12-14

- 实现 `aide flow`（进度追踪 + Git 自动提交 + 流程校验 + Hooks）
- 补充 `aide flow` 详细设计文档与导航链路



