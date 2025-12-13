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

# env: 虚拟环境与依赖配置
[env]
venv = ".venv"           # 虚拟环境路径
requirements = "requirements.txt"  # 依赖文件路径

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
- `aide env ensure` 输出这两个路径供 LLM 记录

### 4.3 [env] 环境配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `venv` | string | `".venv"` | 虚拟环境目录路径 |
| `requirements` | string | `"requirements.txt"` | 依赖文件路径 |

**使用场景**：
- `aide env ensure` 检查/创建虚拟环境
- `aide env ensure` 安装依赖

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

aide config get flow.phases
# 输出: → flow.phases = ['task-optimize', 'flow-design', 'impl', 'verify', 'docs', 'finish']

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

aide config set runtime.python_min "3.12"
# 输出: ✓ 已更新 runtime.python_min = '3.12'
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

---

## 七、扩展配置

### 7.1 添加新配置项

1. 在本文档添加字段说明
2. 更新 `ConfigManager` 中的 `DEFAULT_CONFIG`
3. 在相关代码中读取新配置
4. 更新相关设计文档

### 7.2 配置项命名规范

- 使用小写字母和下划线
- 使用点号分隔层级：`section.key`
- 保持语义清晰

---

## 八、相关文档

- [program 导览](../README.md)
- [aide init 设计](../commands/init.md)
- [aide env 设计](../commands/env.md)
- [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)
