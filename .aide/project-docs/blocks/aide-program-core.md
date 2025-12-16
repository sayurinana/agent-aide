# aide-program-core

> 路径：aide-program/aide/core/ 及 aide-program/aide/
> 最后更新：2025-12-17

## 概述

Aide CLI 工具的核心模块，包含配置管理、输出格式化和命令行入口。本区块是整个 aide 程序的基础设施层，被所有其他模块依赖。

## 目录结构

```
aide-program/aide/
├── __init__.py              包初始化
├── __main__.py              模块入口
├── main.py                  命令行主入口
└── core/
    ├── __init__.py          核心模块初始化
    ├── config.py            配置管理
    └── output.py            输出格式工具
```

## 文件清单

| 文件 | 类型 | 说明 |
|------|------|------|
| aide/__init__.py | 源码 | 包初始化，定义包描述 |
| aide/__main__.py | 源码 | 模块入口，支持 `python -m aide` |
| aide/main.py | 源码 | 命令行主入口，解析参数并分发命令 |
| core/__init__.py | 源码 | 核心模块初始化，定义模块描述 |
| core/config.py | 源码 | 配置管理器，处理 TOML 配置 |
| core/output.py | 源码 | 输出格式工具，统一输出前缀 |

## 核心组件

### ConfigManager 类

- **职责**：管理 .aide 目录和 config.toml 配置文件
- **位置**：`aide/core/config.py:240`
- **关键方法**：
  - `ensure_base_dirs()` - 创建 .aide 目录结构
  - `ensure_gitignore()` - 根据配置更新 .gitignore
  - `ensure_config()` - 确保配置文件存在
  - `load_config()` - 加载 TOML 配置
  - `get_value(key)` - 读取配置值（支持点号分隔键）
  - `set_value(key, value)` - 设置配置值（保留注释）
  - `_update_config_value()` - 保守更新配置，保留原文件注释

### DEFAULT_CONFIG

- **职责**：默认配置模板，包含完整注释说明
- **位置**：`aide/core/config.py:13-237`
- **配置节**：
  - `[general]` - 通用配置（gitignore_aide）
  - `[runtime]` - 运行时要求（python_min, use_uv）
  - `[task]` - 任务文档路径（source, spec）
  - `[env]` - 环境检测模块配置
  - `[docs]` - 项目文档配置
  - `[flow]` - 流程追踪配置
  - `[plantuml]` - PlantUML 工具配置
  - `[decide]` - 待定项确认配置

### output 模块

- **职责**：统一输出格式
- **位置**：`aide/core/output.py`
- **函数**：
  - `ok(msg)` - 成功输出（✓ 前缀）
  - `warn(msg)` - 警告输出（⚠ 前缀）
  - `err(msg)` - 错误输出（✗ 前缀）
  - `info(msg)` - 信息输出（→ 前缀）
  - `step(msg, current, total)` - 步骤输出（[n/m] 前缀）

### main() 函数

- **职责**：命令行入口，构建解析器并分发命令
- **位置**：`aide/main.py:16`
- **支持的命令**：
  - `aide init` - 初始化 .aide 目录
  - `aide env {ensure|list|set}` - 环境管理
  - `aide config {get|set}` - 配置管理
  - `aide flow {start|next-step|back-step|next-part|back-part|issue|error|status|list|show}` - 进度追踪
  - `aide decide {submit|result}` - 待定项确认

## 接口说明

### 配置读写 API

```python
from aide.core.config import ConfigManager

cfg = ConfigManager(Path.cwd())
cfg.ensure_config()  # 确保配置存在
value = cfg.get_value("env.venv.path")  # 读取嵌套键
cfg.set_value("task.source", "my-task.md")  # 设置值
```

### 输出格式 API

```python
from aide.core import output

output.ok("操作成功")    # ✓ 操作成功
output.warn("警告信息")  # ⚠ 警告信息
output.err("错误信息")   # ✗ 错误信息
output.info("提示信息")  # → 提示信息
```

## 依赖关系

- 依赖：tomllib（标准库）、tomli_w（第三方）
- 被依赖：aide/env、aide/flow、aide/decide

## 注意事项

- `set_value()` 使用正则替换保留原文件注释格式
- 配置键支持点号分隔的嵌套访问（如 `env.venv.path`）
- 新增配置键时会回退到完全重写模式（丢失注释）
