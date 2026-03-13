# Aide 文档

Aide 是一个命令行工作流辅助工具，用于任务进度追踪、流程管理和待定项决策确认。

## 文档目录

- [安装和构建指南](./install.md) - 环境要求、编译构建、部署说明
- [命令参考](./commands.md) - 所有命令和子命令的详细说明
- [数据格式](./data-formats.md) - 配置文件和状态文件格式说明

## 快速开始

```bash
# 初始化项目
aide init

# 查看/修改配置
aide config get task.source
aide config set task.source "my-task.md"

# 配置管理
aide config reset          # 重置配置到默认值（自动备份）
aide config update         # 更新配置到最新版本

# 开始任务流程
aide flow start task-optimize "开始任务优化"
aide flow next-step "完成第一步"
aide flow next-part flow-design "进入流程设计"

# 待定项确认
aide decide submit pending.json
aide decide result
```

## 命令概览

| 命令 | 说明 |
|------|------|
| `aide init` | 初始化 .aide 目录与默认配置 |
| `aide config` | 配置管理（get/set/reset/update） |
| `aide flow` | 进度追踪与 git 集成 |
| `aide decide` | 待定项确认与决策记录 |
