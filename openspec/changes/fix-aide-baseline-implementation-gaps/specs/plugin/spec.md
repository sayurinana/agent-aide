## MODIFIED Requirements

### Requirement: 项目级命令与技能分发

`aide init` SHALL 将全局仓库中的 commands 与 skills 作为唯一源，分发到项目宿主目录，并确保 Claude 与 Codex 两类宿主都获得一致且已收口的产物。

#### Scenario: 单一源同时驱动 Claude 与 Codex 分发
- **WHEN** 用户执行 `aide init`
- **THEN** 系统以 `aide-plugin` 源内容同时更新 `.claude/`、`.agents/skills/` 与 Codex 目标目录
- **AND** 保证不同宿主看到的是同一套 current truth

#### Scenario: 重新同步时替换受管旧资产
- **WHEN** 系统重新同步 command 或 skill 资产
- **THEN** 受 aide 管理的旧版副本会被替换或清理
- **AND** 不保留与源仓库语义冲突的过时受管文件

#### Scenario: 自定义宿主文件不被误删
- **WHEN** 宿主目录中存在不属于 aide 受管范围的用户自定义文件
- **THEN** 系统保留这些自定义文件
- **AND** 只更新自身受管的分发产物
