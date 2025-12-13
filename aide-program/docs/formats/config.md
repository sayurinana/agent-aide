# 配置文件格式规范

## 一、概述

aide 使用 TOML 格式的配置文件，位于 `.aide/config.toml`。

配置文件采用**自文档化**设计，包含详细注释说明各字段用途。

---

## 二、文件位置

```
.aide/
└── config.toml
```

---

## 三、完整配置结构

```toml
# Aide 默认配置（由 aide init 生成）

# runtime: aide 自身运行要求
[runtime]
python_min = "3.11"      # Python 最低版本要求
use_uv = true            # 是否使用 uv 管理依赖

# task: 任务文档路径
[task]
source = "task-now.md"   # 任务原文档默认路径
spec = "task-spec.md"    # 任务细则文档默认路径

# env: 环境模块配置
[env]
# 启用的模块列表
modules = ["python", "uv", "venv", "requirements"]

# Python 版本要求（可选，默认使用 runtime.python_min）
# [env.python]
# min_version = "3.11"

# 虚拟环境配置（类型B模块，必须配置）
[env.venv]
path = ".venv"

# 依赖文件配置（类型B模块，必须配置）
[env.requirements]
path = "requirements.txt"

# flow: 流程配置
[flow]
phases = ["task-optimize", "flow-design", "impl", "verify", "docs", "finish"]
```

---

## 四、字段详解

### 4.1 [runtime] 运行时配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `python_min` | string | `"3.11"` | Python 最低版本要求 |
| `use_uv` | bool | `true` | 是否使用 uv 管理虚拟环境和依赖 |

**使用场景**：
- `aide env ensure --runtime` 使用硬编码的 `"3.11"`
- `aide env ensure` 读取 `python_min` 进行检查

### 4.2 [task] 任务文档配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `source` | string | `"task-now.md"` | 任务原文档默认路径 |
| `spec` | string | `"task-spec.md"` | 任务细则文档默认路径 |

**使用场景**：
- `/aide:prep` 未传参数时，使用 `source` 作为默认路径
- `/aide:exec` 未传参数时，使用 `spec` 作为默认路径

### 4.3 [env] 环境配置

#### 4.3.1 模块列表

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `modules` | array | `["python", "uv", "venv", "requirements"]` | 启用的环境检测模块 |

**可用模块**：
- `python` - Python 解释器版本检测
- `uv` - uv 包管理器检测
- `venv` - Python 虚拟环境管理
- `requirements` - Python 依赖管理

#### 4.3.2 模块配置

**类型A模块（可选配置）**：

```toml
[env.python]
min_version = "3.11"    # Python 最低版本，默认使用 runtime.python_min
```

**类型B模块（必须配置）**：

```toml
[env.venv]
path = ".venv"          # 虚拟环境目录路径

[env.requirements]
path = "requirements.txt"  # 依赖文件路径
```

**使用场景**：
- `aide env ensure` 按 `modules` 列表检测环境
- `aide env list` 显示所有可用模块及启用状态
- `aide env ensure --modules X,Y` 检测指定模块

### 4.4 [flow] 流程配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `phases` | array | `["task-optimize", "flow-design", "impl", "verify", "docs", "finish"]` | 环节名称列表 |

**使用场景**：
- `aide flow` 校验环节跳转合法性
- 定义有效的环节名称

---

## 五、配置读写接口

### 5.1 读取配置

```bash
aide config get <key>
```

**示例**：
```bash
aide config get task.source
# 输出: → task.source = 'task-now.md'

aide config get env.modules
# 输出: → env.modules = ['python', 'uv', 'venv', 'requirements']

aide config get env.venv.path
# 输出: → env.venv.path = '.venv'

aide config get runtime.python_min
# 输出: → runtime.python_min = '3.11'
```

### 5.2 设置配置

```bash
aide config set <key> <value>
```

**示例**：
```bash
aide config set task.source "my-task.md"
# 输出: ✓ 已更新 task.source = 'my-task.md'

aide config set env.venv.path ".venv-dev"
# 输出: ✓ 已更新 env.venv.path = '.venv-dev'
```

**值类型自动解析**：
- `true` / `false` → bool
- 纯数字 → int
- 带小数点的数字 → float
- 其他 → string

---

## 六、配置访问规则

### 6.1 LLM 不直接读取配置文件

**原则**：LLM 不允许直接读取 `.aide/config.toml` 文件内容，避免污染上下文。

**正确做法**：通过 `aide config get <key>` 读取需要的配置值。

### 6.2 配置缺失处理

- 配置文件不存在时，`aide config get` 输出警告并返回空
- 配置项不存在时，`aide config get` 输出警告
- 建议先执行 `aide init` 确保配置文件存在

### 6.3 模块配置规则

- 类型A模块（python, uv）：配置可选，有默认行为
- 类型B模块（venv, requirements）：如果在 `modules` 列表中启用，必须有对应配置
- 启用的B类模块无配置时，`aide env ensure` 会报错

---

## 七、配置兼容性

### 7.1 旧格式支持

aide 兼容旧版配置格式：

```toml
[env]
venv = ".venv"
requirements = "requirements.txt"
```

读取时自动转换为新格式：

```toml
[env.venv]
path = ".venv"

[env.requirements]
path = "requirements.txt"
```

### 7.2 默认模块列表

如果配置中没有 `env.modules` 字段，使用默认值：

```toml
modules = ["python", "uv", "venv", "requirements"]
```

---

## 八、扩展配置

### 8.1 添加新配置项

1. 在本文档添加字段说明
2. 更新 `ConfigManager` 中的 `DEFAULT_CONFIG`
3. 在相关代码中读取新配置
4. 更新相关设计文档

### 8.2 添加新环境模块

1. 在 `aide/env/modules/` 创建模块文件
2. 在 `registry.py` 注册模块
3. 更新本文档的模块列表
4. 更新 `aide env` 设计文档

### 8.3 配置项命名规范

- 使用小写字母和下划线
- 使用点号分隔层级：`section.key`
- 保持语义清晰

---

## 九、相关文档

- [program 导览](../README.md)
- [aide init 设计](../commands/init.md)
- [aide env 设计](../commands/env.md)
- [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)
