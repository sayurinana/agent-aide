## 1. 配置系统变更

- [x] 1.1 更新 `DEFAULT_CONFIG` 常量：移除 `jar_path`，新增 `download_cache_path`、`clean_cache_after_install`、`install_path`、`download_url`
- [x] 1.2 更新 `DEFAULT_CONFIG_MD` 常量：同步更新 `[plantuml]` 节文档说明
- [x] 1.3 将 `CURRENT_SCHEMA_VERSION` 从 1 升级到 2
- [x] 1.4 在 `aide config update` 中实现 v1 → v2 迁移逻辑（移除 jar_path，添加新字段）

## 2. PlantUML 管理模块

- [x] 2.1 新建 `src/core/plantuml.rs` 模块
- [x] 2.2 实现 `get_plantuml_path(config) -> PathBuf`：根据配置拼接可执行文件路径
- [x] 2.3 实现 `check_plantuml(path) -> Option<String>`：检测可执行文件是否存在并返回版本号
- [x] 2.4 实现 `download_plantuml(config)`：从配置的 URL 下载压缩包到缓存目录
- [x] 2.5 实现 `install_plantuml(config)`：解压 tar.gz 到安装目录，验证结果，按配置清理缓存
- [x] 2.6 添加 reqwest（blocking + rustls-tls）、flate2、tar 依赖到 Cargo.toml

## 3. init --global 集成

- [x] 3.1 在 `handle_init_global()` 末尾添加 PlantUML 检测逻辑
- [x] 3.2 检测不可用时提示用户确认自动安装（读取 stdin）
- [x] 3.3 用户确认后调用下载和安装函数
- [x] 3.4 安装完成后输出版本信息

## 4. 版本命令集成

- [x] 4.1 修改 main.rs，自定义 `-V`/`--version` 处理逻辑
- [x] 4.2 输出 aide 版本后附加 PlantUML 状态信息（版本/路径/状态）
- [x] 4.3 PlantUML 不可用时显示安装提示

## 5. 流程图 hook 适配

- [x] 5.1 修改 `get_plantuml_command()`：使用新的可执行程序路径查找逻辑
- [x] 5.2 移除 jar_path 相关代码

## 6. 测试与文档

- [x] 6.1 为 plantuml 模块添加单元测试（路径拼接、版本解析）
- [x] 6.2 更新 `docs/data-formats.md` 中的配置格式说明
- [x] 6.3 更新 `docs/install.md` 中的 PlantUML 配置说明
- [x] 6.4 更新 `docs/commands.md` 中的 init --global 和 -V 说明
