# Change: 添加全局配置支持

## Why

目前 aide 的配置文件仅存在于项目目录的 `.aide/config.toml` 中，每次在新项目中执行 `aide init` 都会生成硬编码的默认配置。用户无法自定义全局默认值（如 `plantuml.jar_path`、`decide.port` 等），导致每个新项目都需要重复修改相同的配置项。

通过引入全局配置（`~/.aide/config.toml`），用户可以一次性设置自己偏好的默认值，所有新项目的初始化都将基于全局配置进行复制，减少重复配置工作。

## What Changes

- `aide init`：新增全局配置初始化步骤——先确保 `~/.aide/config.toml` 存在，再将其复制到项目目录
- `aide init --global`：新增 `--global` 标志，仅创建/管理全局配置，不影响当前工作目录
- `aide config get/set --global`：支持直接读取/修改全局配置
- `aide config reset --global`：支持重置全局配置（备份到 `~/.aide/backups/`）
- `aide config update --global`：支持升级全局配置的 schema 版本
- `ConfigManager`：新增 `new_global()` 构造函数，以 `~/.aide/` 为根目录复用现有方法
- 新增 `global_aide_dir()` 辅助函数，获取全局配置目录路径

## Impact

- Affected specs: `config`、`cli`
- Affected code: `src/main.rs`、`src/cli/init.rs`、`src/cli/config.rs`、`src/core/config.rs`
- 向后兼容：现有项目配置行为不变，项目已有 `.aide/config.toml` 时 `aide init` 不覆盖
