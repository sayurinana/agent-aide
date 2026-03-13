# Change: 配置文件管理重构 - 分离配置与文档

## Why

当前 `aide init` 生成的 `config.toml` 采用自文档形式，包含大量注释说明（约100行，实际配置仅20行）。这导致配置文件冗长，不符合配置文件简洁性原则，且文档与配置耦合难以维护。

此外，缺少配置版本管理机制，当 aide 升级时无法自动更新用户的配置文件，也无法重置配置到默认值。

## What Changes

- 将 `config.toml` 改为简洁格式，仅保留配置项和简短行内注释
- 新增 `config.md` 作为独立的配置说明文档，详细解释每个配置项
- 新增 `aide config reset` 命令，重置配置到默认值（自动备份）
- 新增 `aide config update` 命令，处理配置版本升级（添加新配置项，注释废弃配置）
- 在配置文件中添加版本元数据 `[meta]` 节，记录 aide 版本和 schema 版本

## Impact

- **Affected specs**: `config`
- **Affected code**:
  - `src/core/config.rs` - 重构 `DEFAULT_CONFIG`，添加版本管理和迁移逻辑
  - `src/main.rs` - 添加 `Reset` 和 `Update` 子命令
  - `src/cli/config.rs` - 实现新命令处理函数
  - 新增 `config.md` 模板内容
- **Breaking changes**: 无（现有配置文件继续兼容，仅影响新生成的配置）
