# aide env 子命令设计文档

## 一、背景

### 1.1 解决的问题

| 问题 | 影响 |
|------|------|
| 环境不一致 | 命令执行失败，打断业务流程 |
| 手动检查繁琐 | 每次都要检查 Python、虚拟环境、依赖 |
| 修复方式不统一 | 不同人有不同的修复习惯 |

### 1.2 设计目标

提供**统一的环境检测与修复**：
- 自动检测环境问题
- 能修复的自动修复
- 不能修复的给出明确建议

---

## 二、职责

### 2.1 做什么

1. 检测 Python 版本是否满足要求
2. 检测 uv 是否可用
3. 检测/创建虚拟环境
4. 安装依赖
5. 输出项目配置信息

### 2.2 不做什么

- 不修改业务代码
- 不执行业务逻辑
- 不进行流程追踪

---

## 三、接口规格

### 3.1 命令语法

```
aide env ensure [--runtime]
```

### 3.2 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `--runtime` | 可选 | 仅检查 aide 运行时环境，不依赖配置文件 |

### 3.3 输出

**成功（runtime 模式）**：
```
✓ 运行时环境就绪 (python:3.12, uv:0.4.0)
```

**成功（完整模式）**：
```
→ 任务原文档: task-now.md
→ 任务细则文档: task-spec.md
✓ 环境就绪 (python:3.12, uv:0.4.0, venv:.venv)
```

**自动修复**：
```
→ 创建虚拟环境: .venv
✓ 已创建虚拟环境
⚠ 未找到 requirements.txt，已创建空文件
→ 安装依赖（uv pip install -r requirements.txt）
✓ 环境就绪 (python:3.12, uv:0.4.0, venv:.venv)
```

**失败**：
```
✗ Python 版本不足，要求>=3.11，当前 3.9
```

```
✗ 未检测到 uv，请先安装（FileNotFoundError）
```

---

## 四、业务流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

if (--runtime 参数?) then (是)
  :required_py = "3.11" (硬编码);
else (否)
  :从配置文件读取 required_py;
endif

:检查 Python 版本;
if (版本满足?) then (是)
else (否)
  :输出错误信息;
  stop
endif

:检查 uv 可用性;
if (uv 可用?) then (是)
else (否)
  :输出错误信息;
  stop
endif

if (--runtime 参数?) then (是)
  :输出运行时环境就绪;
  stop
endif

:读取配置文件;
:确保 .gitignore 包含 .aide/;

:读取 venv 路径配置;
if (虚拟环境存在?) then (是)
else (否)
  :使用 uv venv 创建;
  if (创建成功?) then (是)
  else (否)
    :输出错误信息;
    stop
  endif
endif

:读取 requirements 路径配置;
if (requirements.txt 存在?) then (是)
else (否)
  :创建空文件;
  :输出警告;
endif

:使用 uv pip install 安装依赖;
if (安装成功?) then (是)
else (否)
  :输出错误信息;
  stop
endif

:输出任务文档路径配置;
:输出环境就绪;

stop
@enduml
```

---

## 五、数据结构

### 5.1 配置依赖

从 `.aide/config.toml` 读取：

```
[runtime]
python_min     # Python 最低版本要求

[env]
venv           # 虚拟环境路径
requirements   # 依赖文件路径

[task]
source         # 任务原文档路径
spec           # 任务细则文档路径
```

### 5.2 方法签名原型

```
class EnvManager:
    root: Path              # 项目根目录

    ensure(runtime_only: bool, cfg: ConfigManager) -> bool
        # 主入口，返回是否成功

    _get_required_python(cfg: ConfigManager, runtime_only: bool) -> str
        # 获取 Python 版本要求

    _parse_version(version: str) -> tuple[int, ...]
        # 解析版本号字符串

    _check_python_version(required: str) -> bool
        # 检查 Python 版本

    _check_uv() -> str | None
        # 检查 uv，返回版本号或 None

    _ensure_venv(venv_path: Path) -> bool
        # 确保虚拟环境存在

    _ensure_requirements_file(req_path: Path) -> None
        # 确保 requirements.txt 存在

    _install_requirements(venv_path: Path, req_path: Path) -> bool
        # 安装依赖
```

---

## 六、依赖

| 依赖项 | 类型 | 说明 |
|--------|------|------|
| ConfigManager | 内部模块 | 配置读写 |
| output | 内部模块 | 输出格式化 |
| uv | 外部工具 | 虚拟环境和依赖管理 |

---

## 七、被依赖

| 依赖方 | 说明 |
|--------|------|
| /aide:init | 调用 env ensure --runtime 和 env ensure |
| aide init | 内部可能调用 env 检查 |

---

## 八、修改指南

### 8.1 修改检测逻辑

1. 更新本文档的业务流程图
2. 修改 `aide/env/ensure.py`
3. 如有新的输出，更新输出示例

### 8.2 添加新的检测项

1. 在本文档添加检测项说明
2. 在 `EnvManager` 添加对应方法
3. 在 `ensure()` 中调用
4. 更新 [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)

### 8.3 修改配置依赖

1. 更新本文档的"配置依赖"章节
2. 修改代码实现
3. 同步更新 [配置格式文档](../formats/config.md)

---

## 九、相关文档

- [program 导览](../README.md)
- [配置格式文档](../formats/config.md)
- [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)
- [/aide:init 命令设计](../../../aide-marketplace/aide-plugin/docs/commands/init.md)
