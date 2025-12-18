# 区块计划

> 最后更新：2025-12-19

## 项目概况

- 项目名称：agent-aide（Aide 工作流工具项目）
- 主要语言：Python
- 总文件数：161（排除忽略项）
- 总目录数：54（含 3 个空目录）
- 被忽略项：7 个目录
- 代码行数：约 5350 行（Python）

## 完整目录树（简化版 - 前两层）

```
agent-aide/
├── .aide/                       Aide 配置和数据目录
│   ├── config.toml              项目配置文件
│   ├── branches.json            分支概况数据
│   ├── branches.md              分支概况文档
│   ├── decisions/               [空目录] 待定项决策记录
│   ├── diagrams/                [空目录] PlantUML 流程图
│   ├── task-plans/              [空目录] 复杂任务计划文档
│   ├── logs/                    历史任务归档
│   └── project-docs/            项目文档目录（本文档所在）
├── aide-marketplace/            Aide 插件市场
│   ├── .claude-plugin/          市场配置
│   └── aide-plugin/             Aide 插件源码（版本 2.1.3）
├── aide-program/                Aide 主程序
│   ├── aide/                    Python 源码目录
│   ├── bin/                     可执行脚本
│   ├── docs/                    程序文档
│   ├── lib/                     依赖库（plantuml.jar）
│   ├── .venv/                   [ignored] 虚拟环境
│   ├── requirements.txt         依赖清单
│   └── .gitignore               忽略规则
├── anthropic-agent-skills/      [ignored] Anthropic Agent Skills（第三方）
├── cache/                       临时缓存目录
├── .cache/                      [ignored] 隐藏缓存目录
├── test-cache/                  [ignored] 测试缓存目录
├── .venv/                       [ignored] 根目录虚拟环境
├── __pycache__/                 [ignored] Python 字节码缓存
├── docs/                        项目文档目录
│   └── reference/               参考文档（7 个文件）
├── AGENTS.md                    Agent 配置说明
├── CHANGELOG.md                 变更日志
├── CLAUDE.md                    Claude 配置指令
├── README.md                    项目说明
├── requirements.txt             依赖清单
├── task-now.md                  当前任务文档
└── .gitignore                   Git 忽略规则
```

## 区块划分

### 区块 1：aide-program（核心程序）

- **路径**：`aide-program/`
- **文件数**：约 72 个文件（排除 .venv 和 __pycache__）
- **空目录**：0 个
- **状态**：已完成
- **说明**：Aide 命令行工具的核心实现，包括：
  - `aide/core/` - 核心功能模块（配置、输出）
  - `aide/env/` - 环境检测模块
  - `aide/flow/` - 流程追踪模块（含分支管理）
  - `aide/decide/` - 待定项确认模块
  - `bin/` - 可执行脚本
  - `docs/` - 程序文档

### 区块 2：aide-marketplace（插件市场）

- **路径**：`aide-marketplace/`
- **文件数**：约 39 个文件
- **空目录**：0 个
- **状态**：已完成
- **说明**：Aide 插件市场和 aide-plugin 源码（版本 2.1.3），包括：
  - `.claude-plugin/` - 市场配置
  - `aide-plugin/commands/` - 斜杠命令定义（8 个）
  - `aide-plugin/skills/` - 技能定义（5 个，含 readme-templates 和 rework）
  - `aide-plugin/docs/` - 插件文档

### 区块 3：项目配置与文档

- **路径**：`.aide/`, `docs/`, 根目录配置文件
- **文件数**：约 50 个文件
- **空目录**：3 个（.aide/decisions/, .aide/diagrams/, .aide/task-plans/）
- **状态**：已完成
- **说明**：项目级配置、Aide 数据目录和项目文档资源，包括：
  - `.aide/config.toml` - 项目配置
  - `.aide/branches.json/.md` - 分支概况数据
  - `.aide/decisions/` - [空目录] 待定项决策记录
  - `.aide/diagrams/` - [空目录] PlantUML 流程图
  - `.aide/task-plans/` - [空目录] 复杂任务计划文档
  - `.aide/logs/` - 历史任务归档
  - `docs/reference/` - 指南和概览文档（7 个）
  - 根目录 `.md` 文件

## 被忽略项清单

| 目录 | 说明 |
|------|------|
| `anthropic-agent-skills/` | 第三方技能库（Git submodule） |
| `cache/` | 临时缓存目录（部分保留） |
| `.cache/` | 隐藏缓存目录 |
| `test-cache/` | 测试缓存目录 |
| `.venv/` | 根目录虚拟环境 |
| `__pycache__/` | Python 字节码缓存 |
| `aide-program/.venv/` | 程序虚拟环境 |

## 进度追踪

- [x] 区块 1：aide-program
- [x] 区块 2：aide-marketplace
- [x] 区块 3：项目配置与文档
- [x] 总导览文档生成
