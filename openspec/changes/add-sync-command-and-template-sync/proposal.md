# Change: 新增 aide sync 命令与模板同步功能

## Why

当前 aide 工具缺少独立的仓库同步命令，用户需要执行 `aide init --global` 才能更新全局仓库，这会导致不必要的重复初始化操作。同时，项目初始化时未同步模板文件，用户无法获取最新的模板资源。

## What Changes

- 新增 `aide sync` 命令，用于独立同步全局仓库
- 在 `aide init` 时同步模板文件到项目目录
- 新增配置项 `template.sync_strategy`，支持四种同步策略
- 更新默认仓库地址为 HTTPS 协议

## Impact

- 受影响规格：`cli`
- 受影响代码：
  - `aide/src/main.rs` - 命令定义
  - `aide/src/cli/init.rs` - 初始化逻辑
  - `aide/src/core/config.rs` - 配置定义
  - `aide/src/cli/sync.rs` - 新增文件