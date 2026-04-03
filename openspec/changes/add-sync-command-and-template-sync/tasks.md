# 实现任务清单

## 1. 配置更新

- [ ] 1.1 更新 `DEFAULT_PLUGIN_REPO_URL` 为 HTTPS 地址
- [ ] 1.2 新增 `[template]` 配置节和 `sync_strategy` 配置项
- [ ] 1.3 更新 `DEFAULT_CONFIG_MD` 添加模板配置说明

## 2. aide sync 命令实现

- [ ] 2.1 在 `main.rs` 新增 `Sync` 命令定义
- [ ] 2.2 创建 `cli/sync.rs` 实现同步逻辑
- [ ] 2.3 复用现有 `sync_plugin_repo` 函数逻辑
- [ ] 2.4 添加单元测试

## 3. 模板同步功能

- [ ] 3.1 在 `cli/init.rs` 新增 `sync_templates_to_project` 函数
- [ ] 3.2 实现四种同步策略逻辑（backup/skip/overwrite/backup-and-replace）
- [ ] 3.3 在 `handle_init` 中调用模板同步
- [ ] 3.4 添加单元测试

## 4. 验证与文档

- [ ] 4.1 运行 `cargo test` 确保所有测试通过
- [ ] 4.2 手动测试 `aide sync` 命令
- [ ] 4.3 手动测试 `aide init` 模板同步
- [ ] 4.4 更新 CLI 帮助信息验证