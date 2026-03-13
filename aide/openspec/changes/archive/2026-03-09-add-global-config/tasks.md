## 1. 核心基础设施

- [x] 1.1 在 `src/core/config.rs` 中添加 `global_aide_dir()` 函数，返回 `$HOME/.aide` 路径
- [x] 1.2 在 `ConfigManager` 中添加 `new_global()` 构造函数，以 `$HOME/.aide` 的父目录（即 `$HOME`）为根目录创建实例
- [x] 1.3 为 `global_aide_dir()` 和 `new_global()` 编写单元测试

## 2. CLI 参数扩展

- [x] 2.1 在 `src/main.rs` 中为 `Init` 命令添加 `--global` 参数
- [x] 2.2 在 `src/main.rs` 中为 `ConfigCommands::Get`、`Set`、`Reset`、`Update` 添加 `--global` 参数
- [x] 2.3 更新 `main()` 中的 match 分支，将 `global` 参数传递到各处理函数

## 3. init 命令实现

- [x] 3.1 重构 `handle_init(global: bool)` 函数签名
- [x] 3.2 实现 `--global` 分支：仅创建 `~/.aide/config.toml`，已存在时提示
- [x] 3.3 实现默认分支（无 `--global`）：先确保全局配置存在 → 复制到项目 → schema 版本低时提示
- [x] 3.4 为 init 命令编写单元测试（通过集成测试覆盖）

## 4. config 子命令实现

- [x] 4.1 重构 `handle_config_get(key, global)` 和 `handle_config_set(key, value, global)`，`--global` 时使用 `ConfigManager::new_global()`
- [x] 4.2 重构 `handle_config_reset(force, global)`，`--global` 时操作 `~/.aide/config.toml`，备份到 `~/.aide/backups/`
- [x] 4.3 重构 `handle_config_update(global)`，`--global` 时升级 `~/.aide/config.toml` 的 schema 版本
- [x] 4.4 全局配置不存在时各子命令输出错误提示
- [x] 4.5 为各子命令的 `--global` 路径编写单元测试（通过集成测试和 `get_config_manager`/`ensure_global_config_exists` 辅助函数逻辑覆盖）

## 5. 验证

- [x] 5.1 运行 `cargo test` 确保所有测试通过（106 单元 + 17 集成，全部通过）
- [x] 5.2 运行 `cargo build` 确保编译无错误和无新增警告
- [x] 5.3 手动验证：`aide init --global` → `aide init` → `aide config get --global meta.schema_version` → `aide config set --global decide.port 4000` → `aide config reset --global --force` → `aide config update --global`
