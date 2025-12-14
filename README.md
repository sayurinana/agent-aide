# Aide 项目状态文档

## 一、项目简介

Aide 是一套面向 Claude Code 的工作流辅助体系，旨在解决 AI 辅助开发中的信息过载、操作不确定性和流程耦合问题。

### 1.1 核心理念

将原本堆积在 CLAUDE.md 中的规则和流程转化为按需触发的模块化单元：

| 原有问题 | Aide 解决方案 |
|----------|---------------|
| CLAUDE.md 信息过载 | 流程按需触发（Command） |
| 操作不确定性 | 程序化封装（aide 程序） |
| 输出信息冗余 | 精简输出，静默即成功 |
| 流程耦合 | Command + Skill 分离职责 |

### 1.2 系统架构

```
用户
  │
  ▼
aide-plugin (Claude Code 插件)
  ├── Commands: /aide:init, /aide:prep, /aide:exec
  │   └── 定义流程（做什么、按什么顺序）
  └── Skill: aide
      └── 定义工具使用方法（怎么调用）
  │
  ▼ 调用
aide-program (命令行工具)
  ├── aide init   - 初始化配置
  ├── aide env    - 环境检测（模块化）
  ├── aide config - 配置读写
  ├── aide flow   - 进度追踪 + git 集成
  └── aide decide - 待定项 Web 确认
      ├── aide decide submit '<json>' - 提交待定项并启动 Web 服务
      └── aide decide result          - 获取用户决策结果
```

---

## 二、项目结构

```
ccoptimize/
├── CLAUDE.md                 # 项目级指令
├── README.md                 # 本文档
│
├── docs/                     # 总导览
│   ├── aide-overview.md      # Aide 系统概述
│   ├── 01-自定义斜杠命令指南.md
│   ├── 02-技能指南.md
│   ├── 03-插件指南.md
│   ├── 04-插件市场指南.md
│   └── 为什么要更换到command+skill+专用处理程序.md
│
├── statements/               # 项目声明文档
│   └── optimize.md           # 沟通准则
│
├── aide-marketplace/         # Claude Code 插件市场
│   ├── .claude-plugin/
│   │   └── marketplace.json
│   └── aide-plugin/          # Aide 插件
│       ├── .claude-plugin/
│       │   └── plugin.json
│       ├── commands/         # 执行文件（给 LLM）
│       │   ├── init.md
│       │   ├── prep.md
│       │   └── exec.md
│       ├── skills/
│       │   ├── aide/
│       │   │   └── SKILL.md      # 基础命令指南
│       │   └── env-config/
│       │       └── SKILL.md      # 环境配置详细指南（按需触发）
│       └── docs/             # 设计文档（给人）
│           ├── README.md
│           ├── commands/
│           │   ├── init.md
│           │   ├── prep.md
│           │   └── exec.md
│           └── skill/
│               └── aide.md
│
└── aide-program/             # Aide 命令行工具
    ├── bin/
    │   ├── aide.sh           # Linux/Mac 入口
    │   └── aide.bat          # Windows 入口
    ├── aide/                 # Python 代码
    │   ├── __init__.py
    │   ├── __main__.py
    │   ├── main.py           # CLI 路由
    │   ├── core/
    │   │   ├── config.py     # 配置管理
    │   │   └── output.py     # 输出格式
    │   ├── env/
    │       ├── manager.py    # 环境管理器
    │       ├── registry.py   # 模块注册表
    │       └── modules/      # 环境检测模块
    │           ├── base.py
    │           ├── python.py, uv.py
    │           ├── rust.py, node.py, flutter.py
    │           ├── android.py, node_deps.py
    │           ├── venv.py, requirements.py
    │           └── ...
    │   └── flow/             # 进度追踪（已实现）
    │       └── ...
    └── docs/                 # 设计文档（给人）
        ├── README.md
        ├── commands/
        │   ├── env.md
        │   ├── flow.md
        │   ├── flow/          # flow 详细设计（交接包）
        │   ├── decide.md
        │   └── init.md
        └── formats/
            ├── config.md
            └── data.md
```

---

## 三、实现状态

### 3.1 aide-plugin

| 组件 | 状态 | 说明 |
|------|------|------|
| /aide:init | ✅ 设计完成 | 项目认知与环境初始化 |
| /aide:prep | ✅ 设计完成 | 任务准备流程 |
| /aide:exec | ✅ 设计完成 | 任务执行流程 |
| aide skill | ✅ 设计完成 | aide 基础命令指南 |
| env-config skill | ✅ 设计完成 | 环境配置详细指南（按需触发） |

执行文件位于 `aide-marketplace/aide-plugin/commands/` 和 `skills/`

**Skill 设计理念**：
- `aide` skill：始终加载，提供基础命令用法
- `env-config` skill：按需触发，仅在 `aide env ensure` 失败时使用

### 3.2 aide-program

| 子命令 | 状态 | 说明 |
|--------|------|------|
| aide init | ✅ 已实现 | 初始化 .aide 目录和配置 |
| aide env list | ✅ 已实现 | 列出所有可用模块 |
| aide env ensure | ✅ 已实现 | 模块化环境检测与修复 |
| aide env set | ✅ 已实现 | 设置环境配置（带验证） |
| aide env ensure --runtime | ✅ 已实现 | 运行时环境检测 |
| aide env ensure --modules | ✅ 已实现 | 指定模块检测 |
| aide env ensure --all | ✅ 已实现 | 全量检测（仅检查） |
| aide env ensure --verbose | ✅ 已实现 | 详细配置输出 |
| aide config get/set | ✅ 已实现 | 配置读写 |
| aide flow | ✅ 已实现 | 进度追踪 + git 集成 |
| aide decide submit | ✅ 已实现 | 提交待定项并启动 Web 服务 |
| aide decide result | ✅ 已实现 | 获取用户决策结果 |

代码位于 `aide-program/aide/`

### 3.3 环境检测模块

| 模块 | 类型 | 能力 | 说明 |
|------|------|------|------|
| python | A | check | Python 解释器版本 |
| uv | A | check | uv 包管理器 |
| rust | A | check | Rust 工具链（rustc + cargo） |
| node | A | check | Node.js 运行时 |
| flutter | A | check | Flutter SDK |
| android | A | check | Android SDK |
| venv | B | check, ensure | Python 虚拟环境 |
| requirements | B | check, ensure | Python 依赖管理 |
| node_deps | B | check, ensure | Node.js 项目依赖 |

- 类型A：无需配置即可检测
- 类型B：需要配置路径才能检测
- 支持模块实例化命名：`模块类型:实例名`（如 `node_deps:react`）

### 3.4 设计文档

| 区块 | 状态 | 位置 |
|------|------|------|
| 总导览 | ✅ 完成 | `docs/aide-overview.md` |
| aide-plugin 设计文档 | ✅ 完成 | `aide-marketplace/aide-plugin/docs/` |
| aide-program 设计文档 | ✅ 完成 | `aide-program/docs/` |

---

## 四、文档导航

### 4.1 快速了解 Aide 系统

1. 阅读 [Aide 系统概述](docs/aide-overview.md) - 系统概述和架构
2. 阅读 [为什么要更换到command+skill+专用处理程序](docs/为什么要更换到command+skill+专用处理程序.md) - 设计理念

### 4.2 了解/修改 Commands 或 Skill

1. 阅读 [aide-plugin 导览](aide-marketplace/aide-plugin/docs/README.md)
2. 阅读对应 command 的设计文档

### 4.3 了解/修改 aide 程序

1. 阅读 [aide-program 导览](aide-program/docs/README.md)
2. 阅读对应子命令的设计文档（如 [flow 子命令概览](aide-program/docs/commands/flow.md)）
3. 深入 flow 实现细节：[`aide-program/docs/commands/flow/README.md`](aide-program/docs/commands/flow/README.md)

### 4.4 了解数据格式

- 配置文件：[aide-program/docs/formats/config.md](aide-program/docs/formats/config.md)
- 数据格式：[aide-program/docs/formats/data.md](aide-program/docs/formats/data.md)

---

## 五、待完成工作

### 5.1 扩展环境模块（可选）

可按需添加更多环境检测模块：
- java - Java JDK 检测
- go - Go 语言检测
- docker - Docker 环境检测
- cargo_deps - Rust 项目依赖（类似 node_deps）
- pub_deps - Flutter/Dart 项目依赖

### 5.2 整体验证

进行完整工作流验证：
1. `/aide:init` → `/aide:prep` → `/aide:exec` 完整流程测试
2. 验证 git 自动提交功能
3. 验证待定项 Web 界面（aide decide）

---

## 六、开发约束

### 6.1 文档约束

- 设计文档（`docs/`）给人看，包含完整上下文和流程图
- 执行文件（`commands/`、`skills/`）给 LLM 看，聚焦执行指令
- aide-program 设计文档不包含代码实现，仅使用 PlantUML 流程图和伪代码

### 6.2 代码约束

- Python >= 3.11
- 使用 uv 管理虚拟环境和依赖
- 所有输出使用 `core/output.py` 中的函数（✓/⚠/✗/→ 前缀）
- 遵循静默原则：无输出 = 正常完成

### 6.3 语言约束

- 所有对话、思考、文档与注释使用简体中文

---

## 七、版本信息

- 文档版本：1.3.0
- 更新日期：2025-12-15
- 项目阶段：设计完成，核心功能已实现
- 最近更新：
  - aide decide 子命令实现（submit/result）
  - 支持 Web 界面待定项确认
  - 支持自定义监听地址（bind）和访问地址（url）配置
  - 推荐选项默认选中
  - aide flow 子命令实现
  - 新增环境模块：rust, node, flutter, android, node_deps
  - 支持模块实例化命名（多项目场景）
  - Skill 拆分：aide（基础）+ env-config（按需）
