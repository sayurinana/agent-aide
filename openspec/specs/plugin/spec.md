# plugin Specification

## Purpose
定义 agent-aide 插件仓库的同步、项目级命令与技能分发，以及 Claude/Codex 双宿主的插件资产落盘边界。

## Requirements

### Requirement: 全局插件仓库同步

`aide` SHALL 在全局初始化或显式同步时维护 agent-aide 插件仓库，使其成为项目级 command 与 skill 分发的统一来源。

#### Scenario: 全局仓库首次同步
- **WHEN** 用户执行全局初始化或显式同步
- **AND** 全局插件仓库不存在
- **THEN** 系统克隆默认或配置指定的仓库地址
- **AND** 使后续项目级分发可用

#### Scenario: 全局仓库更新同步
- **WHEN** 用户执行全局初始化或显式同步
- **AND** 全局插件仓库已存在
- **THEN** 系统更新该仓库
- **AND** 不破坏已有可用分发能力

#### Scenario: Git 不可用时跳过同步
- **WHEN** 用户执行全局初始化或显式同步
- **AND** Git 未安装或不可用
- **THEN** 系统输出可理解的警告
- **AND** 继续执行不依赖仓库同步的其他初始化步骤

### Requirement: 项目级命令与技能分发

`aide init` SHALL 将全局仓库中的 commands 与 skills 分发到项目宿主目录，并确保 Claude 与 Codex 两类宿主都获得可用产物。

#### Scenario: Claude 宿主分发
- **WHEN** 用户执行 `aide init`
- **THEN** 系统将命令与技能同步到项目 `.claude/` 目录
- **AND** 保证项目内命令入口与技能目录可直接被 Claude Code 使用

#### Scenario: Codex 宿主分发
- **WHEN** 用户执行 `aide init`
- **THEN** 系统将命令与技能同步到 Codex 需要的目标位置
- **AND** 保证 Codex 宿主可获得与 Claude 侧一致的 workflow 资产

### Requirement: 宿主分发边界

插件分发 capability SHALL 只负责资产同步与宿主落盘，不负责 command 与 skill 的业务语义定义。

#### Scenario: 分发与业务语义解耦
- **WHEN** 系统同步 command 或 skill 文件
- **THEN** `plugin` capability 仅约束来源、目标、覆盖策略与失败处理
- **AND** 具体 command/skill 的行为语义由其他 capability 规范

### Requirement: 插件同步失败与覆盖策略

插件同步 SHALL 明确失败处理和覆盖策略，使初始化流程在不同宿主环境下具备稳定可预期的行为。

#### Scenario: 同步失败不阻断主流程
- **WHEN** 插件同步过程出现网络、权限或目标目录错误
- **THEN** 系统输出可理解的错误或警告
- **AND** 在允许的情况下继续完成其他初始化步骤

#### Scenario: 目标已存在文件时执行既定策略
- **WHEN** 目标目录中已存在旧版 commands 或 skills
- **THEN** 系统按规定的覆盖或替换策略处理这些文件
- **AND** 最终宿主目录中的资产与当前全局仓库保持一致

### Requirement: 插件配置项一致性

系统 SHALL 使用一致的插件配置项控制仓库地址与同步策略，并在文档中明确这些配置如何影响实际分发行为。

#### Scenario: 默认配置使用 HTTPS 仓库地址
- **WHEN** 用户首次生成默认插件配置
- **THEN** 默认仓库地址使用 HTTPS 协议
- **AND** 不再以 SSH 地址作为 current truth 默认值

#### Scenario: 使用自定义插件仓库配置
- **WHEN** 用户在配置中指定自定义插件仓库地址
- **THEN** 后续同步使用该地址
- **AND** 项目级分发仍遵循相同的宿主落盘规则

#### Scenario: 禁用自动同步
- **WHEN** 配置中关闭自动插件同步
- **THEN** 系统跳过初始化时的自动分发
- **AND** 不影响用户后续显式触发同步
