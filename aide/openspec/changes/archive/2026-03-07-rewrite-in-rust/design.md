## Context

Aide 是一个命令行工作流辅助工具，当前使用 Python 3.11+ 实现。需要将其迁移到 Rust 以消除运行时依赖，实现单一可执行文件分发。Python 源码保留在 `aide/` 目录作为参考实现。

迁移范围包括 init、config、flow（含 git 集成和分支管理）、decide（含 HTTP 服务器和 Web UI）四个功能域，不包括 env 模块。

## Goals / Non-Goals

- Goals:
  - 用 Rust 实现与 Python 版本功能对等的 CLI 工具（除 env 外）
  - 保持与现有 `.aide/` 目录结构和数据格式的完全兼容
  - 单一可执行文件，无运行时依赖（Git、Java/PlantUML 为可选外部工具）
  - 异步 HTTP 服务器（tokio）
  - Web 前端文件从文件系统加载
  - 编写完整中文文档

- Non-Goals:
  - 不实现 `aide env` 及相关功能
  - 不改变现有数据格式（JSON/TOML schema 保持兼容）
  - 不重构 Web 前端（直接复用现有 HTML/CSS/JS）
  - 不添加 Python 版本中未有的新功能

## Decisions

### 1. CLI 框架: clap

- **决定**: 使用 `clap` (derive API) 处理命令行参数解析
- **理由**: Rust 生态最成熟的 CLI 框架，derive 宏简化定义，自动生成帮助信息
- **替代方案**: argh（更轻量但社区较小）、手动解析（工作量大）

### 2. 异步运行时: tokio

- **决定**: 使用 `tokio` 作为异步运行时，`axum` 作为 HTTP 框架
- **理由**: 用户明确要求异步实现；axum 是 tokio 生态的一流 HTTP 框架，API 简洁
- **替代方案**: tiny_http（同步，更简单但用户选择异步）、hyper（更底层）
- **注意**: 仅 decide 服务器部分使用异步，CLI 主逻辑使用 `#[tokio::main]` 但大部分操作仍为同步

### 3. 序列化: serde + toml + serde_json

- **决定**: 使用 `serde` 统一序列化框架，`toml` crate 处理配置，`serde_json` 处理状态数据
- **理由**: Rust 标准实践，类型安全，编译时检查

### 4. Web 前端文件加载

- **决定**: 从文件系统加载，默认路径为可执行文件所在目录下的 `web/`，可通过命令行参数 `--web-dir` 覆盖
- **理由**: 用户明确要求运行时从文件系统加载，便于自定义和调试
- **路径解析**: 相对于可执行文件路径（`std::env::current_exe()`），不是工作目录

### 5. 文件锁: fs2 或 fd-lock

- **决定**: 使用 `fs2` crate 实现文件锁（与 Python 版 fcntl/flock 对等）
- **理由**: 跨平台文件锁，API 简洁，与 Python 版行为一致
- **替代方案**: fd-lock（更轻量）、advisory-lock

### 6. 项目结构

- **决定**: 采用模块化 Rust 项目结构

```
src/
├── main.rs              # 入口 + CLI 定义
├── cli/                 # CLI 命令处理
│   ├── mod.rs
│   ├── init.rs
│   ├── config.rs
│   ├── flow.rs
│   └── decide.rs
├── core/                # 核心基础
│   ├── mod.rs
│   ├── config.rs        # ConfigManager
│   ├── output.rs        # 输出格式化
│   └── project.rs       # 项目根目录发现
├── flow/                # 流程追踪
│   ├── mod.rs
│   ├── types.rs         # 数据结构
│   ├── tracker.rs       # FlowTracker
│   ├── storage.rs       # FlowStorage
│   ├── validator.rs     # 校验器
│   ├── git.rs           # Git 操作
│   ├── branch.rs        # 分支管理
│   └── hooks.rs         # 环节钩子
├── decide/              # 待定项确认
│   ├── mod.rs
│   ├── types.rs         # 数据结构
│   ├── storage.rs       # 数据存储
│   ├── server.rs        # HTTP 服务器
│   └── handlers.rs      # 请求处理
└── utils.rs             # 通用工具函数
```

### 7. PlantUML 集成

- **决定**: 完整保留 PlantUML hooks，通过 `std::process::Command` 调用 PlantUML
- **理由**: 用户要求完整保留
- **JAR 路径解析**: 优先读取配置 `plantuml.jar_path`，为空时检查 `lib/plantuml.jar`（相对于可执行文件），最后尝试系统 PATH 中的 `plantuml` 命令

### 8. 后台进程管理

- **决定**: decide 服务器的后台模式通过 fork/spawn 子进程实现
- **Linux**: 使用 `std::process::Command` 以 detach 方式启动自身
- **PID 跟踪**: 写入 `.aide/decisions/server.json`

## Risks / Trade-offs

- **tokio 依赖体积**: 引入 tokio 会增加编译时间和二进制大小 → 可接受，仅用于 decide 服务器
- **TOML 注释保留**: Rust `toml` crate 在写入时不保留注释 → 使用 `toml_edit` crate 替代 `toml` 进行配置写入，保留注释
- **跨平台兼容**: Python 版本已处理 Windows 批处理脚本 → Rust 编译为原生可执行文件，天然跨平台
- **PlantUML Java 依赖**: 保留了外部 Java 依赖 → hooks 检测可用性，不可用时警告而非报错

## Migration Plan

1. 搭建 Rust 项目骨架（Cargo.toml 依赖、模块结构）
2. 实现 core 层（config、output、project root discovery）
3. 实现 init 命令
4. 实现 config get/set 命令
5. 实现 flow 数据类型和存储
6. 实现 flow 基础命令（start、next-step、back-step、status、list、show）
7. 实现 flow 高级命令（next-part、back-part、back-confirm、issue、error、clean）
8. 实现 git 集成和分支管理
9. 实现 flow hooks（PlantUML、CHANGELOG、cleanup）
10. 实现 decide 数据类型和存储
11. 实现 decide HTTP 服务器和 API
12. 复用 Web 前端文件
13. 编写文档
14. 集成测试

## Open Questions

- 无（关键决策已通过用户确认解决）
