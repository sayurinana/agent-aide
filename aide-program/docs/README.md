# Aide Program 设计文档

## 一、概述

aide-program 是 Aide 工作流体系的命令行工具，为 aide-plugin 提供底层支持。

### 1.1 解决的问题

| 问题 | 解决方案 |
|------|----------|
| 操作不确定性 | 程序化封装，固定输入输出 |
| 输出信息冗余 | 精简输出，静默即成功 |
| git 操作分散 | 集成到 flow 命令，自动提交 |
| 状态难追踪 | 统一的状态文件和日志 |

### 1.2 与 aide-plugin 的关系

```
┌─────────────────────────────────────────────────┐
│               aide-plugin                        │
│  Commands: 定义流程（做什么、按什么顺序）        │
│  Skill: 定义工具使用方法（怎么调用）             │
└─────────────────────────────────────────────────┘
                      │
                      ▼ 调用
┌─────────────────────────────────────────────────┐
│               aide-program                       │
│  实际执行操作，返回精简结果                      │
│                                                  │
│  ┌────────┐  ┌────────┐  ┌────────┐            │
│  │  env   │  │  flow  │  │ decide │            │
│  └────────┘  └────────┘  └────────┘            │
│  ┌────────┐  ┌────────┐                        │
│  │ config │  │  init  │                        │
│  └────────┘  └────────┘                        │
└─────────────────────────────────────────────────┘
```

---

## 二、子命令索引

| 子命令 | 设计文档 | 实现状态 | 职责 |
|--------|----------|----------|------|
| `aide init` | [commands/init.md](commands/init.md) | ✅ 已实现 | 初始化 .aide 目录 |
| `aide env ensure` | [commands/env.md](commands/env.md) | ✅ 已实现 | 环境检测与修复 |
| `aide env list` | [commands/env.md](commands/env.md) | ✅ 已实现 | 列出可用模块 |
| `aide env set` | [commands/env.md](commands/env.md) | ✅ 已实现 | 设置环境配置（带验证） |
| `aide config` | [formats/config.md](formats/config.md) | ✅ 已实现 | 配置读写 |
| `aide flow` | [commands/flow.md](commands/flow.md) | ⏳ 待实现 | 进度追踪与 git 集成 |
| `aide decide` | [commands/decide.md](commands/decide.md) | ⏳ 待实现 | 待定项 Web 确认 |

### 2.1 环境检测模块

| 模块 | 类型 | 说明 |
|------|------|------|
| python, uv | A | Python 运行时 |
| rust | A | Rust 工具链 |
| node | A | Node.js 运行时 |
| flutter | A | Flutter SDK |
| android | A | Android SDK |
| venv, requirements | B | Python 项目依赖 |
| node_deps | B | Node.js 项目依赖 |

- 类型A：无需配置即可检测
- 类型B：需要配置路径
- 支持模块实例化命名：`模块类型:实例名`

---

## 三、目录结构

```
aide-program/
├── aide.sh                  # Linux/Mac 入口脚本
├── aide.bat                 # Windows 入口脚本
├── docs/                    # 设计文档（本目录）
│   ├── README.md            # 导览（本文件）
│   ├── commands/            # 子命令设计文档
│   │   ├── env.md
│   │   ├── flow.md
│   │   ├── decide.md
│   │   └── init.md
│   └── formats/             # 数据格式文档
│       ├── config.md
│       └── data.md
└── aide/                    # Python 代码
    ├── __init__.py
    ├── __main__.py          # 支持 python -m aide
    ├── main.py              # CLI 解析与命令分发
    ├── core/
    │   ├── config.py        # 配置读写
    │   └── output.py        # 输出格式（✓/⚠/✗/→）
    ├── env/
    │   ├── manager.py       # 环境管理器
    │   ├── registry.py      # 模块注册表
    │   └── modules/         # 环境检测模块
    │       ├── base.py      # 模块基类
    │       ├── python.py, uv.py, rust.py
    │       ├── node.py, flutter.py, android.py
    │       ├── venv.py, requirements.py
    │       └── node_deps.py
    ├── flow/                # 待实现
    │   └── ...
    └── decide/              # 待实现
        └── ...
```

---

## 四、输出规范

### 4.1 前缀符号

| 前缀 | 函数 | 用途 |
|------|------|------|
| `✓` | `output.ok()` | 成功 |
| `⚠` | `output.warn()` | 警告（可继续） |
| `✗` | `output.err()` | 失败 |
| `→` | `output.info()` | 进行中/信息 |
| `[n/m]` | `output.step()` | 步骤进度 |

### 4.2 静默原则

**无输出 = 正常完成**

只有在需要反馈信息时才输出。

### 4.3 输出示例

```bash
# 成功
✓ 环境就绪 (python:3.12, uv:0.4.0)

# 警告
⚠ 已修复: 创建虚拟环境 .venv

# 错误
✗ Python 版本不满足要求 (需要 >=3.10, 当前 3.8)
  建议: 安装 Python 3.10+ 或使用 pyenv 管理版本

# 信息
→ 任务原文档: task-now.md
```

---

## 五、数据存储

### 5.1 存储位置

所有 aide 数据文件存放在项目根目录的 `.aide/` 下：

```
.aide/
├── config.toml          # 项目配置
├── flow-status.json     # 当前任务进度状态
├── decisions/           # 待定项决策记录
│   └── {timestamp}.json
└── logs/                # 操作日志
```

### 5.2 .gitignore 处理

- `aide init` 时自动检查 `.gitignore`
- 默认添加 `.aide/` 为忽略项

---

## 六、运行方式

### 6.1 通过入口脚本

```bash
# Linux/Mac
./aide-program/aide.sh <command> [args]

# Windows
aide-program\aide.bat <command> [args]
```

### 6.2 通过 Python 模块

```bash
# 需要先激活虚拟环境或设置 PYTHONPATH
python -m aide <command> [args]
```

### 6.3 依赖要求

- Python >= 3.11
- uv（用于虚拟环境和依赖管理）
- tomli-w（TOML 写入）

---

## 七、开发指南

### 7.1 添加新子命令

1. 在 `docs/commands/` 创建设计文档
2. 在 `aide/` 下创建对应模块目录
3. 在 `aide/main.py` 添加 CLI 路由
4. 更新本导览的子命令索引
5. 更新 [aide skill 设计文档](../../aide-marketplace/aide-plugin/docs/skill/aide.md)

### 7.2 修改现有子命令

1. 阅读对应的设计文档
2. 修改代码实现
3. 更新设计文档（如有接口变更）
4. 同步更新 aide skill 文档

### 7.3 代码规范

- 所有输出使用 `core/output.py` 中的函数
- 配置操作使用 `core/config.py` 中的 `ConfigManager`
- 遵循静默原则：成功时尽量少输出

---

## 八、相关文档

- [总导览](../../docs/aide-overview.md)
- [aide-plugin 导览](../../aide-marketplace/aide-plugin/docs/README.md)
- [aide skill 设计文档](../../aide-marketplace/aide-plugin/docs/skill/aide.md)
