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

---

## 三、模块系统

### 3.1 模块分类

**类型A：自包含模块（无需配置即可检测）**

| 模块 | 描述 | 能力 |
|------|------|------|
| `python` | Python 解释器版本 | check |
| `uv` | uv 包管理器 | check |

**类型B：路径依赖模块（必须有配置才能检测）**

| 模块 | 描述 | 能力 | 必需配置 |
|------|------|------|----------|
| `venv` | Python 虚拟环境 | check, ensure | `path` |
| `requirements` | Python 依赖管理 | check, ensure | `path` |

### 3.2 模块能力

- `check`：检测环境是否可用
- `ensure`：检测失败时尝试自动修复

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

### 4.2 配置兼容性

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

### 5.2 业务流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:读取配置;
:获取启用模块列表;

if (--runtime?) then (是)
  :target = [python, uv];
else if (--modules?) then (是)
  :target = 指定模块;
else if (--all?) then (是)
  :target = 启用模块;
  :check_only = true;
else (否)
  :target = 启用模块;
endif

if (verbose?) then (是)
  :输出详细头部信息;
endif

:遍历 target 模块;

repeat
  :获取模块实例;
  :获取模块配置;

  if (verbose?) then (是)
    :输出模块配置详情;
  endif

  if (B类模块 && 无配置?) then (是)
    if (在启用列表?) then (是)
      :输出错误;
      stop
    else (否)
      :输出警告，跳过;
    endif
  endif

  :执行 check();

  if (成功?) then (是)
    :输出成功;
  else (否)
    if (check_only?) then (是)
      :输出警告;
    else if (可修复?) then (是)
      :执行 ensure();
      if (修复成功?) then (是)
        :输出成功;
      else (否)
        if (在启用列表?) then (是)
          :输出错误;
          stop
        else (否)
          :输出警告;
        endif
      endif
    else (否)
      if (在启用列表?) then (是)
        :输出错误;
        stop
      else (否)
        :输出警告;
      endif
    endif
  endif

repeat while (还有模块?)

:输出环境就绪;

stop
@enduml
```

---

## 六、输出示例

### 6.1 aide env list

```
可用模块:
  模块          描述                    能力               需要配置
  ────────────────────────────────────────────────────────────
  python       Python 解释器版本         check            否
  uv           uv 包管理器              check            否
  venv         Python 虚拟环境          check, ensure    是 [path]
  requirements Python 依赖管理          check, ensure    是 [path]

当前启用: python, uv, venv, requirements
```

### 6.2 aide env ensure

**成功：**
```
✓ python: 3.14.2 (>=3.11)
✓ uv: uv 0.9.16
✓ venv: .venv
✓ requirements: requirements.txt
✓ 环境就绪 (python:3.14.2, uv:uv 0.9.16, venv:.venv, requirements:requirements.txt)
```

**需修复：**
```
✓ python: 3.14.2 (>=3.11)
✓ uv: uv 0.9.16
→ venv: 虚拟环境不存在: .venv，尝试修复...
✓ venv: 已创建
✓ requirements: requirements.txt
✓ 环境就绪 (...)
```

**启用模块失败：**
```
✓ python: 3.14.2 (>=3.11)
✓ uv: uv 0.9.16
✗ venv: 已启用但缺少配置项: path
```

### 6.3 aide env ensure --verbose

```
============================================================
环境检测详细信息
============================================================

  工作目录: /home/user/myproject
  配置文件: /home/user/myproject/.aide/config.toml
  配置存在: 是

  启用模块: python, uv, venv, requirements

  目标模块: python, uv, venv, requirements

  [python] 配置:
    min_version: 3.11
✓ python: 3.14.2 (>=3.11)
  [uv] 配置:
    (无配置)
✓ uv: uv 0.9.16
  [venv] 配置:
    path: .venv
    path (绝对): /home/user/myproject/.venv
    path (存在): 是
✓ venv: .venv
  [requirements] 配置:
    path: requirements.txt
    path (绝对): /home/user/myproject/requirements.txt
    path (存在): 是
✓ requirements: requirements.txt
✓ 环境就绪 (...)
```

### 6.4 aide env ensure --runtime

```
✓ python: 3.14.2 (>=3.11)
✓ uv: uv 0.9.16
✓ 环境就绪 (python:3.14.2, uv:uv 0.9.16)
```

### 6.5 aide env ensure --all

```
✓ python: 3.14.2 (>=3.11)
✓ uv: uv 0.9.16
✓ venv: .venv
✓ requirements: requirements.txt
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
    └── requirements.py     # requirements 模块
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
