## ADDED Requirements

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