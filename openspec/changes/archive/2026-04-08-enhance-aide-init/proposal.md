# Change: 增强 aide init 初始化流程

## Why

当前 `aide init` 仅完成文件初始化，缺少 Git 仓库初始化和任务描述文档创建。用户需要手动执行额外步骤才能开始工作流，增加了使用门槛。

## What Changes

- 在非 git 仓库中自动执行 `git init`、`git add .` 和初始提交
- 创建并切换到常驻分支（`branch.resident` 配置）
- 从模板文件创建任务描述文档（从 `task.template` 复制到 `task.description_file`）

## Impact

- 受影响规格：`cli`
- 受影响代码：
  - `aide/src/cli/init.rs` - 主要修改
  - `aide/src/flow/git.rs` - 可能需要新增辅助函数