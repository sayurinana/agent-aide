# aide-program-env

> 路径：aide-program/aide/env/
> 最后更新：2025-12-15

## 概述

环境检测模块，提供模块化的开发环境检测和修复功能。支持多种语言和工具，以及模块实例化（多项目场景）。

## 文件清单

| 文件 | 说明 |
|------|------|
| `__init__.py` | 模块初始化 |
| `manager.py` | 环境管理器（~374 行） |
| `registry.py` | 模块注册表（~55 行） |
| `modules/base.py` | 模块基类定义（~90 行） |
| `modules/python.py` | Python 检测模块 |
| `modules/uv.py` | uv 包管理器检测 |
| `modules/venv.py` | Python 虚拟环境检测 |
| `modules/requirements.py` | Python 依赖检测 |
| `modules/rust.py` | Rust 工具链检测 |
| `modules/node.py` | Node.js 检测 |
| `modules/flutter.py` | Flutter SDK 检测 |
| `modules/android.py` | Android SDK 检测 |
| `modules/node_deps.py` | Node.js 项目依赖检测 |

## 核心组件

### EnvManager

- **职责**：环境检测和修复的入口
- **位置**：`manager.py:53`
- **关键方法**：
  - `list_modules()` - 列出所有可用模块
  - `ensure(runtime_only, modules, check_only, verbose)` - 检测并修复环境
  - `set_modules(module_names)` - 设置启用的模块列表
  - `set_module_config(module_name, key, value)` - 设置模块配置

### ModuleRegistry

- **职责**：模块注册表，管理所有可用的环境检测模块
- **位置**：`registry.py:8`
- **关键方法**：
  - `register(module)` - 注册模块
  - `get(name)` - 获取模块
  - `names()` - 获取所有模块名称

### BaseModule（抽象基类）

- **职责**：定义模块接口
- **位置**：`modules/base.py:37`
- **关键方法**：
  - `info` - 返回模块元信息
  - `check(config, root)` - 检测环境
  - `ensure(config, root)` - 修复环境（可选）
  - `validate_config(config)` - 验证配置

### 数据类

- `CheckResult` - 检测/修复结果
- `ModuleInfo` - 模块元信息（名称、描述、能力、配置需求）

## 接口说明

```python
# CLI 入口
aide env list              # 列出可用模块
aide env ensure            # 检测启用的模块
aide env ensure --runtime  # 仅检测运行时
aide env ensure --modules python,node  # 指定模块
aide env ensure --all      # 检测所有（仅报告）
aide env set modules python,uv,venv    # 设置启用模块
aide env set venv.path .venv           # 设置模块配置
```

## 模块分类

| 类型 | 模块 | 需要配置 | 支持修复 |
|------|------|----------|----------|
| A | python, uv, rust, node, flutter, android | 否 | 否 |
| B | venv, requirements, node_deps | 是 | 是 |

## 依赖关系

- 依赖：core（output, config）
- 被依赖：main.py

## 注意事项

- 支持模块实例化命名：`模块类型:实例名`（如 `node_deps:frontend`）
- 类型B模块必须配置路径才能检测
- 启用模块失败时会停止检测
