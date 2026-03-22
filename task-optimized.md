[toc]

# 任务解析结果：Aide 体系全面重构

## 原始内容概述

用户在 task-now.md 中以口语化方式描述了对 aide 工作流体系的全面重构需求，涵盖：Commands 精简与重定义、Skills 体系重组、aide-memory 数据目录结构调整、aide 程序功能重构、新的工作流情境与阶段体系设计、Agent 角色从"亲力亲为的执行者"转变为"统筹全局的总工程师"。

## 核心意图

**将当前以"单 Agent 亲力亲为"为核心的 aide 工作流，重构为以"总工程师 Agent 统筹 + 专家子代理执行"为核心的分层协作体系**，同时重新设计数据目录结构、简化 commands、按需加载 skills、增强 aide 程序的自动化能力。

**重要说明**：这是一个全新的体系，与旧 aide 体系完全独立，无需考虑迁移兼容性。旧项目需删除旧相关内容，重新构建 memory。

---

## 结构化任务描述

### 一、数据目录结构调整

将 `.aide/` 目录迁移至项目目录下的 `aide-memory/`，新结构如下：

```
/path/to/project/
  aide-memory/
    memory/                          # 项目认知记忆
      structure/
        index.md                     # 完整目录结构索引（汇总文档）
        *.md                         # 区块内容概述文档
      concepts/
        term.md                      # 项目专用术语表
        arch.md                      # 抽象架构描述
      diagram/
        *.puml                       # 概念图解源码
        *.png                        # 编译输出的图片
      overview.md                    # 导览文档
    tasks/                           # 未归档任务目录
      task-{n}/                      # 已分配编号的任务
        information.md               # 任务描述
        design.md                    # 设计与架构方案
        todo.md                      # 待办列表（含阶段流程定义）
        flow-graphics/               # 任务相关图解
          main.puml
          *.puml
        task-summary.md              # 任务摘要
      task-now/                      # 起草中的任务（临时）
    archived-tasks/                  # 已归档任务目录
      task-{n}/
    config.toml                      # aide 程序配置（禁止 Agent 直接读取）
    config.md                        # 配置项详细文档（面向用户）
    branches.json                    # 任务分支映射数据（由 aide 程序维护）
    branches.md                      # 分支信息可读版本（由 aide 程序维护）
    templates/
      任务口述模板.md                  # 任务描述文档的初始模板
      期望激进创造大展身手的解析指导.md   # 任务解析指导文档
    aide-process-overview.md         # aide 体系总览文档
    AGENT.md                         # Agent 身份与行为准则
```

#### 关键说明

**文件位置**：
- `task-now.md`：位于项目根目录，是用户编辑任务描述的入口文件
- `task-now/`：位于 `aide-memory/tasks/` 下，是解析后的结构化任务数据目录

**访问约束**：
- `config.toml`：Agent 不可直接读取，必须通过 aide 程序获取配置信息
- `config.md`：Agent 可在任务涉及配置理解时读取
- `branches.json`：Agent 不应直接读取，应通过 aide 命令查询分支信息
- `branches.md`：Agent 不可读取（面向用户的自动生成报告）

**图解标记**：
- 不使用 `.no_graphics` 文件
- 在 `design.md` 中使用特定格式标记表示无需图解，并记录原因

---

### 二、Commands（共 5 个）

Commands 定位为**精简的入口指引**，主要用于：指出基本原则和注意事项，指导应学习哪些 skills 来达成目标。

#### 2.1 command: make-memory

- 指导 Agent 创建子代理，由子代理学习 `make-memory` skill 后为项目生成 memory 文档集

#### 2.2 command: load-memory

- 指导 Agent 学习 `load-memory` skill，根据该 skill 的方法载入项目 memory

#### 2.3 command: hi

- 若尚未理解 `aide-process-overview.md`，要求先完整读取并学习
- 指导执行 `aide hi` 子命令，说明其输出内容及如何理解
- 根据输出信息判断是否需要载入 memory（已载入则跳过）
- 结合 aide hi 输出和项目 memory，向用户提出建议的行动

#### 2.4 command: go

- 若尚未理解 `aide-process-overview.md`，要求先完整读取并学习
- 若尚未载入 memory，要求学习 `load-memory` skill 并载入
- 指导执行 `aide hi` 获取当前项目状态
- 指导执行 `aide go` 接续当前任务状态，按计划流程继续实施

#### 2.5 command: bye

- 若尚未理解 `aide-process-overview.md`，要求先完整读取并学习
- 指导执行 `aide hi` 获取当前状态
- 根据状态判断应执行的收尾操作：
  - **在常驻分支**：检查仓库状态，视情况暂存提交或直接道别
  - **在任务分支且任务已到结束阶段**：清理仓库、使用 aide 子命令结束任务、合并回常驻分支
  - **在任务分支且任务未完成**：`git add .` 暂存、编写提交消息、调用 `aide bye` 暂停任务、回到常驻分支

---

### 三、Skills

Skills 分为两类：**基础 Skills** 和 **子过程 Skills**。

#### 3.1 基础 Skills

##### skill: make-memory

由 `commands/docs.md` 迁移而来，调整为新的目录结构和配置。

核心职责：
- 严格按项目目录结构递归扫描所有文件和目录（排除 .gitignore 忽略项）
- 提取内容概述 → 保存至 `memory/structure/`
- 提取抽象概念 → 保存至 `memory/concepts/`
- 编写概念图解 → 保存至 `memory/diagram/`
- 编写导览 `overview.md`

##### skill: load-memory

由 `commands/load.md` 迁移而来，调整为新的目录结构和配置。

核心职责：按需载入 memory 文档，建立项目认知。

#### 3.2 子过程 Skills（阶段执行方法论）

每个阶段对应一个专用 skill，由总工程师 Agent 在执行到相应阶段时创建专家子代理并令其学习。

##### skill: build-task（任务构建）

核心职责：
- 指导如何构建符合 aide 体系规范的任务
- 输出 `information.md`、`design.md`、`todo.md`、`task-summary.md`
- 在 `todo.md` 中根据任务特性确定适用的阶段流程（从预设中选择并调整）
- 在 `design.md` 中标记是否需要图解，不需要时记录原因
- 引导用户对任务内容进行打磨完善

**关键变更**：
- 任务解析的具体方法和风格由用户指定的"任务解析指导文档"决定
- aide 程序在 build-task 阶段会动态输出解析指导文档的绝对路径
- build-task skill 聚焦于"如何构建规范的 task"，而非"如何解析口语化内容"

##### skill: make-graphics（图解绘制）

- 为任务编写 PlantUML 图解
- 保存至 `flow-graphics/` 目录
- 根据 `design.md` 中的标记判断是否需要绘制图解

##### skill: impl-verify（实现与验证循环）

- 按 `todo.md` 中的任务点逐一实施
- 每完成一个任务点立即进行审验
- 审验通过则标记完成，更新 `todo.md` 和 `task-summary.md`
- 审验未通过则在当前阶段内修复
- 发现需求偏差时可发起返工流程

##### skill: integration（集成测试）—— 可选阶段

- 所有任务点完成 impl-verify 循环后执行
- 整体集成测试，确保各部分协同工作
- 检查遗漏的边界情况和兼容性问题

##### skill: review（代码审查）—— 可选阶段

- 代码质量评估（可读性、可维护性、性能、安全性）
- 架构一致性检查
- 最佳实践合规检查

##### skill: docs-update（文档更新）—— 可选阶段

- 更新项目文档（README、API 文档等）
- 同步 memory 中的全局信息
- 更新 `concepts/` 和 `structure/` 中的相关内容

##### skill: confirm（用户确认）

- 向用户展示成果和变更摘要
- 收集用户反馈
- 不符合预期时触发返工决策

##### skill: finish（收尾归档）

- 使用 aide 程序完成任务归档
- 将任务目录从 `tasks/` 移至 `archived-tasks/`
- 合并任务分支回常驻分支
- 同步更新 memory 全局信息

##### skill: rework（返工）

调整现有 `rework` skill。

- 从任意阶段返回之前的阶段
- 记录返工原因和影响分析
- 确保返工后至少经过后续必要阶段才能再次进入 confirm

#### 3.3 技术参考 Skills（非阶段性）

##### skill: plantuml

保留现有 skill，供 make-graphics 等需要编写 PlantUML 的场景引用。

##### skill: aide

保留并更新，反映新的 aide 程序命令体系。

---

### 四、aide 程序功能调整

#### 4.1 核心子命令

##### `aide hi [-v]`

状态查询与展示。默认精简输出，`-v` 显示详细信息。

**在常驻分支时**：
- 显示项目目录绝对路径、当前 git 分支
- 扫描 `tasks/` 目录获取未归档任务 ID
- 从 `branches.json` 查询对应分支名
- 通过 git 读取各任务分支中的 `task-summary.md` 内容
- 显示各任务最后一次提交时间（UTC+8 完整时间 + 相对时间）
- 提示用户可使用 `aide go {n}` 进入最近活跃的任务
- 若无未归档任务，检查关键文件状态（task-now/ 是否存在、task-now.md 是否有变更等）

**在任务分支时**：
- 显示当前任务工作分支的状态信息
- 显示任务摘要
- 根据 `todo.md` 显示任务点统计（总计/已完成/未完成）
- 显示最后一次提交时间

**在其他分支时**：
- 查找常驻分支是否存在，输出相应提示

**所有路径输出使用绝对路径。**

##### `aide go [n] [-v]`

进入任务分支。

- 未传入任务编号时：
  - 仅有一个未归档任务 → 自动跳转并提示
  - 多个未归档任务 → 输出提示和帮助信息
  - `-v` 且未传入编号 → 额外执行 `aide hi -v`
- 传入有效任务编号时：
  - 仓库状态干净 → 直接切换分支
  - 仓库状态不干净 → 根据配置决定行为（默认：自动 `git add .` + commit，提交消息可配置）
  - `-v` → 切换后执行 `aide hi -v`

##### `aide bye`

清理并回到常驻分支。

- 在常驻分支：检查仓库状态，不干净则自动暂存提交（提交消息可配置）
- 在任务分支：暂存提交（如需），切换到常驻分支，输出 bye 消息
- 在其他分支：仅输出提示信息

#### 4.2 任务管理子命令

##### `aide verify`

审验 `task-now/` 目录的规范性：
- 检查 information.md、design.md、todo.md、task-summary.md 是否存在且内容符合规范
- 检查 flow-graphics/ 目录及 .puml 文件（或 .no_graphics 标记）
- 编译 plantuml 文件并检查是否通过
- 输出审验结果和反馈信息

##### `aide confirm`

敲定任务（审验通过后执行）：
- 重置 task-now.md 为模板内容
- 分配下一个任务编号（自增计数器）
- 重命名 `task-now/` → `task-{n}/`
- 根据配置的分支名前缀和格式规范确定工作分支名
- 更新 `branches.json` 和 `branches.md`
- `git add .` + 创建规范的 git 提交
- 从该提交创建任务工作分支（不切换）

##### `aide archive [n]`

归档任务：
- 将 `tasks/task-{n}/` 移至 `archived-tasks/task-{n}/`
- 更新 `branches.json` 和 `branches.md`

#### 4.3 配置项（新增/调整）

| 配置项 | 说明 | 默认值 |
|--------|------|--------|
| task_description_file | 任务描述文档路径（相对于项目根目录） | `task-now.md` |
| task_template | 任务描述文档模板路径（相对于 templates/） | `任务口述模板.md` |
| task_parse_guide | 任务解析指导文档路径（相对于 templates/） | `任务解析指导.md` |
| branch_prefix | 任务分支名前缀 | 空 |
| branch_format | 任务分支名格式（支持变量如 {n}） | `task-{n}` |
| resident_branch | 常驻分支名（不等价于主分支） | `dev` |
| auto_commit_on_switch | 切换分支时自动暂存提交 | `true` |
| auto_commit_message | 自动提交的默认消息 | `暂存：清理仓库状态以切换分支` |
| bye_commit_message | bye 操作的默认提交消息 | `暂存：清理仓库状态` |

**配置说明**：
- `resident_branch`：常驻工作分支，通常为 `dev` 或 `user-name_dev`，不建议使用主分支（master/main）
- `task_template`：模板内容参考 `/repo/agent-aide/1.md`
- `task_parse_guide`：从 `aide-plugin/skills/task-parser/SKILL.md` 去掉 yaml 头后的内容

#### 4.4 通用要求

- **所有路径输出使用绝对路径**
- 输出格式沿用现有符号规范（✓ 成功、⚠ 警告、✗ 错误、→ 进行中）
- plantuml 变更检测与自动编译集成在 hi 子命令中

---

### 五、工作流体系设计

#### 5.1 情境（Situations）

情境描述项目当前所处的大状态，非互斥，可叠加：

| 情境 | 说明 |
|------|------|
| **干净状态** | 配置有效、git 可用且干净、在常驻/任务分支、无草案残留 |
| **不干净状态** | 不符合干净标准的任何状态 |
| **草案进行中** | task-now.md 有变更 或 task-now/ 目录存在 |
| **任务实施中** | 存在未归档的任务 |

#### 5.2 阶段（Phases）

阶段属于"任务实施中"情境，具有严格的先后顺序。

**固定阶段**（每个任务必备）：
1. `build-task` — 任务构建与需求完善
2. `impl-verify` — 实施与验证（是否循环取决于任务拆分）
3. `confirm` — 用户确认
4. `finish` — 收尾归档

**可选阶段**（在 build-task 阶段根据任务特性选定，插入固定阶段之间）：
- `make-graphics` — 图解绘制（在 build-task 之后、impl-verify 之前）
- `integration` — 集成测试（在 impl-verify 之后、confirm 之前）
- `review` — 审查（代码审查/文档审校/方案评审，在 impl-verify/integration 之后、confirm 之前）
- `docs-update` — 文档更新（在 review 之后、confirm 之前）

**返工规则**：
- 从任意阶段可返回 `build-task` 发起返工
- 返工后必须至少经过后续的 `impl-verify` → `confirm` 才能再次完成
- 阶段流程写入 `todo.md` 中，由 aide 程序提取和管理

#### 5.3 场景预设（Presets）

预设为常见场景提供快速起点，在 build-task 阶段可根据具体任务动态调整。

**预设定义**：

| 预设名 | 适用场景 | 阶段流程 |
|--------|----------|----------|
| **full** | 大型软件开发、多模块系统、需要完整质量保证 | build-task → make-graphics → impl-verify:loop → integration → review → docs-update → confirm → finish |
| **standard** | 中等规模功能开发、常规软件迭代 | build-task → impl-verify:loop → review → confirm → finish |
| **lite** | 简单功能、小型修改、bug修复 | build-task → impl-verify → confirm → finish |
| **docs** | 纯文档工作、知识库管理、规范制定 | build-task → impl-verify → review → confirm → finish |
| **research** | 技术调研、方案设计、可行性分析 | build-task → make-graphics → impl-verify → docs-update → confirm → finish |

**说明**：
- `impl-verify:loop` 表示该阶段启用分部循环模式
- 预设中的阶段名称保持通用，但在不同场景下语义有所不同：
  - 软件开发：impl = 编写代码，verify = 运行测试
  - 文档工作：impl = 撰写文档，verify = 审校准确性
  - 数据分析：impl = 执行分析，verify = 验证结论

#### 5.4 Agent 角色定位

- **用户**：把握战略与方向，最终决策者
- **总工程师 Agent**：统筹流程与协作，指派和管理子代理
- **专家子代理**：执行具体阶段任务，由总工程师根据当时情境动态编写最优提示词后创建

---

### 六、AGENT.md 与 aide-process-overview.md

#### AGENT.md

- 要求先完整理解 `aide-process-overview.md`
- 定义 Agent 身份为统筹全局的总工程师
- 明确职责分工：用户决策战略、Agent 统筹流程、子代理执行具体事务

#### aide-process-overview.md

- 概述 aide 体系的所有情境和阶段
- 列出每个阶段对应的详细 skill
- 要求 Agent 在执行到相应阶段时才学习对应 skill
- 提供全局流程视图

---

## 用户确认的关键决策

基于用户在 reply.md 中的回复，以下决策已确认：

### 已确认事项

1. **task-now.md 位置**：项目根目录下的 task-now.md 是用户编辑入口，task-now/ 在 aide-memory/tasks/ 下是结构化数据目录 ✓

2. **旧功能处理**：
   - `aide decide` — 废弃 ✓
   - `aide env` — 废弃 ✓
   - `aide flow` — 保留，仅追踪阶段级别（不追踪 step） ✓

3. **常驻分支**：
   - 不等价于主分支（master/main）✓
   - 建议使用 `dev` 或 `user-name_dev` ✓
   - 通过配置指定 ✓

4. **阶段与 Skills**：
   - `confirm` 和 `finish` 需要独立 skill ✓
   - `review`（代码审查/文档审校）作为可选阶段加入 ✓

5. **模板文件**：
   - `任务口述模板.md` 参考 `/repo/agent-aide/1.md` ✓
   - `任务解析指导.md` 从 `aide-plugin/skills/task-parser/SKILL.md` 去掉 yaml 头 ✓

6. **skill 重命名**：
   - `task-parse` → `build-task` ✓
   - 解析风格由用户指定的解析指导文档决定 ✓
   - aide 程序在 build-task 阶段输出解析指导文档的绝对路径 ✓

7. **子代理运作**：
   - 子代理上下文隔离是期望的设计 ✓
   - 信息传递通过 aide-memory 中的文档实现 ✓
   - 每个子代理自行获取所需上下文和学习必要 skills ✓

8. **场景适配**：
   - 需要预定义场景预设（full/standard/lite/docs/research）✓
   - 阶段名称保持通用，语义在不同场景下适配 ✓

9. **设计优化**：
   - 保留 `branches.md`（自动生成，面向用户，无维护成本）✓
   - 使用 `design.md` 标记替代 `.no_graphics` 文件 ✓
   - 放宽 `config.md` 访问限制 ✓

10. **实施策略**：
    - 无需考虑迁移兼容性，新旧体系完全独立 ✓
    - task-now/ 冲突时用户必须先敲定或清除（暂不做智能压栈）✓
    - 可拆分为多个独立提案分阶段实施 ✓

---

## 风险评估与缓解

### 已解决的风险

1. ~~**全面重构的兼容性风险**~~：已明确无需考虑迁移，新旧体系独立 ✓
2. ~~**子代理上下文隔离**~~：已确认这是期望的设计，非风险 ✓
3. ~~**task-now 冲突**~~：已明确处理策略（用户手动敲定或清除）✓

### 需要关注的风险

1. **阶段状态管理复杂度**：动态阶段选定 + 返工机制，状态转换逻辑较复杂
   - 缓解：在 build-task 阶段明确定义阶段流程，写入 todo.md，由 aide 程序严格管理

2. **并发任务冲突**：多个未归档任务同时存在时的优先级和资源竞争
   - 缓解：通过 `aide hi` 显示最近活跃任务，`aide go` 默认进入最近活跃的任务

---

## 复杂度评估

| 维度 | 评估 | 说明 |
|------|------|------|
| 结构复杂度 | **高** | 5 个 commands、13 个 skills、aide 程序重构、目录结构全面调整 |
| 逻辑复杂度 | **高** | 情境判断、阶段流转、返工机制、分支管理、场景预设 |
| 集成复杂度 | **高** | commands ↔ skills ↔ aide 程序 ↔ git 的多层协同 |
| 风险等级 | **中** | 新旧体系独立无迁移风险，可分阶段实施 |

**Skills 统计**：
- 基础 Skills：make-memory、load-memory（2个）
- 子过程 Skills：build-task、make-graphics、impl-verify、integration、review、docs-update、confirm、finish、rework（9个）
- 技术参考 Skills：plantuml、aide（2个）
- **总计：13 个 skills**

---

## 建议的实施拆分

为降低单次变更的复杂度和风险，建议拆分为以下独立提案：

### 阶段一：基础设施（优先级：高）

**提案 1**：aide-memory 目录结构设计与初始化
- 定义完整的目录结构规范
- 实现 `aide init` 命令创建目录结构
- 编写 config.toml 配置项定义和 config.md 文档

**提案 2**：aide 程序核心子命令（hi / go / bye）
- 实现状态查询与展示逻辑
- 实现分支切换与清理逻辑
- 集成 plantuml 自动编译

### 阶段二：任务管理（优先级：高）

**提案 3**：aide 程序任务管理子命令
- 实现 `aide verify` 任务审验
- 实现 `aide confirm` 任务敲定
- 实现 `aide archive` 任务归档
- 实现 branches.json 和 branches.md 自动维护

**提案 4**：aide 程序阶段管理（flow）
- 实现阶段级别追踪
- 实现阶段流程提取和管理
- 实现场景预设定义

### 阶段三：Commands 与基础 Skills（优先级：中）

**提案 5**：Commands 重写
- make-memory、load-memory、hi、go、bye（5个）
- 编写 aide-process-overview.md 和 AGENT.md

**提案 6**：基础 Skills
- make-memory skill（迁移自 commands/docs.md）
- load-memory skill（迁移自 commands/load.md）

### 阶段四：子过程 Skills（优先级：中）

**提案 7**：核心子过程 Skills
- build-task（重命名自 task-parser）
- impl-verify
- confirm
- finish

**提案 8**：可选子过程 Skills
- make-graphics
- integration
- review
- docs-update
- rework（调整现有）

### 阶段五：技术参考 Skills（优先级：低）

**提案 9**：技术参考 Skills 更新
- plantuml skill（保留并调整）
- aide skill（更新以反映新命令体系）

---

## 附录：模板文件示例

### 任务口述模板.md

```markdown
学习并理解**任务解析指导文档**的要求后，根据**任务解析指导的要求**优化本文档，与用户进行沟通引导用户对任务进行进一步扩展和完善后，把优化后的清晰准确的任务要求保存到task-optimized.md，然后再基于task-optimized分析和构建规范的task所需文档。

非常重要！！！：你在了解了当前的项目情况对本文档内容进行理解和思考后，必须先根据**任务解析指导文档中的要求**对本文档进行分析并编写task-optimized.md！然后**经过用户检阅并答复确认后**才能开始构建task。

# 完成下列要求
```

### 任务解析指导.md

从 `aide-plugin/skills/task-parser/SKILL.md` 去掉 yaml 头部分（前3行）后的完整内容。
