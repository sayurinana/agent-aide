# plugin Specification Delta

## ADDED Requirements

### Requirement: 插件仓库克隆

`aide init --global` SHALL 在初始化全局配置时，检测并克隆 agent-aide 仓库：

1. 检测系统是否安装 Git 命令行工具
   - 若未安装：输出警告 `⚠ Git 未安装，跳过插件仓库同步`，继续执行后续步骤
   - 若已安装：继续下一步

2. 读取全局配置中的 `plugin.repo_url` 值
   - 若配置项不存在：使用硬编码默认值 `git@github.com:sayurinana/agent-aide.git`
   - 若配置项存在：使用配置值

3. 检查 `$HOME/.aide/agent-aide/` 目录是否存在
   - 若不存在：执行 `git clone <repo_url> "$HOME/.aide/agent-aide"`
   - 若已存在且为 git 仓库：执行 `git -C "$HOME/.aide/agent-aide" pull`
   - 若已存在但非 git 仓库：输出警告，跳过同步

4. 克隆或更新成功后，输出 `✓ 插件仓库已同步到 ~/.aide/agent-aide/`

#### Scenario: 全局初始化时首次克隆插件仓库

- **WHEN** 运行 `aide init --global`
- **AND** Git 已安装
- **AND** `$HOME/.aide/agent-aide/` 不存在
- **THEN** 执行 `git clone <repo_url> "$HOME/.aide/agent-aide"`
- **AND** 输出 `✓ 插件仓库已同步到 ~/.aide/agent-aide/`

#### Scenario: 全局初始化时更新已有仓库

- **WHEN** 运行 `aide init --global`
- **AND** Git 已安装
- **AND** `$HOME/.aide/agent-aide/` 已存在且为 git 仓库
- **THEN** 执行 `git -C "$HOME/.aide/agent-aide" pull`
- **AND** 输出 `✓ 插件仓库已同步到 ~/.aide/agent-aide/`

#### Scenario: 全局初始化时 Git 未安装

- **WHEN** 运行 `aide init --global`
- **AND** Git 未安装
- **THEN** 输出 `⚠ Git 未安装，跳过插件仓库同步`
- **AND** 继续执行后续初始化步骤

#### Scenario: 全局初始化时使用自定义仓库地址

- **WHEN** 运行 `aide init --global`
- **AND** 全局配置中 `plugin.repo_url = "git@github.com:user/custom-aide.git"`
- **THEN** 使用配置值 `git@github.com:user/custom-aide.git` 作为克隆地址

#### Scenario: 克隆失败时输出错误

- **WHEN** 运行 `aide init --global`
- **AND** `git clone` 命令失败（网络错误、权限问题等）
- **THEN** 输出 `✗ 插件仓库同步失败: <错误信息>`
- **AND** 继续执行后续初始化步骤

### Requirement: 项目插件同步

`aide init` SHALL 在初始化项目时，将 commands 和 skills 从全局仓库同步到项目目录：

1. 检查 `$HOME/.aide/agent-aide/aide-plugin/` 目录是否存在
   - 若不存在：输出警告 `⚠ 全局插件仓库不存在，跳过插件同步。请先执行 aide init --global`
   - 若存在：继续下一步

2. 创建项目 `.claude/` 目录（若不存在）

3. 复制 `$HOME/.aide/agent-aide/aide-plugin/commands/` 到项目 `.claude/commands/`
   - 若目标已存在：覆盖现有文件

4. 复制 `$HOME/.aide/agent-aide/aide-plugin/skills/` 到项目 `.claude/skills/`
   - 若目标已存在：覆盖现有文件

5. 输出 `✓ 已同步 commands 和 skills 到 .claude/`

#### Scenario: 项目初始化时同步插件

- **WHEN** 运行 `aide init`
- **AND** `$HOME/.aide/agent-aide/aide-plugin/` 存在
- **THEN** 创建项目 `.claude/` 目录
- **AND** 复制 `commands/` 到 `.claude/commands/`
- **AND** 复制 `skills/` 到 `.claude/skills/`
- **AND** 输出 `✓ 已同步 commands 和 skills 到 .claude/`

#### Scenario: 项目初始化时全局仓库不存在

- **WHEN** 运行 `aide init`
- **AND** `$HOME/.aide/agent-aide/` 不存在
- **THEN** 输出 `⚠ 全局插件仓库不存在，跳过插件同步。请先执行 aide init --global`
- **AND** 继续执行其他初始化步骤

#### Scenario: 项目初始化时覆盖已有文件

- **WHEN** 运行 `aide init`
- **AND** 项目 `.claude/commands/` 已存在
- **AND** `$HOME/.aide/agent-aide/aide-plugin/` 存在
- **THEN** 覆盖 `.claude/commands/` 中的文件
- **AND** 输出 `✓ 已同步 commands 和 skills 到 .claude/`

### Requirement: 插件配置项

配置文件 SHALL 包含 `[plugin]` 配置段，支持以下配置项：

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `repo_url` | String | `git@github.com:sayurinana/agent-aide.git` | agent-aide 仓库 Git 地址 |
| `sync_on_init` | Boolean | `true` | 项目初始化时是否同步插件 |

配置说明文档 `config.md` SHALL 包含 `[plugin]` 段的详细说明。

#### Scenario: 默认配置包含 plugin 段

- **WHEN** 运行 `aide init` 首次初始化
- **THEN** 生成的 `config.toml` 包含 `[plugin]` 节
- **AND** `plugin.repo_url = "git@github.com:sayurinana/agent-aide.git"`
- **AND** `plugin.sync_on_init = true`

#### Scenario: 配置文档包含 plugin 说明

- **WHEN** 运行 `aide init` 首次初始化
- **THEN** 生成的 `config.md` 包含 `[plugin]` 节的详细说明
- **AND** 说明包含 `repo_url` 的用途和修改方法

#### Scenario: 禁用自动同步

- **WHEN** 配置中 `plugin.sync_on_init = false`
- **AND** 运行 `aide init`
- **THEN** 跳过插件同步步骤
- **AND** 不输出插件同步相关提示

## MODIFIED Requirements

### Requirement: 初始化命令（修改自 config/spec.md）

`aide init --global` SHALL 仅在用户主目录下操作：

1. 检查 `$HOME/.aide/config.toml` 是否存在
   - 若不存在：创建 `$HOME/.aide/` 目录及子目录，写入默认配置和配置说明文档
   - 若已存在：输出 `→ 全局配置已存在：$HOME/.aide/config.toml`，不做任何修改
2. 不修改当前工作目录下的任何文件
3. 不在当前工作目录创建 `.aide/` 目录
4. **检测 Git 可用性并克隆/更新 agent-aide 仓库**（新增）
5. 检测 PlantUML 可执行程序是否可用：
   - 若可用：输出 PlantUML 版本信息
   - 若不可用：提示用户 `PlantUML 未安装，是否现在自动下载并安装？[Y/n]`

#### Scenario: 全局初始化完整流程（修改）

- **WHEN** 运行 `aide init --global`
- **AND** 全局配置不存在
- **AND** Git 已安装
- **THEN** 创建 `$HOME/.aide/` 目录及子目录
- **AND** 写入默认配置和配置说明文档
- **AND** 克隆 agent-aide 仓库到 `$HOME/.aide/agent-aide/`
- **AND** 检测 PlantUML 可用性并提示安装
- **AND** 输出 `✓ 全局配置初始化完成`