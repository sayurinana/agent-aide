# Tasks: add-plugin-sync

## 实施任务清单

### 1. 配置项更新

- [x] 1.1 更新 `aide/src/core/config.rs` 中的 `DEFAULT_CONFIG`，添加 `[plugin]` 配置段
- [x] 1.2 更新 `DEFAULT_CONFIG_MD`，添加 `[plugin]` 配置说明
- [x] 1.3 更新 `CURRENT_SCHEMA_VERSION` 从 3 递增到 4
- [x] 1.4 添加 `DEFAULT_PLUGIN_REPO_URL` 常量

### 2. Git 检测与克隆实现

- [x] 2.1 创建 `aide/src/core/git.rs` 模块，实现 Git 可用性检测
  - `fn is_git_available() -> bool`
- [x] 2.2 实现仓库克隆/更新函数
  - `fn clone_or_update_repo(repo_url: &str, target_dir: &Path) -> Result<(), String>`
- [x] 2.3 添加配置读取逻辑
  - 优先使用配置值，无则使用默认值

### 3. init 命令修改

- [x] 3.1 修改 `aide/src/cli/init.rs` 中的 `handle_init_global()`
  - 在 PlantUML 检测前添加插件仓库同步逻辑
- [x] 3.2 修改 `handle_init()`
  - 在 `create_aide_memory_files()` 后添加插件同步逻辑
- [x] 3.3 实现插件复制函数
  - `fn sync_plugins_to_project(project_root: &Path) -> bool`

### 4. 单元测试

- [x] 4.1 测试 Git 可用性检测
- [x] 4.2 测试配置读取优先级（配置值 vs 默认值）
- [x] 4.3 测试插件同步逻辑（使用临时目录模拟）

### 5. 集成测试

- [x] 5.1 测试完整全局初始化流程
  - Git 可用时克隆成功
  - Git 不可用时跳过并警告
- [x] 5.2 测试完整项目初始化流程
  - 全局仓库存在时同步成功
  - 全局仓库不存在时警告并跳过

### 6. 文档更新

- [x] 6.1 更新 `aide/docs/commands.md` 中的 init 命令说明
- [x] 6.2 更新 `aide-plugin/docs/skill/aide.md` 中的 init 命令文档

## 验证清单

- [x] 执行 `aide init --global` 成功克隆仓库
- [x] 执行 `aide init` 成功同步 commands 和 skills
- [x] 配置文件包含 `[plugin]` 段
- [x] `aide config get plugin.repo_url` 返回正确值
- [x] `aide config set plugin.repo_url <url>` 可修改仓库地址
- [x] Git 不可用时有明确警告
- [x] 全局仓库不存在时项目初始化有明确提示

## 依赖关系

```
1.1 → 1.3 → 2.3
2.1 → 2.2 → 3.1, 3.3
3.1 → 3.2
3.1, 3.2 → 4.1, 4.2, 4.3
4.x → 5.1, 5.2
5.x → 6.1, 6.2
```

## 并行化建议

- 任务 1.x（配置更新）和任务 2.x（Git 模块）可并行
- 任务 4.x（单元测试）可在对应功能实现后立即开始
- 任务 6.x（文档更新）可在所有功能完成后统一进行