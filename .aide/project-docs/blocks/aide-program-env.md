# aide-program-env

> 路径：aide-program/aide/env/
> 最后更新：2025-12-17

## 概述

环境检测和管理模块，负责检测和修复开发环境。采用插件式架构，支持多种开发环境的检测。模块分为两类：类型A（全局工具，无需配置）和类型B（项目级，需要配置路径）。

## 目录结构

```
aide-program/aide/env/
├── __init__.py              模块初始化
├── manager.py               环境管理器
├── registry.py              模块注册表
└── modules/
    ├── __init__.py          模块集合初始化
    ├── base.py              模块基类定义
    ├── python.py            Python 解释器检测
    ├── uv.py                uv 包管理器检测
    ├── venv.py              虚拟环境管理
    ├── requirements.py      Python 依赖管理
    ├── rust.py              Rust 工具链检测
    ├── node.py              Node.js 运行时检测
    ├── node_deps.py         Node.js 项目依赖
    ├── flutter.py           Flutter SDK 检测
    └── android.py           Android SDK 检测
```

## 文件清单

| 文件 | 类型 | 说明 |
|------|------|------|
| __init__.py | 源码 | 模块初始化 |
| manager.py | 源码 | EnvManager 类，环境检测核心逻辑 |
| registry.py | 源码 | ModuleRegistry 注册表，管理所有检测模块 |
| modules/base.py | 源码 | BaseModule 基类和数据类定义 |
| modules/python.py | 源码 | Python 版本检测（类型A） |
| modules/uv.py | 源码 | uv 包管理器检测（类型A） |
| modules/venv.py | 源码 | 虚拟环境检测/创建（类型B） |
| modules/requirements.py | 源码 | Python 依赖检测/安装（类型B） |
| modules/rust.py | 源码 | Rust 工具链检测（类型A） |
| modules/node.py | 源码 | Node.js 运行时检测（类型A） |
| modules/node_deps.py | 源码 | Node.js 依赖检测/安装（类型B） |
| modules/flutter.py | 源码 | Flutter SDK 检测（类型A） |
| modules/android.py | 源码 | Android SDK 检测（类型A） |

## 核心组件

### EnvManager 类

- **职责**：环境管理器，协调各模块的检测和修复
- **位置**：`aide/env/manager.py:53`
- **关键方法**：
  - `list_modules()` - 列出所有可用模块（aide env list）
  - `ensure()` - 检测并修复环境（aide env ensure）
  - `set_modules()` - 设置启用的模块列表
  - `set_module_config()` - 设置模块配置
  - `_process_module()` - 处理单个模块的检测/修复

### ModuleRegistry 类

- **职责**：模块注册表，管理所有可用的环境检测模块
- **位置**：`aide/env/registry.py:8`
- **关键方法**：
  - `register(module)` - 注册模块
  - `get(name)` - 获取指定模块
  - `names()` - 获取所有模块名称
  - `list_info()` - 获取所有模块的元信息

### BaseModule 抽象类

- **职责**：模块基类，定义检测模块的接口
- **位置**：`aide/env/modules/base.py:37`
- **抽象方法**：
  - `info` - 返回模块元信息（ModuleInfo）
  - `check(config, root)` - 检测环境
  - `ensure(config, root)` - 修复环境（可选）
  - `validate_config(config)` - 验证模块配置

### CheckResult 数据类

- **职责**：检测结果封装
- **位置**：`aide/env/modules/base.py:12`
- **字段**：
  - `success: bool` - 是否成功
  - `version: str | None` - 版本信息
  - `message: str | None` - 消息
  - `can_ensure: bool` - 是否可修复

### ModuleInfo 数据类

- **职责**：模块元信息
- **位置**：`aide/env/modules/base.py:22`
- **字段**：
  - `name: str` - 模块名称
  - `description: str` - 描述
  - `capabilities: list[str]` - 能力（check, ensure）
  - `requires_config: bool` - 是否需要配置
  - `config_keys: list[str]` - 需要的配置键

## 模块列表

| 模块 | 类型 | 能力 | 说明 |
|------|------|------|------|
| python | A | check | Python 解释器版本检测 |
| uv | A | check | uv 包管理器检测 |
| venv | B | check, ensure | 虚拟环境检测/创建 |
| requirements | B | check, ensure | Python 依赖检测/安装 |
| rust | A | check | Rust 工具链（rustc + cargo） |
| node | A | check | Node.js 运行时 |
| node_deps | B | check, ensure | Node.js 项目依赖 |
| flutter | A | check | Flutter SDK |
| android | A | check | Android SDK |

## 接口说明

### 环境检测 API

```python
from aide.env.manager import EnvManager
from aide.core.config import ConfigManager

cfg = ConfigManager(Path.cwd())
manager = EnvManager(Path.cwd(), cfg)

# 检测运行时环境
manager.ensure(runtime_only=True)

# 检测所有启用模块
manager.ensure()

# 检测指定模块
manager.ensure(modules=["python", "node"])

# 列出可用模块
manager.list_modules()
```

### 模块实例化命名

支持多项目场景，如 `node_deps:frontend`、`node_deps:backend`：

```python
# 配置格式
# [env."node_deps:frontend"]
# path = "frontend"
# manager = "pnpm"
```

## 依赖关系

- 依赖：aide/core（ConfigManager、output）
- 被依赖：aide/main.py

## 注意事项

- 类型A模块无需配置即可检测（全局工具）
- 类型B模块需要在配置中指定路径
- requirements 模块会自动注入 venv 路径
- node_deps 模块支持自动检测包管理器（npm/yarn/pnpm/bun）
- 模块名支持实例化命名（module_type:instance_name）
