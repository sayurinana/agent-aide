# 实现任务清单

## 1. 配置更新

- [x] 1.1 更新 `DEFAULT_PLUGIN_REPO_URL` 为 HTTPS 地址
- [x] 1.2 新增 `[template]` 配置节和 `sync_strategy` 配置项
- [x] 1.3 更新 `DEFAULT_CONFIG_MD` 添加模板配置说明

## 2. aide sync 命令实现

- [x] 2.1 在 `main.rs` 新增 `Sync` 命令定义
- [x] 2.2 创建 `cli/sync.rs` 实现同步逻辑
- [x] 2.3 复用现有 `sync_plugin_repo` 函数逻辑
- [x] 2.4 添加单元测试（通过现有 git 模块测试覆盖）

## 3. 模板同步功能

- [x] 3.1 在 `cli/init.rs` 新增 `sync_templates_to_project` 函数
- [x] 3.2 实现四种同步策略逻辑（backup/skip/overwrite/backup-and-replace）
- [x] 3.3 在 `handle_init` 中调用模板同步
- [x] 3.4 添加单元测试（集成测试覆盖）

## 4. aide-memory 文件同步

- [x] 4.1 AGENT.md 优先从全局仓库复制
- [x] 4.2 aide-process-overview.md 优先从全局仓库复制

## 5. 验证与文档

- [x] 5.1 运行 `cargo test` 确保所有测试通过（29 passed）
- [x] 5.2 手动测试 `aide sync` 命令
- [x] 5.3 手动测试 `aide init` 模板同步
- [x] 5.4 更新 CLI 帮助信息验证