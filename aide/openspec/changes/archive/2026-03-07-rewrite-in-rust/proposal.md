# Change: 将 Aide 从 Python 重写为 Rust

## Why

当前 Aide 使用 Python 3.11+ 实现，作为独立的命令行工具分发时存在运行环境依赖（Python + uv + 虚拟环境）。迁移到 Rust 可以编译为单一可执行文件，消除运行时依赖，提升启动速度和可靠性。项目已完成从 agent-aide 的分离，Rust 重写是当前阶段的核心任务。

## What Changes

- 使用 Rust（edition 2024）重新实现 `aide init`、`aide config`、`aide flow`、`aide decide` 全部命令
- 使用 `clap` 处理 CLI 参数解析
- 使用 `tokio` 实现 decide HTTP 服务器（异步）
- 使用 `serde` + `toml` + `serde_json` 处理配置和数据序列化
- Web 前端文件从文件系统加载（相对于可执行文件路径的 `web/` 目录，可通过参数覆盖）
- 完整保留 PlantUML 相关 hooks（需要 Java 环境）
- **不实现** `aide env` 及其所有子命令
- 完成实现后编写完整配套文档，保存到 `docs/` 目录

## Impact

- Affected specs: cli, config, flow, decide（全部为新建）
- Affected code: `src/` 目录（全新 Rust 实现），`aide/` 目录保留作为参考
- Affected docs: `docs/` 目录（全新文档）
- 参考实现: `aide/`（Python 源码）、`old-data-reference/`（原始文档）
