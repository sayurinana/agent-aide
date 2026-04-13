# cli Specification

## Purpose
TBD - created by archiving change enhance-aide-init. Update Purpose after archive.
## Requirements
### Requirement: aide init Git 仓库初始化

系统 SHALL 在 `aide init` 时自动处理 Git 仓库初始化。

初始化流程：
1. 检测当前目录是否在 Git 仓库中
2. 若不在仓库中且 Git 可用：
   - 执行 `git init`
   - 执行 `git add .`
   - 创建初始提交
3. 若 Git 不可用，输出警告并继续文件初始化

#### Scenario: 非 Git 仓库自动初始化
- **WHEN** 用户在非 Git 仓库目录执行 `aide init`
- **AND** Git 可用
- **THEN** 执行 `git init` 初始化仓库
- **AND** 执行 `git add .` 暂存所有文件
- **AND** 创建初始提交

#### Scenario: 已在 Git 仓库中
- **WHEN** 用户在 Git 仓库目录执行 `aide init`
- **THEN** 跳过 Git 初始化步骤

#### Scenario: Git 不可用
- **WHEN** 用户执行 `aide init`
- **AND** Git 未安装或不可用
- **THEN** 输出警告信息
- **AND** 继续完成文件初始化

### Requirement: aide init 常驻分支创建

系统 SHALL 在 `aide init` 时创建并切换到常驻分支。

分支处理流程：
1. 读取 `branch.resident` 配置（默认 `dev`）
2. 检测常驻分支是否已存在
3. 若不存在，创建并切换到该分支
4. 若已存在，切换到该分支

前置条件：Git 仓库已初始化且有初始提交。

#### Scenario: 常驻分支不存在
- **WHEN** 用户执行 `aide init`
- **AND** Git 仓库已初始化
- **AND** 常驻分支（如 `dev`）不存在
- **THEN** 创建常驻分支
- **AND** 切换到常驻分支

#### Scenario: 常驻分支已存在
- **WHEN** 用户执行 `aide init`
- **AND** 常驻分支已存在
- **THEN** 切换到常驻分支

#### Scenario: 当前已在常驻分支
- **WHEN** 用户执行 `aide init`
- **AND** 当前分支即为常驻分支
- **THEN** 不执行分支切换

#### Scenario: Git 不可用时跳过
- **WHEN** 用户执行 `aide init`
- **AND** Git 不可用
- **THEN** 跳过分支创建

### Requirement: aide init 任务描述文档创建

系统 SHALL 在 `aide init` 时从模板创建任务描述文档。

文档创建流程：
1. 读取 `task.description_file` 配置（默认 `task-now.md`）
2. 读取 `task.template` 配置（默认 `任务口述模板.md`）
3. 从 `aide-memory/templates/{template}` 读取模板内容
4. 若描述文件不存在，将模板内容写入描述文件
5. 若描述文件已存在，跳过（保留已有文件）

#### Scenario: 描述文件不存在
- **WHEN** 用户执行 `aide init`
- **AND** `task-now.md` 不存在
- **AND** 模板文件 `aide-memory/templates/任务口述模板.md` 存在
- **THEN** 从模板复制内容到 `task-now.md`

#### Scenario: 描述文件已存在
- **WHEN** 用户执行 `aide init`
- **AND** `task-now.md` 已存在
- **THEN** 保留已有文件，不做任何修改

#### Scenario: 模板文件不存在
- **WHEN** 用户执行 `aide init`
- **AND** 模板文件不存在
- **THEN** 跳过描述文件创建

### Requirement: aide sync 命令

系统 SHALL 提供 `aide sync` 命令，用于同步全局仓库。

命令行为：
- 克隆或更新 `~/.aide/agent-aide/` 仓库
- 使用配置中的 `plugin.repo_url` 作为仓库地址
- 成功时遵循静默成功原则，不产生输出
- 失败时输出 `✗ ` 前缀错误信息并返回非零退出码

#### Scenario: 首次同步（仓库不存在）
- **WHEN** 用户执行 `aide sync`
- **AND** `~/.aide/agent-aide/` 目录不存在
- **THEN** 克隆仓库到该目录
- **AND** 不产生成功输出

#### Scenario: 更新同步（仓库已存在）
- **WHEN** 用户执行 `aide sync`
- **AND** `~/.aide/agent-aide/` 目录已存在
- **THEN** 执行 `git pull` 更新仓库
- **AND** 不产生成功输出

#### Scenario: Git 不可用
- **WHEN** 用户执行 `aide sync`
- **AND** Git 未安装或不可用
- **THEN** 输出警告信息
- **AND** 不执行同步操作

#### Scenario: 网络失败
- **WHEN** 用户执行 `aide sync`
- **AND** 网络连接失败或仓库地址无效
- **THEN** 输出 `✗ ` 前缀错误信息
- **AND** 返回非零退出码

### Requirement: 模板同步配置

系统 SHALL 支持模板同步策略配置。

配置项：
- 键名：`template.sync_strategy`
- 默认值：`backup`
- 可选值：`backup`、`skip`、`overwrite`、`backup-and-replace`

策略行为：
| 策略 | 行为 |
|------|------|
| `backup` | 下载为 `.bak` 文件，保留原文件 |
| `skip` | 跳过已存在的文件 |
| `overwrite` | 直接覆盖已存在的文件 |
| `backup-and-replace` | 备份原文件后用新文件替换 |

#### Scenario: 配置读取默认值
- **WHEN** 配置文件未设置 `template.sync_strategy`
- **THEN** 使用默认值 `backup`

#### Scenario: 配置读取自定义值
- **WHEN** 配置文件设置 `template.sync_strategy = "overwrite"`
- **THEN** 使用策略 `overwrite`

### Requirement: aide init 模板同步

系统 SHALL 在 `aide init` 时同步模板文件。

同步行为：
- 来源：`~/.aide/agent-aide/templates/`
- 目标：项目 `aide-memory/templates/`
- 根据 `template.sync_strategy` 配置处理已存在文件
- 全局仓库不存在时跳过模板同步并输出警告

#### Scenario: 模板同步成功（文件不存在）
- **WHEN** 用户执行 `aide init`
- **AND** 项目 `aide-memory/templates/` 目录不存在或为空
- **AND** 全局仓库存在且包含 templates 目录
- **THEN** 复制所有模板文件到项目目录

#### Scenario: 模板同步策略 backup
- **WHEN** 用户执行 `aide init`
- **AND** 模板文件已存在
- **AND** 配置 `template.sync_strategy = "backup"`
- **THEN** 下载新模板为 `.bak` 文件
- **AND** 保留原文件不变

#### Scenario: 模板同步策略 skip
- **WHEN** 用户执行 `aide init`
- **AND** 模板文件已存在
- **AND** 配置 `template.sync_strategy = "skip"`
- **THEN** 跳过已存在的文件
- **AND** 仅复制不存在的新文件

#### Scenario: 模板同步策略 overwrite
- **WHEN** 用户执行 `aide init`
- **AND** 模板文件已存在
- **AND** 配置 `template.sync_strategy = "overwrite"`
- **THEN** 直接覆盖已存在的文件

#### Scenario: 模板同步策略 backup-and-replace
- **WHEN** 用户执行 `aide init`
- **AND** 模板文件已存在
- **AND** 配置 `template.sync_strategy = "backup-and-replace"`
- **THEN** 将原文件备份为 `.bak` 文件
- **AND** 用新模板文件替换原文件

#### Scenario: 全局仓库不存在
- **WHEN** 用户执行 `aide init`
- **AND** `~/.aide/agent-aide/` 目录不存在
- **THEN** 输出警告信息
- **AND** 跳过模板同步

### Requirement: aide init 核心文件同步

系统 SHALL 在 `aide init` 时从全局仓库同步核心文件到项目目录。

同步文件：
- `AGENT.md`：Agent 角色说明文档
- `aide-process-overview.md`：Aide 工作流程总览

同步行为：
- 来源：`~/.aide/agent-aide/aide-memory/`
- 目标：项目 `aide-memory/`
- 文件不存在时才复制（不覆盖已有文件）
- 全局仓库不存在时使用内置默认内容

#### Scenario: 核心文件同步（全局仓库存在）
- **WHEN** 用户执行 `aide init`
- **AND** 项目 `aide-memory/AGENT.md` 不存在
- **AND** 全局仓库 `~/.aide/agent-aide/aide-memory/AGENT.md` 存在
- **THEN** 复制该文件到项目目录

#### Scenario: 核心文件同步（全局仓库不存在）
- **WHEN** 用户执行 `aide init`
- **AND** 全局仓库不存在
- **THEN** 使用内置默认内容创建文件

#### Scenario: 核心文件已存在
- **WHEN** 用户执行 `aide init`
- **AND** 项目 `aide-memory/AGENT.md` 已存在
- **THEN** 保留现有文件，不覆盖

### Requirement: 默认仓库地址更新

系统 SHALL 使用 HTTPS 协议作为默认仓库地址。

默认值变更：
- 原地址：`git@github.com:sayurinana/agent-aide.git`（SSH）
- 新地址：`https://github.com/sayurinana/agent-aide.git`（HTTPS）

#### Scenario: 新安装使用 HTTPS 地址
- **WHEN** 用户首次执行 `aide init --global`
- **AND** 配置文件不存在
- **THEN** 生成的配置使用 HTTPS 仓库地址

#### Scenario: 已有配置保持不变
- **WHEN** 用户已有配置文件
- **AND** 配置中 `plugin.repo_url` 已设置
- **THEN** 保持用户原有配置不变

### Requirement: aide init Codex 插件同步

系统 SHALL 在 `aide init` 时同步插件到 Codex 目录结构。

同步行为：
- 来源：`~/.aide/agent-aide/aide-plugin/`
- 目标：
  - Commands：`~/.codex/prompts/`（全局目录）
  - Skills：`.agents/skills/`（项目目录）
- 默认启用，无需额外配置
- 如果目标目录不存在，自动创建
- 如果全局插件仓库不存在，输出警告并跳过
- 同步失败不影响整体初始化流程

#### Scenario: Codex 插件同步成功（项目初始化）

- **WHEN** 用户执行 `aide init`
- **AND** 全局插件仓库 `~/.aide/agent-aide/aide-plugin/` 存在
- **THEN** 复制 commands 到 `~/.codex/prompts/`
- **AND** 复制 skills 到 `.agents/skills/`
- **AND** 输出成功提示信息

#### Scenario: Codex 插件同步成功（全局初始化）

- **WHEN** 用户执行 `aide init --global`
- **AND** 全局插件仓库 `~/.aide/agent-aide/aide-plugin/` 存在
- **THEN** 复制 commands 到 `~/.codex/prompts/`
- **AND** 输出成功提示信息

#### Scenario: 全局插件仓库不存在

- **WHEN** 用户执行 `aide init`
- **AND** `~/.aide/agent-aide/aide-plugin/` 目录不存在
- **THEN** 输出警告信息
- **AND** 跳过 Codex 插件同步
- **AND** 继续完成其他初始化步骤

#### Scenario: 目标目录创建失败

- **WHEN** 用户执行 `aide init`
- **AND** 无法创建 `~/.codex/prompts/` 或 `.agents/skills/` 目录
- **THEN** 输出警告信息
- **AND** 跳过该目录的同步
- **AND** 继续完成其他初始化步骤

#### Scenario: 文件复制失败

- **WHEN** 用户执行 `aide init`
- **AND** 文件复制过程中发生错误
- **THEN** 输出警告信息
- **AND** 继续完成其他初始化步骤

#### Scenario: 目标目录已存在文件

- **WHEN** 用户执行 `aide init`
- **AND** `~/.codex/prompts/` 或 `.agents/skills/` 目录已存在文件
- **THEN** 删除已有目录并重新复制
- **AND** 确保同步后的文件与源文件一致

