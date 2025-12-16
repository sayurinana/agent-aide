# aide-program（核心程序）

> 路径：`aide-program/`
> 最后更新：2025-12-17

## 概述

Aide 命令行工具的核心实现，提供环境检测、流程追踪、待定项确认等功能。使用 Python 3.11+ 编写，通过 uv 管理虚拟环境和依赖。

## 目录结构

```
aide-program/
├── .aide/                           程序级 Aide 配置
│   ├── config.toml                  配置文件
│   ├── decisions/                   [空目录] 决策记录
│   └── logs/                        [空目录] 日志归档
├── aide/                            Python 源码包
│   ├── __init__.py                  包入口（docstring）
│   ├── __main__.py                  模块入口
│   ├── main.py                      CLI 主入口（约 440 行）
│   ├── aide.sh                      Shell 启动脚本
│   ├── core/                        核心基础模块
│   │   ├── __init__.py              模块标识
│   │   ├── config.py                配置管理（约 390 行）
│   │   └── output.py                输出格式工具（25 行）
│   ├── env/                         环境检测模块
│   │   ├── __init__.py              模块标识
│   │   ├── manager.py               环境管理器（约 370 行）
│   │   ├── registry.py              模块注册表（55 行）
│   │   └── modules/                 检测模块实现
│   │       ├── __init__.py          模块集合标识
│   │       ├── base.py              模块基类（90 行）
│   │       ├── python.py            Python 检测（59 行）
│   │       ├── uv.py                uv 检测（53 行）
│   │       ├── venv.py              虚拟环境（81 行）
│   │       ├── requirements.py      依赖管理（89 行）
│   │       ├── rust.py              Rust 检测（99 行）
│   │       ├── node.py              Node.js 检测（94 行）
│   │       ├── flutter.py           Flutter 检测（133 行）
│   │       ├── android.py           Android 检测（147 行）
│   │       └── node_deps.py         Node 依赖（142 行）
│   ├── flow/                        流程追踪模块
│   │   ├── __init__.py              模块入口
│   │   ├── types.py                 数据结构（103 行）
│   │   ├── tracker.py               流程追踪器（233 行）
│   │   ├── storage.py               状态存储（147 行）
│   │   ├── validator.py             流程校验（55 行）
│   │   ├── git.py                   Git 集成（79 行）
│   │   ├── hooks.py                 环节钩子（148 行）
│   │   ├── errors.py                错误类型（9 行）
│   │   └── utils.py                 工具函数（19 行）
│   └── decide/                      待定项确认模块
│       ├── __init__.py              模块导出
│       ├── types.py                 数据结构（324 行）
│       ├── cli.py                   CLI 处理（134 行）
│       ├── storage.py               数据存储（164 行）
│       ├── server.py                HTTP 服务（271 行）
│       ├── handlers.py              请求处理（155 行）
│       ├── daemon.py                后台服务（48 行）
│       ├── errors.py                错误类型（7 行）
│       └── web/                     前端资源
│           ├── index.html           HTML 页面（50 行）
│           ├── style.css            样式（345 行）
│           └── app.js               交互逻辑（321 行）
├── bin/                             可执行脚本
│   ├── aide                         Unix 启动脚本（16 行）
│   ├── aide.bat                     Windows 批处理
│   └── aide.sh                      Shell 脚本
├── docs/                            程序文档
│   ├── README.md                    文档索引
│   ├── commands/                    命令文档
│   │   ├── env.md                   环境命令
│   │   ├── flow.md                  流程命令
│   │   ├── flow/                    flow 子文档
│   │   │   ├── README.md
│   │   │   ├── cli.md
│   │   │   ├── git.md
│   │   │   ├── hooks.md
│   │   │   ├── state-and-storage.md
│   │   │   ├── validation.md
│   │   │   └── verification.md
│   │   ├── decide.md                待定项命令
│   │   ├── decide/                  decide 子文档
│   │   │   ├── README.md
│   │   │   ├── cli.md
│   │   │   ├── server.md
│   │   │   ├── storage.md
│   │   │   ├── verification.md
│   │   │   └── web.md
│   │   └── init.md                  初始化命令
│   └── formats/                     数据格式文档
│       ├── config.md                配置格式
│       └── data.md                  数据格式
├── lib/                             依赖库
│   └── plantuml.jar                 PlantUML（二进制）
├── .venv/                           [ignored] 虚拟环境
├── requirements.txt                 Python 依赖
└── .gitignore                       忽略规则
```

## 文件清单

| 文件 | 类型 | 说明 |
|------|------|------|
| aide/__init__.py | 源码 | 包入口，定义 docstring |
| aide/__main__.py | 源码 | 模块入口，调用 main() |
| aide/main.py | 源码 | CLI 主入口，命令行解析和处理器 |
| aide/core/config.py | 源码 | 配置管理，TOML 读写，.aide 目录维护 |
| aide/core/output.py | 源码 | 输出格式工具（✓/⚠/✗/→） |
| aide/env/manager.py | 源码 | 环境管理器，协调模块检测和修复 |
| aide/env/registry.py | 源码 | 模块注册表，管理检测模块 |
| aide/env/modules/base.py | 源码 | 模块基类，定义检测接口 |
| aide/env/modules/python.py | 源码 | Python 版本检测 |
| aide/env/modules/uv.py | 源码 | uv 包管理器检测 |
| aide/env/modules/venv.py | 源码 | 虚拟环境检测和创建 |
| aide/env/modules/requirements.py | 源码 | 依赖文件检测和安装 |
| aide/env/modules/rust.py | 源码 | Rust 工具链检测 |
| aide/env/modules/node.py | 源码 | Node.js 运行时检测 |
| aide/env/modules/flutter.py | 源码 | Flutter SDK 检测 |
| aide/env/modules/android.py | 源码 | Android SDK 检测 |
| aide/env/modules/node_deps.py | 源码 | Node 项目依赖检测 |
| aide/flow/types.py | 源码 | 流程状态数据结构 |
| aide/flow/tracker.py | 源码 | 流程追踪器核心逻辑 |
| aide/flow/storage.py | 源码 | 状态文件读写和归档 |
| aide/flow/validator.py | 源码 | 环节跳转校验 |
| aide/flow/git.py | 源码 | Git 操作封装 |
| aide/flow/hooks.py | 源码 | PlantUML/CHANGELOG 钩子 |
| aide/flow/errors.py | 源码 | FlowError 异常类 |
| aide/flow/utils.py | 源码 | 时间戳和文本处理 |
| aide/decide/types.py | 源码 | 待定项数据结构和校验 |
| aide/decide/cli.py | 源码 | submit/result 命令处理 |
| aide/decide/storage.py | 源码 | pending/result 文件管理 |
| aide/decide/server.py | 源码 | HTTP 服务器生命周期 |
| aide/decide/handlers.py | 源码 | API 和静态资源处理 |
| aide/decide/daemon.py | 源码 | 后台服务入口 |
| aide/decide/errors.py | 源码 | DecideError 异常类 |
| aide/decide/web/* | 前端 | Web 界面资源 |
| bin/aide | 脚本 | Unix 启动脚本 |
| lib/plantuml.jar | 二进制 | PlantUML 流程图工具 |
| requirements.txt | 配置 | tomli-w 依赖 |

## 核心组件

### ConfigManager (aide/core/config.py:240)

配置管理器，负责 .aide 目录和配置文件的维护。

- **职责**：
  - 创建和维护 .aide/ 目录结构
  - 读写 config.toml 配置文件
  - 管理 .gitignore 中的 .aide/ 忽略项
- **关键方法**：
  - `ensure_config()` - 确保配置文件存在
  - `load_config()` - 加载配置
  - `get_value(key)` - 读取点分隔键值
  - `set_value(key, value)` - 设置键值（保留注释）

### EnvManager (aide/env/manager.py:53)

环境管理器，协调各检测模块的检测和修复。

- **职责**：
  - 加载和管理检测模块
  - 执行环境检测（ensure）
  - 处理模块配置验证
- **关键方法**：
  - `ensure()` - 检测并修复环境
  - `list_modules()` - 列出可用模块
  - `set_modules()` - 设置启用模块
  - `set_module_config()` - 设置模块配置

### BaseModule (aide/env/modules/base.py:37)

环境检测模块基类，定义统一接口。

- **属性**：
  - `info` - 模块元信息（ModuleInfo）
- **方法**：
  - `check(config, root)` - 检测环境
  - `ensure(config, root)` - 修复环境
  - `validate_config(config)` - 验证配置

### FlowTracker (aide/flow/tracker.py:20)

流程追踪器，编排 flow 动作的完整流程。

- **职责**：
  - 协调校验、钩子、存储、Git 提交
  - 管理任务状态转换
- **关键方法**：
  - `start(phase, summary)` - 开始新任务
  - `next_step(summary)` - 步骤前进
  - `back_step(reason)` - 步骤回退
  - `next_part(phase, summary)` - 环节前进
  - `back_part(phase, reason)` - 环节回退
  - `issue(description)` - 记录问题
  - `error(description)` - 记录错误

### FlowStorage (aide/flow/storage.py:16)

流程状态存储，管理 flow-status.json 文件。

- **职责**：
  - 原子化读写状态文件
  - 文件锁管理
  - 状态归档
- **关键方法**：
  - `load_status()` - 加载当前状态
  - `save_status(status)` - 保存状态
  - `archive_existing_status()` - 归档旧状态
  - `list_all_tasks()` - 列出所有任务

### DecideServer (aide/decide/server.py:26)

待定项确认 HTTP 服务器。

- **职责**：
  - 启动和管理 HTTP 服务
  - 端口探测和配置读取
  - 服务生命周期控制
- **关键方法**：
  - `start()` - 交互式启动
  - `start_daemon(pid)` - 后台启动
  - `stop(reason)` - 停止服务

## 接口说明

### CLI 命令

| 命令 | 说明 |
|------|------|
| `aide init` | 初始化 .aide 目录 |
| `aide env ensure` | 检测并修复环境 |
| `aide env list` | 列出可用模块 |
| `aide env set` | 设置环境配置 |
| `aide config get <key>` | 读取配置 |
| `aide config set <key> <value>` | 写入配置 |
| `aide flow start <phase> "<summary>"` | 开始任务 |
| `aide flow next-step "<summary>"` | 步骤前进 |
| `aide flow next-part <phase> "<summary>"` | 环节前进 |
| `aide flow status` | 查看状态 |
| `aide flow list` | 列出任务 |
| `aide flow show <task_id>` | 查看任务详情 |
| `aide decide submit <file>` | 提交待定项 |
| `aide decide result` | 获取决策结果 |

### 环境检测模块

| 模块 | 类型 | 能力 | 说明 |
|------|------|------|------|
| python | A | check | Python 版本检测 |
| uv | A | check | uv 包管理器检测 |
| venv | B | check, ensure | 虚拟环境管理 |
| requirements | B | check, ensure | 依赖管理 |
| rust | A | check | Rust 工具链检测 |
| node | A | check | Node.js 检测 |
| flutter | A | check | Flutter SDK 检测 |
| android | A | check | Android SDK 检测 |
| node_deps | B | check, ensure | Node 项目依赖 |

## 依赖关系

- **内部依赖**：
  - `main.py` → `core/`, `env/`, `flow/`, `decide/`
  - `flow/tracker.py` → `flow/storage.py`, `flow/git.py`, `flow/hooks.py`, `flow/validator.py`
  - `decide/cli.py` → `decide/storage.py`, `decide/types.py`
  - `decide/server.py` → `decide/handlers.py`, `decide/storage.py`

- **外部依赖**：
  - `tomllib` (Python 3.11+ 内置)
  - `tomli_w` (TOML 写入)

## 注意事项

1. **虚拟环境**：运行 aide 命令前需要激活 `.venv` 虚拟环境或使用 `bin/aide` 脚本
2. **Git 集成**：flow 命令会自动执行 git add/commit，确保在 git 仓库中使用
3. **PlantUML**：流程图生成依赖 `lib/plantuml.jar`，需要 Java 环境
4. **端口配置**：decide 服务默认端口 3721，可通过配置修改
