# aide-program-core

> 路径：aide-program/aide/core/
> 最后更新：2025-12-15

## 概述

核心模块，提供配置管理和输出格式化功能，是所有其他模块的基础依赖。

## 文件清单

| 文件 | 说明 |
|------|------|
| `__init__.py` | 模块初始化，导出 output |
| `config.py` | 配置管理器（~390 行） |
| `output.py` | 统一输出格式（25 行） |

## 核心组件

### ConfigManager

- **职责**：管理 `.aide/config.toml` 配置文件
- **位置**：`config.py:240`
- **关键方法**：
  - `ensure_config()` - 确保配置文件存在，不存在则创建默认配置
  - `load_config()` - 加载配置（返回 dict）
  - `get_value(key)` - 获取配置值（支持点号分隔的键）
  - `set_value(key, value)` - 设置配置值（保留注释和格式）
  - `ensure_gitignore()` - 根据配置决定是否忽略 .aide 目录

### DEFAULT_CONFIG

- **职责**：默认配置模板（自文档化，含完整注释）
- **位置**：`config.py:13`
- **特点**：
  - 包含所有配置节的详细说明
  - 用户可仅通过此文件了解所有支持的功能
  - 约 230 行注释文档

### output 模块

- **职责**：统一输出格式
- **位置**：`output.py`
- **输出函数**：
  - `ok(msg)` → `✓ {msg}` 成功
  - `warn(msg)` → `⚠ {msg}` 警告
  - `err(msg)` → `✗ {msg}` 错误
  - `info(msg)` → `→ {msg}` 信息
  - `step(msg, current, total)` → `[n/m] {msg}` 步骤

## 接口说明

被所有其他模块依赖：
```python
from aide.core import output
from aide.core.config import ConfigManager
```

## 依赖关系

- 依赖：标准库 + tomllib + tomli_w
- 被依赖：env, flow, decide, main

## 注意事项

- `set_value()` 使用正则表达式保守更新配置，保留注释
- 配置键不存在时会回退到完全重写（丢失注释）
