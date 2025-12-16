# 区块计划

> 最后更新：2025-12-17

## 项目概况

- 项目名称：ccoptimize（Aide 工作流工具优化项目）
- 主要语言：Python
- 总文件数：132（排除忽略项）
- 总目录数：53（含 5 个空目录）
- 被忽略项：7 个目录
- 代码行数：约 20082 行

## 完整目录树（简化版 - 前两层）

```
ccoptimize/
├── .aide/                       Aide 配置和数据目录
│   ├── config.toml              项目配置文件
│   ├── decisions/               [空目录] 待定项决策记录
│   ├── diagrams/                流程图目录（含 .puml 和 .png）
│   ├── flow-status.json         当前任务进度
│   ├── logs/                    历史任务归档
│   └── project-docs/            项目文档目录（本文档所在）
├── aide-marketplace/            Aide 插件市场
│   ├── .claude-plugin/          插件市场配置
│   └── aide-plugin/             Aide 插件源码
├── aide-program/                Aide 主程序
│   ├── .aide/                   程序级配置
│   ├── aide/                    Python 源码目录
│   ├── bin/                     可执行脚本
│   ├── docs/                    程序文档
│   ├── lib/                     依赖库（plantuml.jar）
│   ├── .venv/                   [ignored] 虚拟环境
│   ├── requirements.txt         依赖清单
│   └── .gitignore               忽略规则
├── anthropic-agent-skills/      [ignored] Anthropic Agent Skills（第三方）
├── cache/                       [ignored] 缓存目录
├── .cache/                      [ignored] 隐藏缓存目录
├── test-cache/                  [ignored] 测试缓存目录
├── .venv/                       [ignored] 根目录虚拟环境
├── __pycache__/                 [ignored] Python 字节码缓存
├── discuss/                     [空目录] 讨论文档目录
├── docs/                        项目文档目录
│   ├── 01-自定义斜杠命令指南.md
│   ├── 02-技能指南.md
│   ├── 03-插件指南.md
│   ├── 04-插件市场指南.md
│   ├── aide-overview.md
│   ├── project-details.md
│   └── 为什么要更换到command+skill+专用处理程序.md
├── reply/                       [空目录] 回复目录
├── statements/                  声明文档目录
│   ├── old-task-section.md
│   └── optimize.md
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
- **文件数**：约 50 个源码文件
- **空目录**：2 个（decisions/, logs/）
- **状态**：待处理
- **说明**：Aide 命令行工具的核心实现，包括：
  - `aide/core/` - 核心功能模块（配置、输出）
  - `aide/env/` - 环境检测模块
  - `aide/flow/` - 流程追踪模块
  - `aide/decide/` - 待定项确认模块
  - `bin/` - 可执行脚本
  - `docs/` - 程序文档

### 区块 2：aide-marketplace（插件市场）

- **路径**：`aide-marketplace/`
- **文件数**：约 15 个文件
- **空目录**：0 个
- **状态**：待处理
- **说明**：Aide 插件市场和 aide-plugin 源码，包括：
  - `.claude-plugin/` - 市场配置
  - `aide-plugin/commands/` - 斜杠命令定义
  - `aide-plugin/skills/` - 技能定义
  - `aide-plugin/docs/` - 插件文档

### 区块 3：项目配置（根目录配置）

- **路径**：`.aide/`, 根目录配置文件
- **文件数**：约 15 个文件
- **空目录**：1 个（decisions/）
- **状态**：待处理
- **说明**：项目级配置和 Aide 数据目录，包括：
  - `.aide/config.toml` - 项目配置
  - `.aide/diagrams/` - 流程图
  - `.aide/flow-status.json` - 任务状态
  - 根目录 `.md` 文件

### 区块 4：项目文档与资源

- **路径**：`docs/`, `discuss/`, `reply/`, `statements/`
- **文件数**：约 10 个文件
- **空目录**：2 个（discuss/, reply/）
- **状态**：待处理
- **说明**：项目文档和资源文件目录，包括：
  - `docs/` - 指南和概览文档
  - `statements/` - 声明文档
  - `discuss/` - [空目录] 讨论记录
  - `reply/` - [空目录] 回复记录

## 被忽略项清单

| 目录 | 说明 |
|------|------|
| `anthropic-agent-skills/` | 第三方技能库（Git submodule） |
| `cache/` | 临时缓存目录 |
| `.cache/` | 隐藏缓存目录 |
| `test-cache/` | 测试缓存目录 |
| `.venv/` | 根目录虚拟环境 |
| `__pycache__/` | Python 字节码缓存 |
| `aide-program/.venv/` | 程序虚拟环境 |

## 进度追踪

- [x] 区块 1：aide-program
- [x] 区块 2：aide-marketplace
- [x] 区块 3：项目配置与文档（合并区块 3 和 4）
- [x] 总导览文档生成
