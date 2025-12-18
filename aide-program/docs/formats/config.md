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
# 本配置文件采用自文档化设计，所有字段均有注释说明

# general: 通用设置
[general]
gitignore_aide = false   # 是否自动将 .aide/ 添加到 .gitignore

# runtime: aide 自身运行要求
[runtime]
python_min = "3.11"      # Python 最低版本要求
use_uv = true            # 是否使用 uv 管理依赖

# task: 任务文档路径
[task]
source = "task-now.md"   # 任务原文档默认路径
spec = "task-spec.md"    # 任务细则文档默认路径
plans_path = ".aide/task-plans/"  # 复杂任务计划文档目录

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

# docs: 项目文档配置
[docs]
path = ".aide/project-docs"  # 项目文档存放路径
block_plan_path = ".aide/project-docs/block-plan.md"  # 区块计划文件路径
steps_path = ".aide/project-docs/steps"  # 步骤文档目录路径

# user_docs: 面向用户的文档配置
[user_docs]
readme_path = "README.md"  # README 文件路径
rules_path = "make-readme-rules.md"  # README 编写规范文件路径
docs_path = "docs"  # 用户文档目录路径
docs_plan_path = "docs/user-docs-plan.md"  # 用户文档计划文件路径
docs_steps_path = "docs/steps"  # 用户文档步骤目录路径
graph_path = "docs/graph-guide"  # 用户流程图目录路径
graph_plan_path = "docs/graph-guide/plan.md"  # 流程图计划文件路径
graph_steps_path = "docs/graph-guide/steps"  # 流程图步骤目录路径

# flow: 流程配置
[flow]
phases = ["task-optimize", "flow-design", "impl", "verify", "docs", "confirm", "finish"]
diagram_path = ".aide/diagrams"  # 流程图存放路径

# plantuml: PlantUML 配置
[plantuml]
jar_path = ""            # plantuml.jar 路径，为空时使用内置 jar
font_name = "Arial"      # 默认字体名称
dpi = 300                # DPI 值
scale = 0.5              # 缩放系数

# decide: 待定项确认服务配置
[decide]
port = 3721
timeout = 0
```

---

## 四、字段详解

### 4.1 [general] 通用设置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `gitignore_aide` | bool | `false` | 是否自动将 .aide/ 添加到 .gitignore |

**使用场景**：
- `aide init` 时检查此配置，决定是否修改 .gitignore
- 默认 `false`，推荐将 .aide/ 纳入版本控制，便于多设备同步
- 设为 `true` 可自动将 .aide/ 添加到 .gitignore

### 4.2 [runtime] 运行时配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `python_min` | string | `"3.11"` | Python 最低版本要求 |
| `use_uv` | bool | `true` | 是否使用 uv 管理虚拟环境和依赖 |

**使用场景**：
- `aide env ensure --runtime` 使用硬编码的 `"3.11"`
- `aide env ensure` 读取 `python_min` 进行检查

### 4.3 [task] 任务文档配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `source` | string | `"task-now.md"` | 任务原文档默认路径 |
| `spec` | string | `"task-spec.md"` | 任务细则文档默认路径 |
| `plans_path` | string | `".aide/task-plans/"` | 复杂任务计划文档目录 |

**使用场景**：
- `/aide:run` 未传参数时，使用 `source` 作为默认路径
- `/aide:run` 续接任务时，使用 `spec` 读取任务细则
- 当任务被拆分为多个子计划时，存放在 `plans_path` 目录下

### 4.4 [env] 环境配置

#### 4.4.1 模块列表

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `modules` | array | `["python", "uv", "venv", "requirements"]` | 启用的环境检测模块 |

**可用模块**：

| 模块 | 类型 | 说明 |
|------|------|------|
| `python` | A | Python 解释器版本检测 |
| `uv` | A | uv 包管理器检测 |
| `rust` | A | Rust 工具链检测（rustc + cargo） |
| `node` | A | Node.js 运行时检测 |
| `flutter` | A | Flutter SDK 检测 |
| `android` | A | Android SDK 检测 |
| `venv` | B | Python 虚拟环境管理 |
| `requirements` | B | Python 依赖管理 |
| `node_deps` | B | Node.js 项目依赖管理 |

**模块实例化命名**：支持 `模块类型:实例名` 格式，用于同类型多实例场景。

#### 4.4.2 模块配置

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

[env.node_deps]
path = "frontend"       # package.json 所在目录
manager = "npm"         # 可选：npm/pnpm/yarn/bun，默认自动检测
```

**实例化模块配置**（多项目场景）：

```toml
[env]
modules = ["node", "node_deps:react", "node_deps:vue"]

[env."node_deps:react"]
path = "react-demo"

[env."node_deps:vue"]
path = "vue-demo"
manager = "pnpm"
```

**使用场景**：
- `aide env ensure` 按 `modules` 列表检测环境
- `aide env list` 显示所有可用模块及启用状态
- `aide env ensure --modules X,Y` 检测指定模块

### 4.5 [docs] 项目文档配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `path` | string | `".aide/project-docs"` | 项目文档存放路径 |
| `block_plan_path` | string | `".aide/project-docs/block-plan.md"` | 区块计划文件路径 |
| `steps_path` | string | `".aide/project-docs/steps"` | 步骤文档目录路径 |

**使用场景**：
- `/aide:docs` 创建和更新项目文档时使用 `path`
- `/aide:load` 载入项目文档时读取 `path`
- `block_plan_path` 记录文档区块划分和生成进度，用于多对话续接
- `steps_path` 存放分步执行的步骤文档，用于接续执行

### 4.6 [user_docs] 面向用户的文档配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `readme_path` | string | `"README.md"` | README 文件路径 |
| `rules_path` | string | `"make-readme-rules.md"` | README 编写规范文件路径 |
| `docs_path` | string | `"docs"` | 用户文档目录路径 |
| `docs_plan_path` | string | `"docs/user-docs-plan.md"` | 用户文档计划文件路径 |
| `docs_steps_path` | string | `"docs/steps"` | 用户文档步骤目录路径 |
| `graph_path` | string | `"docs/graph-guide"` | 用户流程图目录路径 |
| `graph_plan_path` | string | `"docs/graph-guide/plan.md"` | 流程图计划文件路径 |
| `graph_steps_path` | string | `"docs/graph-guide/steps"` | 流程图步骤目录路径 |

**使用场景**：
- `/aide:readme` 生成 README 时使用 `readme_path` 和 `rules_path`
- `/aide:user-docs` 生成用户文档时使用 `docs_path`、`docs_plan_path`、`docs_steps_path`
- `/aide:user-graph` 生成流程图时使用 `graph_path`、`graph_plan_path`、`graph_steps_path`
- `*_plan_path` 用于分步执行和接续执行的计划管理
- `*_steps_path` 存放分步执行的步骤文档

### 4.7 [flow] 流程配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `phases` | array | `["task-optimize", "flow-design", "impl", "verify", "docs", "confirm", "finish"]` | 环节名称列表 |
| `diagram_path` | string | `".aide/diagrams"` | 流程图存放路径 |

**使用场景**：
- `aide flow` 校验环节跳转合法性
- 定义有效的环节名称
- `diagram_path` 存放 PlantUML 源文件（.puml）和生成的图片（.png）

### 4.8 [plantuml] PlantUML 配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `jar_path` | string | `""` | plantuml.jar 路径，为空时使用内置 jar |
| `font_name` | string | `"Arial"` | 默认字体名称 |
| `dpi` | int | `300` | DPI 值（影响图片清晰度） |
| `scale` | float | `0.5` | 缩放系数（0.5 表示缩小到 50%） |

**使用场景**：
- `aide flow next-part` 离开 flow-design 时校验和生成流程图
- 支持自定义 jar 路径以使用特定版本的 PlantUML
- LLM 编写 PlantUML 时应在文件头部添加：
  ```plantuml
  skinparam defaultFontName "<font_name>"
  skinparam dpi <dpi>
  scale <scale>
  ```

### 4.9 [decide] 待定项确认配置

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `port` | int | `3721` | Web 服务起始端口，端口被占用时向后探测最多 10 次 |
| `timeout` | int | `0` | 服务超时时间（秒），0 表示不启用超时 |
| `bind` | string | `"127.0.0.1"` | 服务监听地址，设为 `"0.0.0.0"` 可允许外部访问 |
| `url` | string | `""` | 自定义访问地址，为空时自动生成 `http://localhost:{port}` |

**使用场景**：
- `aide decide submit '<json>'` 读取 `port` 作为起始端口
- `aide decide submit '<json>'` 读取 `timeout` 控制服务最长等待时间
- `aide decide submit '<json>'` 读取 `bind` 作为监听地址
- `aide decide submit '<json>'` 读取 `url` 作为输出的访问地址（支持自定义域名）

**示例配置**：
```toml
[decide]
port = 3721
bind = "0.0.0.0"           # 监听所有网络接口
url = "http://example.dev.net:3721"  # 自定义访问地址
```

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
- [aide flow 设计](../commands/flow.md)
- [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skills/aide/SKILL.md)
