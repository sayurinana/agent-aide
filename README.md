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
  ├── aide env    - 环境检测
  ├── aide config - 配置读写
  ├── aide flow   - 进度追踪 + git 集成（待实现）
  └── aide decide - 待定项 Web 确认（待实现）
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
│       │   └── aide/
│       │       └── SKILL.md
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
    ├── aide.sh               # Linux/Mac 入口
    ├── aide.bat              # Windows 入口
    ├── aide/                 # Python 代码
    │   ├── __init__.py
    │   ├── __main__.py
    │   ├── main.py           # CLI 路由
    │   ├── core/
    │   │   ├── config.py     # 配置管理
    │   │   └── output.py     # 输出格式
    │   └── env/
    │       └── ensure.py     # 环境检测
    └── docs/                 # 设计文档（给人）
        ├── README.md
        ├── commands/
        │   ├── env.md
        │   ├── flow.md
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
| aide skill | ✅ 设计完成 | aide 命令使用指南 |

执行文件位于 `aide-marketplace/aide-plugin/commands/` 和 `skills/aide/SKILL.md`

### 3.2 aide-program

| 子命令 | 状态 | 说明 |
|--------|------|------|
| aide init | ✅ 已实现 | 初始化 .aide 目录和配置 |
| aide env ensure | ✅ 已实现 | 环境检测与修复 |
| aide env ensure --runtime | ✅ 已实现 | 运行时环境检测 |
| aide config get/set | ✅ 已实现 | 配置读写 |
| aide flow | ⏳ 待实现 | 进度追踪 + git 集成 |
| aide decide | ⏳ 待实现 | 待定项 Web 确认 |

代码位于 `aide-program/aide/`

### 3.3 设计文档

| 区块 | 状态 | 位置 |
|------|------|------|
| 总导览 | ✅ 完成 | `docs/aide-overview.md` |
| aide-plugin 设计文档 | ✅ 完成 | `aide-marketplace/aide-plugin/docs/` |
| aide-program 设计文档 | ✅ 完成 | `aide-program/docs/` |

---

## 四、文档导航

### 4.1 快速了解 Aide 系统

1. 阅读 `docs/aide-overview.md` - 系统概述和架构
2. 阅读 `docs/为什么要更换到command+skill+专用处理程序.md` - 设计理念

### 4.2 了解/修改 Commands 或 Skill

1. 阅读 `aide-marketplace/aide-plugin/docs/README.md` - plugin 导览
2. 阅读对应 command 的设计文档

### 4.3 了解/修改 aide 程序

1. 阅读 `aide-program/docs/README.md` - program 导览
2. 阅读对应子命令的设计文档

### 4.4 了解数据格式

- 配置文件：`aide-program/docs/formats/config.md`
- 数据格式：`aide-program/docs/formats/data.md`

---

## 五、待完成工作

### 5.1 aide flow 实现

**功能**：进度追踪 + git 自动提交 + 流程校验

**设计文档**：`aide-program/docs/commands/flow.md`

**主要工作**：
- 实现 `aide/flow/tracker.py` - 状态追踪
- 实现 `aide/flow/git.py` - git 集成
- 实现 `aide/flow/validator.py` - 流程校验
- 在 `main.py` 添加 CLI 路由

### 5.2 aide decide 实现

**功能**：待定项 Web 确认界面

**设计文档**：`aide-program/docs/commands/decide.md`

**主要工作**：
- 实现 `aide/decide/server.py` - HTTP 服务
- 实现 `aide/decide/web/` - React 前端
- 在 `main.py` 添加 CLI 路由

### 5.3 整体验证

完成 flow 和 decide 后，需要进行完整工作流验证：
1. `/aide:init` → `/aide:prep` → `/aide:exec` 完整流程测试
2. 验证 git 自动提交功能
3. 验证待定项 Web 界面

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

- 文档版本：1.0.0
- 更新日期：2025-01-15
- 项目阶段：设计完成，部分实现
