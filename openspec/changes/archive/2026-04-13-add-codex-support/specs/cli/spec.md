# CLI Specification Delta

## ADDED Requirements

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
