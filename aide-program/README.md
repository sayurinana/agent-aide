# Aide 程序系统开发文档

## 一、项目概述

### 1.1 项目定位

Aide 是一套命令行工具集，用于支持 AI 辅助开发工作流。本项目实现 aide 程序的核心基础功能，包括：

- 项目初始化（aide init）
- 环境管理（aide env）
- 配置管理（aide config）

**注意**：本阶段不包含 `aide flow`（进度追踪）和 `aide decide`（待定项确认）的实现，这两个功能将在后续阶段开发。

### 1.2 技术栈要求

- **Python 3.10+**：主要编程语言
- **Shell 脚本**：跨平台入口封装（aide.sh / aide.bat）
- **TOML**：配置文件格式
- **标准库优先**：尽量使用 Python 标准库，减少外部依赖

### 1.3 设计原则

1. **确定性**：相同输入产生相同输出，避免不确定性
2. **精简输出**：成功时输出极简，失败时输出详细
3. **幂等性**：重复执行不产生副作用
4. **自文档化**：配置文件包含详细注释
5. **跨平台**：支持 Linux、macOS、Windows

---

## 二、目录结构

```
aide-program/
├── README.md                    # 本文件：项目总览
├── docs/                        # 详细设计文档
│   ├── 01-入口脚本设计.md       # aide.sh / aide.bat 设计
│   ├── 02-aide-init设计.md      # aide init 命令设计
│   ├── 03-aide-env设计.md       # aide env 命令设计
│   ├── 04-aide-config设计.md    # aide config 命令设计
│   ├── 05-配置文件规范.md       # config.toml 格式规范
│   ├── 06-输出格式规范.md       # 统一输出格式规范
│   └── 07-测试规范.md           # 测试要求和用例
├── src/                         # 源代码目录（开发时创建）
│   ├── aide.sh                  # Linux/macOS 入口
│   ├── aide.bat                 # Windows 入口
│   ├── main.py                  # Python 主入口
│   ├── core/                    # 核心模块
│   │   ├── __init__.py
│   │   ├── config.py            # 配置读写
│   │   └── output.py            # 输出格式化
│   ├── commands/                # 命令实现
│   │   ├── __init__.py
│   │   ├── init.py              # aide init
│   │   ├── env.py               # aide env
│   │   └── config_cmd.py        # aide config
│   └── utils/                   # 工具函数
│       ├── __init__.py
│       └── validators.py        # 验证函数
└── tests/                       # 测试目录（开发时创建）
    ├── test_init.py
    ├── test_env.py
    └── test_config.py
```

---

## 三、命令清单

### 3.1 本阶段实现的命令

| 命令 | 功能 | 优先级 |
|------|------|--------|
| `aide init` | 初始化 .aide 目录和配置文件 | P0 |
| `aide env ensure` | 检测并修复项目开发环境 | P0 |
| `aide env ensure --runtime` | 检测 aide 运行时环境 | P0 |
| `aide config get <key>` | 获取配置值 | P1 |
| `aide config set <key> <value>` | 设置配置值 | P1 |

### 3.2 后续阶段实现的命令

| 命令 | 功能 | 说明 |
|------|------|------|
| `aide flow ...` | 进度追踪和 git 集成 | 后续实现 |
| `aide decide ...` | 待定项确认 Web 服务 | 后续实现 |

---

## 四、开发流程

### 4.1 阅读顺序

建议按以下顺序阅读文档：

1. **README.md**（本文件）- 了解项目全貌
2. **docs/06-输出格式规范.md** - 理解统一输出格式
3. **docs/05-配置文件规范.md** - 理解配置文件结构
4. **docs/01-入口脚本设计.md** - 理解命令调用流程
5. **docs/02-aide-init设计.md** - 实现 aide init
6. **docs/03-aide-env设计.md** - 实现 aide env
7. **docs/04-aide-config设计.md** - 实现 aide config
8. **docs/07-测试规范.md** - 编写测试用例

### 4.2 开发步骤

1. **搭建基础框架**
   - 创建目录结构
   - 实现入口脚本（aide.sh / aide.bat）
   - 实现 main.py 命令分发
   - 实现 core/output.py 输出格式化

2. **实现核心模块**
   - 实现 core/config.py 配置读写
   - 实现 utils/validators.py 验证函数

3. **实现命令**
   - 实现 aide init
   - 实现 aide env ensure --runtime
   - 实现 aide env ensure
   - 实现 aide config get/set

4. **编写测试**
   - 单元测试
   - 集成测试
   - 跨平台测试

5. **文档和打包**
   - 编写用户文档
   - 准备分发包

### 4.3 质量要求

1. **代码质量**
   - 遵循 PEP 8 代码规范
   - 函数和类必须有文档字符串
   - 关键逻辑必须有注释

2. **测试覆盖**
   - 核心功能测试覆盖率 ≥ 80%
   - 所有错误路径必须有测试用例
   - 跨平台兼容性测试

3. **用户体验**
   - 输出信息清晰易懂
   - 错误提示包含解决建议
   - 命令执行速度快（< 1秒）

---

## 五、核心概念

### 5.1 输出格式

所有 aide 命令遵循统一的输出格式：

| 前缀 | 含义 | 使用场景 |
|------|------|---------|
| `✓` | 成功 | 操作成功完成 |
| `⚠` | 警告 | 有问题但可继续 |
| `✗` | 错误 | 操作失败 |
| `→` | 信息 | 进行中或提示信息 |

**静默原则**：无输出 = 正常完成（适用于幂等操作）

### 5.2 配置文件

- **位置**：`.aide/config.toml`
- **格式**：TOML
- **特点**：自文档化，包含详细注释
- **访问**：通过 `aide config` 命令或 core/config.py 模块

### 5.3 数据存储

所有 aide 数据统一存放在项目根目录的 `.aide/` 下：

```
.aide/
├── config.toml          # 项目配置
├── flow-status.json     # 任务进度（后续实现）
├── decisions/           # 待定项记录（后续实现）
└── logs/                # 操作日志（可选）
```

### 5.4 错误处理

1. **预期错误**：返回明确的错误信息和建议
2. **非预期错误**：记录详细日志，返回简化错误信息
3. **退出码**：
   - 0：成功
   - 1：一般错误
   - 2：参数错误
   - 3：环境错误

---

## 六、依赖管理

### 6.1 Python 依赖

**核心依赖**（必需）：
- Python 3.10+
- 标准库：os, sys, pathlib, subprocess, json, configparser

**可选依赖**：
- tomli / tomllib（Python 3.11+ 内置）：TOML 解析
- tomli-w：TOML 写入

### 6.2 系统依赖

**必需**：
- Python 3.10+

**可选**（用于项目环境检测）：
- git
- uv / pip

---

## 七、交付物

### 7.1 必需交付物

1. **源代码**
   - 完整的 Python 代码
   - 入口脚本（aide.sh / aide.bat）
   - 所有必需的模块和工具函数

2. **测试代码**
   - 单元测试
   - 集成测试
   - 测试数据和 fixtures

3. **文档**
   - 用户使用文档
   - 开发者文档（如有特殊设计）
   - CHANGELOG

### 7.2 可选交付物

1. **打包脚本**
   - 用于生成分发包的脚本
   - 安装说明

2. **CI/CD 配置**
   - GitHub Actions 或其他 CI 配置
   - 自动化测试流程

---

## 八、注意事项

### 8.1 设计约束

1. **不要实现 aide flow**：进度追踪功能后续实现
2. **不要实现 aide decide**：待定项确认功能后续实现
3. **不要硬编码路径**：所有路径通过配置或参数传入
4. **不要假设环境**：所有环境依赖必须检测和验证

### 8.2 兼容性要求

1. **Python 版本**：支持 3.10+
2. **操作系统**：Linux、macOS、Windows
3. **路径分隔符**：使用 pathlib 处理跨平台路径
4. **编码**：统一使用 UTF-8

### 8.3 安全考虑

1. **配置文件权限**：不存储敏感信息
2. **命令注入**：所有外部命令调用必须参数化
3. **路径遍历**：验证所有文件路径
4. **输入验证**：验证所有用户输入

---

## 九、参考资料

### 9.1 项目文档

- `../aide-requirements.md`：Aide 系统需求规格
- `../aide-marketplace/aide-plugin/`：Commands 和 Skills 定义
- `../discuss/`：设计讨论和决策记录

### 9.2 外部资源

- [TOML 规范](https://toml.io/)
- [PEP 8 代码规范](https://pep8.org/)
- [Python pathlib 文档](https://docs.python.org/3/library/pathlib.html)

---

## 十、联系方式

如有疑问或需要澄清，请：

1. 查阅 `docs/` 目录下的详细设计文档
2. 参考 `../aide-requirements.md` 了解整体设计
3. 查看 `../discuss/` 目录了解设计决策

---

## 附录：快速开始

### A.1 验证环境

```bash
# 检查 Python 版本
python3 --version  # 应该 >= 3.10

# 检查必要的库
python3 -c "import tomllib"  # Python 3.11+
# 或
python3 -c "import tomli"    # Python 3.10
```

### A.2 创建开发环境

```bash
# 创建虚拟环境
cd aide-program
python3 -m venv .venv

# 激活虚拟环境
source .venv/bin/activate  # Linux/macOS
# 或
.venv\Scripts\activate     # Windows

# 安装依赖（如需要）
pip install tomli tomli-w
```

### A.3 运行测试

```bash
# 运行所有测试
python3 -m pytest tests/

# 运行特定测试
python3 -m pytest tests/test_init.py
```

---

**版本**：v1.0
**更新日期**：2025-12-13
**状态**：待开发
