# aide env 子命令设计文档

## 一、背景

### 1.1 解决的问题

| 问题 | 影响 |
|------|------|
| 环境不一致 | 命令执行失败，打断业务流程 |
| 手动检查繁琐 | 每次都要检查 Python、虚拟环境、依赖 |
| 修复方式不统一 | 不同人有不同的修复习惯 |
| 检测项不可扩展 | 无法按需添加新的环境检测 |

### 1.2 设计目标

提供**模块化、可配置的环境检测与修复**：
- 模块化检测项，支持扩展
- 可配置启用哪些模块
- 能修复的自动修复
- 不能修复的给出明确建议
- 详细模式供人工确认

---

## 二、命令结构

```
aide env                         # 等同于 aide env ensure
aide env ensure [options]        # 检测并修复
aide env list                    # 列出所有可用模块
aide env set <key> <value>       # 设置环境配置（带验证）
```

### 2.1 aide env ensure

检测环境并尝试修复问题。

**参数：**

| 参数 | 说明 |
|------|------|
| `--runtime` | 仅检测 aide 运行时环境（python + uv） |
| `--modules M1,M2` | 指定要检测的模块（逗号分隔） |
| `--all` | 检测所有已启用模块，仅检查不修复 |
| `-v, --verbose` | 显示详细配置信息 |

### 2.2 aide env list

列出所有可用的环境检测模块及其状态。

### 2.3 aide env set

设置环境配置，带模块名称验证。

**用法：**

```bash
aide env set modules <模块列表>      # 设置启用的模块（逗号分隔）
aide env set <模块名>.<配置项> <值>  # 设置模块配置
```

**示例：**

```bash
# 设置启用的模块
aide env set modules python,uv,rust,node

# 设置模块配置
aide env set venv.path .venv
aide env set requirements.path requirements.txt

# 设置实例化模块（多项目场景）
aide env set modules rust,node,flutter,android,node_deps:react
aide env set node_deps:react.path react-demo
```

**验证规则：**

- 设置 `modules` 时，验证每个模块类型是否存在
- 设置模块配置时，验证模块类型是否存在
- 无效模块名会报错并显示可用模块列表

**错误示例：**

```
✗ 未知模块: fortran, cobol
→ 可用模块: python, uv, venv, requirements, rust, node, flutter, node_deps, android
```

---

## 三、模块系统

### 3.1 模块分类

**类型A：自包含模块（无需配置即可检测）**

| 模块 | 描述 | 能力 |
|------|------|------|
| `python` | Python 解释器版本 | check |
| `uv` | uv 包管理器 | check |
| `rust` | Rust 工具链（rustc + cargo） | check |
| `node` | Node.js 运行时 | check |
| `flutter` | Flutter SDK | check |
| `android` | Android SDK | check |

**类型B：路径依赖模块（必须有配置才能检测）**

| 模块 | 描述 | 能力 | 必需配置 |
|------|------|------|----------|
| `venv` | Python 虚拟环境 | check, ensure | `path` |
| `requirements` | Python 依赖管理 | check, ensure | `path` |
| `node_deps` | Node.js 项目依赖 | check, ensure | `path` |

### 3.2 模块能力

- `check`：检测环境是否可用
- `ensure`：检测失败时尝试自动修复

### 3.3 模块实例化命名

支持 `模块类型:实例名` 格式，用于同类型多实例场景：

```toml
# 多个 Node.js 项目
modules = ["node", "node_deps:react", "node_deps:vue"]

[env."node_deps:react"]
path = "react-demo"

[env."node_deps:vue"]
path = "vue-demo"
manager = "pnpm"
```

**输出示例：**
```
✓ node: 24.11.1 (npm 11.6.2)
✓ node_deps:react: react-demo (npm)
✓ node_deps:vue: vue-demo (pnpm)
```

---

## 四、配置

### 4.1 配置结构

```toml
[env]
# 启用的模块列表
modules = ["python", "uv", "venv", "requirements"]

# 类型A模块配置（可选）
[env.python]
min_version = "3.11"

# 类型B模块配置（必需）
[env.venv]
path = ".venv"

[env.requirements]
path = "requirements.txt"
```

### 4.2 多项目配置示例

```toml
[env]
modules = ["rust", "node", "flutter", "android", "node_deps:react"]

# Node.js 项目依赖（实例化命名）
[env."node_deps:react"]
path = "react-demo"
# manager = "npm"  # 可选，默认自动检测
```

### 4.3 node_deps 模块配置

| 配置项 | 必需 | 说明 |
|--------|------|------|
| `path` | 是 | package.json 所在目录 |
| `manager` | 否 | 包管理器：npm/pnpm/yarn/bun，默认自动检测 |

**自动检测逻辑**（按锁文件判断）：
- `pnpm-lock.yaml` → pnpm
- `yarn.lock` → yarn
- `bun.lockb` → bun
- `package-lock.json` 或无锁文件 → npm

### 4.4 配置兼容性

支持旧格式配置：

```toml
[env]
venv = ".venv"
requirements = "requirements.txt"
```

读取时自动转换为新格式。

---

## 五、执行逻辑

### 5.1 输出级别规则

| 场景 | 在启用列表 | 有配置 | 结果 | 输出 | 行为 |
|------|-----------|--------|------|------|------|
| ensure | ✓ | ✓/NA | 成功 | ✓ | 继续 |
| ensure | ✓ | ✓/NA | 失败+可修复 | → | 修复 |
| ensure | ✓ | ✓/NA | 失败+不可修复 | ✗ | **停止** |
| ensure | ✓ | ✗(B类) | - | ✗ | **停止** |
| --modules | ✗ | ✓/NA | 成功 | ✓ | 继续 |
| --modules | ✗ | ✓/NA | 失败 | ⚠ | 继续 |
| --modules | ✗ | ✗(B类) | - | ⚠ | 跳过 |
| --all | any | any | any | ✓/⚠ | 仅检测 |

**核心原则：**
- 启用模块失败 = 错误(✗) = 必须解决
- 未启用模块失败 = 警告(⚠) = 可忽略
- 启用的B类模块无配置 = 错误(✗) = 配置错误

---

## 六、输出示例

### 6.1 aide env list

```
可用模块:
  模块           描述                   能力               需要配置
  ────────────────────────────────────────────────────────────
  python        Python 解释器版本       check            否
  uv            uv 包管理器            check            否
  venv          Python 虚拟环境        check, ensure    是 [path]
  requirements  Python 依赖管理        check, ensure    是 [path]
  rust          Rust 工具链           check            否
  node          Node.js 运行时        check            否
  flutter       Flutter SDK          check            否
  node_deps     Node.js 项目依赖      check, ensure    是 [path]
  android       Android SDK          check            否

当前启用: rust, node, flutter, android, node_deps:react
```

### 6.2 aide env ensure（多项目场景）

```
✓ rust: 1.94.0-nightly (cargo 1.94.0-nightly)
✓ node: 24.11.1 (npm 11.6.2)
✓ flutter: 3.38.4 (dart 3.10.3)
✓ android: /home/user/android-sdk (adb 1.0.41, build-tools 36.1.0, API 36)
→ node_deps:react: node_modules 不存在，尝试修复...
✓ node_deps:react: 已安装 (npm)
✓ 环境就绪 (rust:1.94.0-nightly, node:24.11.1, flutter:3.38.4, android:/home/user/android-sdk, node_deps:react:react-demo)
```

### 6.3 aide env set 示例

```bash
# 设置多种环境模块
$ aide env set modules rust,node,flutter,android,node_deps:react
✓ 已更新 env.modules = ['rust', 'node', 'flutter', 'android', 'node_deps:react']

# 设置实例化模块配置
$ aide env set node_deps:react.path react-demo
✓ 已更新 env."node_deps:react".path = 'react-demo'

# 验证失败示例
$ aide env set modules python,fortran
✗ 未知模块: fortran
→ 可用模块: python, uv, venv, requirements, rust, node, flutter, node_deps, android
```

---

## 七、代码结构

```
aide/env/
├── __init__.py
├── manager.py              # 环境管理器主入口
├── registry.py             # 模块注册表
└── modules/
    ├── __init__.py
    ├── base.py             # 模块基类
    ├── python.py           # Python 模块
    ├── uv.py               # uv 模块
    ├── venv.py             # venv 模块
    ├── requirements.py     # requirements 模块
    ├── rust.py             # Rust 模块
    ├── node.py             # Node.js 模块
    ├── flutter.py          # Flutter 模块
    ├── node_deps.py        # Node.js 项目依赖模块
    └── android.py          # Android SDK 模块
```

### 7.1 模块基类

```python
class BaseModule(ABC):
    @property
    @abstractmethod
    def info(self) -> ModuleInfo: ...

    @abstractmethod
    def check(self, config: dict, root: Path) -> CheckResult: ...

    def ensure(self, config: dict, root: Path) -> CheckResult: ...

    def validate_config(self, config: dict) -> tuple[bool, str | None]: ...
```

### 7.2 添加新模块

1. 在 `aide/env/modules/` 创建模块文件
2. 继承 `BaseModule` 实现 `info` 和 `check` 方法
3. 如支持修复，实现 `ensure` 方法
4. 导出 `module` 实例
5. 在 `registry.py` 的 `register_builtin_modules()` 中注册

---

## 八、相关文档

- [program 导览](../README.md)
- [配置格式文档](../formats/config.md)
- [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)
