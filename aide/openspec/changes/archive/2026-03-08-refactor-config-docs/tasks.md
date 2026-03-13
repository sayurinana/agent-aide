# Implementation Tasks

## 1. 配置模板重构
- [x] 1.1 重构 `src/core/config.rs` 中的 `DEFAULT_CONFIG`，移除详细注释，保留简洁配置
- [x] 1.2 添加 `[meta]` 节，包含 `aide_version` 和 `schema_version` 字段
- [x] 1.3 创建 `config.md` 模板内容，详细说明所有配置项

## 2. 配置文档生成
- [x] 2.1 在 `ConfigManager` 中添加 `generate_config_md()` 方法
- [x] 2.2 修改 `ensure_config()` 在生成 `config.toml` 时同时生成 `config.md`
- [x] 2.3 确保 `config.md` 包含所有配置节的详细说明和示例

## 3. aide config reset 命令
- [x] 3.1 在 `src/main.rs` 的 `ConfigCommands` 枚举中添加 `Reset` 变体
- [x] 3.2 在 `src/cli/config.rs` 中实现 `handle_config_reset()` 函数
- [x] 3.3 实现备份逻辑：将现有配置复制到 `.aide/backups/config.toml.{timestamp}`
- [x] 3.4 添加确认提示，避免误操作
- [x] 3.5 重置后重新生成 `config.md`

## 4. aide config update 命令
- [x] 4.1 在 `ConfigCommands` 枚举中添加 `Update` 变体
- [x] 4.2 在 `src/cli/config.rs` 中实现 `handle_config_update()` 函数
- [x] 4.3 实现版本检测逻辑：比较配置中的版本与当前 aide 版本
- [x] 4.4 实现配置迁移框架：添加新配置项，保留用户自定义值
- [x] 4.5 实现废弃配置处理：注释掉废弃项并添加说明
- [x] 4.6 更新后重新生成 `config.md`

## 5. 版本管理基础设施
- [x] 5.1 定义配置 schema 版本常量（从 1 开始）
- [x] 5.2 实现版本比较逻辑
- [x] 5.3 创建配置迁移注册机制（为未来扩展）

## 6. 测试
- [x] 6.1 添加 `DEFAULT_CONFIG` 格式测试（验证简洁性）
- [x] 6.2 添加 `config reset` 集成测试
- [x] 6.3 添加 `config update` 版本检测测试
- [x] 6.4 添加配置备份功能测试
- [x] 6.5 验证 `config.md` 生成正确性

## 7. 文档更新
- [x] 7.1 更新项目 README，说明新的配置管理命令
- [x] 7.2 确保 `config.md` 模板内容完整准确
