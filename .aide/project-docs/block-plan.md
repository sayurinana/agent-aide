# 区块计划

## 项目概况

- 项目名称：ccoptimize（Aide 工具开发项目）
- 主要语言：Python 3.14
- 框架/工具：uv 虚拟环境管理
- 文件总数：约 200（核心项目，排除第三方）
- 空目录数：约 10
- 代码行数：约 5000 行（核心项目）

## 完整目录树（简化版）

```
ccoptimize/
├── .aide/                       项目 Aide 配置
│   ├── config.toml
│   ├── diagrams/
│   ├── flow-status.json
│   └── logs/
├── aide-program/                Aide CLI 核心程序
│   ├── aide/                    Python 包
│   ├── bin/                     可执行脚本
│   ├── docs/                    命令文档
│   └── lib/                     第三方库
├── aide-marketplace/            插件市场组件
│   └── aide-plugin/             Aide 插件
├── anthropic-agent-skills/      [ignored] 第三方技能库
├── cache/                       [ignored] 缓存目录
├── .cache/                      [ignored] 缓存目录
├── .venv/                       [ignored] 虚拟环境
├── test-cache/                  [ignored] 测试缓存
├── discuss/                     讨论文档目录
├── docs/                        项目文档
├── reply/                       回复文档目录
├── statements/                  声明文档目录
├── AGENTS.md
├── CHANGELOG.md
├── CLAUDE.md
├── README.md
├── requirements.txt
└── task-now.md
```

## 区块划分

### 区块 1：aide-program-core

- 路径：aide-program/aide/core/
- 文件数：约 5
- 空目录：0
- 状态：待处理
- 说明：Aide 核心配置和输出模块

### 区块 2：aide-program-env

- 路径：aide-program/aide/env/
- 文件数：约 15
- 空目录：0
- 状态：待处理
- 说明：环境检测和管理模块

### 区块 3：aide-program-flow

- 路径：aide-program/aide/flow/
- 文件数：约 10
- 空目录：0
- 状态：待处理
- 说明：进度追踪和流程控制模块

### 区块 4：aide-program-decide

- 路径：aide-program/aide/decide/
- 文件数：约 12
- 空目录：0
- 状态：待处理
- 说明：待定项确认 Web 服务模块

### 区块 5：aide-plugin-commands

- 路径：aide-marketplace/aide-plugin/commands/
- 文件数：约 8
- 空目录：0
- 状态：待处理
- 说明：Aide 插件斜杠命令定义

### 区块 6：aide-plugin-skills

- 路径：aide-marketplace/aide-plugin/skills/
- 文件数：约 5
- 空目录：0
- 状态：待处理
- 说明：Aide 插件技能定义

## 进度追踪

- [x] 区块 1：aide-program-core
- [x] 区块 2：aide-program-env
- [x] 区块 3：aide-program-flow
- [x] 区块 4：aide-program-decide
- [x] 区块 5：aide-plugin-commands
- [x] 区块 6：aide-plugin-skills
- [x] 总导览文档

## 更新记录

- 2025-12-17：创建区块计划
- 2025-12-17：完成所有区块文档和总导览
