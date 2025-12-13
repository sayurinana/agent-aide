# aide env 重新设计 - 实现计划

## 一、设计概要

### 1.1 命令结构

```
aide env                         # 等同于 aide env ensure
aide env ensure [options]        # 检测并修复
aide env list                    # 列出所有可用模块
```

### 1.2 参数

| 参数 | 说明 |
|------|------|
| `--runtime` | 仅检测 aide 运行时环境（python + uv） |
| `--modules M1,M2` | 指定要检测的模块（逗号分隔） |
| `--all` | 检测所有已启用模块，仅检查不修复 |

### 1.3 模块分类

**类型A：自包含模块（无需配置即可检测）**
- python, uv, java, go, rust, gcc, cmake, node, flutter

**类型B：路径依赖模块（必须有配置才能检测）**
- venv, requirements, npm

---

## 二、目录结构变更

```
aide-program/aide/
├── __init__.py
├── __main__.py
├── main.py                    # [修改] 更新 CLI 路由
├── core/
│   ├── __init__.py
│   ├── config.py              # [修改] 更新默认配置
│   └── output.py
└── env/
    ├── __init__.py
    ├── ensure.py              # [删除] 旧实现
    ├── manager.py             # [新建] 环境管理器主入口
    ├── registry.py            # [新建] 模块注册表
    └── modules/               # [新建] 模块目录
        ├── __init__.py
        ├── base.py            # [新建] 模块基类
        ├── python.py          # [新建] Python 模块
        ├── uv.py              # [新建] uv 模块
        ├── venv.py            # [新建] venv 模块
        ├── requirements.py    # [新建] requirements 模块
        ├── node.py            # [新建] Node.js 模块
        ├── npm.py             # [新建] npm 模块
        └── ...                # 其他模块按需添加
```

---

## 三、配置文件变更

### 3.1 新默认配置

```toml
# Aide 默认配置（由 aide init 生成）

[runtime]
python_min = "3.11"
use_uv = true

[task]
source = "task-now.md"
spec = "task-spec.md"

[env]
# 启用的模块列表
modules = ["python", "uv", "venv", "requirements"]

# 类型A模块配置（可选，指定版本要求）
[env.python]
min_version = "3.11"

# 类型B模块配置（必需，指定路径）
[env.venv]
path = ".venv"

[env.requirements]
path = "requirements.txt"

[flow]
phases = ["task-optimize", "flow-design", "impl", "verify", "docs", "finish"]
```

### 3.2 配置兼容性

旧配置格式：
```toml
[env]
venv = ".venv"
requirements = "requirements.txt"
```

新配置格式：
```toml
[env]
modules = ["python", "uv", "venv", "requirements"]

[env.venv]
path = ".venv"

[env.requirements]
path = "requirements.txt"
```

**迁移策略**：读取时兼容旧格式，写入时使用新格式。

---

## 四、核心类设计

### 4.1 模块基类 (`env/modules/base.py`)

```python
from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Any
from pathlib import Path

@dataclass
class CheckResult:
    """检测结果"""
    success: bool
    version: str | None = None
    message: str | None = None
    can_ensure: bool = False  # 失败时是否可修复

@dataclass
class ModuleInfo:
    """模块元信息"""
    name: str
    description: str
    capabilities: list[str]  # ["check"] 或 ["check", "ensure"]
    requires_config: bool    # 是否需要配置（类型B）
    config_keys: list[str]   # 需要的配置键，如 ["path"]

class BaseModule(ABC):
    """模块基类"""

    @property
    @abstractmethod
    def info(self) -> ModuleInfo:
        """返回模块元信息"""
        pass

    @abstractmethod
    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测环境"""
        pass

    def ensure(self, config: dict[str, Any], root: Path) -> CheckResult:
        """修复环境（可选实现）"""
        return CheckResult(
            success=False,
            message="此模块不支持自动修复"
        )
```

### 4.2 模块注册表 (`env/registry.py`)

```python
from aide.env.modules.base import BaseModule, ModuleInfo

class ModuleRegistry:
    """模块注册表"""

    _modules: dict[str, BaseModule] = {}

    @classmethod
    def register(cls, module: BaseModule) -> None:
        cls._modules[module.info.name] = module

    @classmethod
    def get(cls, name: str) -> BaseModule | None:
        return cls._modules.get(name)

    @classmethod
    def all(cls) -> dict[str, BaseModule]:
        return cls._modules.copy()

    @classmethod
    def list_info(cls) -> list[ModuleInfo]:
        return [m.info for m in cls._modules.values()]

# 自动注册所有模块
def _auto_register():
    from aide.env.modules import python, uv, venv, requirements, node, npm
    # 每个模块文件导出一个 module 实例
    for mod in [python, uv, venv, requirements, node, npm]:
        if hasattr(mod, 'module'):
            ModuleRegistry.register(mod.module)

_auto_register()
```

### 4.3 环境管理器 (`env/manager.py`)

```python
from pathlib import Path
from typing import Any

from aide.core import output
from aide.core.config import ConfigManager
from aide.env.registry import ModuleRegistry
from aide.env.modules.base import CheckResult

class EnvManager:
    """环境管理器"""

    def __init__(self, root: Path, cfg: ConfigManager):
        self.root = root
        self.cfg = cfg

    def list_modules(self) -> None:
        """列出所有可用模块"""
        # 实现 aide env list
        pass

    def ensure(
        self,
        runtime_only: bool = False,
        modules: list[str] | None = None,
        check_only: bool = False,  # --all 时为 True
    ) -> bool:
        """检测并修复环境"""
        # 主逻辑实现
        pass

    def _get_enabled_modules(self) -> list[str]:
        """获取已启用的模块列表"""
        pass

    def _get_module_config(self, name: str) -> dict[str, Any]:
        """获取模块配置"""
        pass

    def _check_module(self, name: str, is_enabled: bool) -> tuple[bool, CheckResult]:
        """检测单个模块"""
        pass

    def _ensure_module(self, name: str, is_enabled: bool) -> tuple[bool, CheckResult]:
        """检测并修复单个模块"""
        pass
```

---

## 五、执行逻辑详解

### 5.1 `aide env ensure` 流程

```
1. 读取配置
2. 获取 modules 列表（已启用模块）
3. 对每个模块：
   a. 获取模块实例
   b. 获取模块配置
   c. 检查类型B模块是否有必需配置
      - 已启用 + 无配置 → ✗ 错误，停止
      - 未启用 + 无配置 → ⚠ 警告，跳过
   d. 执行 check()
   e. 如果失败且可修复：执行 ensure()
   f. 根据启用状态决定输出级别
4. 输出最终状态
```

### 5.2 `aide env ensure --all` 流程

```
1. 读取配置
2. 获取 modules 列表
   - 有列表 → 使用列表
   - 无列表 → ⚠ 警告 + 使用所有已注册模块
3. 对每个模块：
   a. 仅执行 check()（不修复）
   b. 输出检测结果
4. 输出汇总
```

### 5.3 `aide env ensure --modules X,Y` 流程

```
1. 解析指定的模块列表
2. 读取配置，获取已启用列表
3. 对每个指定模块：
   a. 判断是否在启用列表中
   b. 检查类型B模块配置
   c. 执行 check() + ensure()
   d. 根据启用状态决定输出级别
4. 输出最终状态
```

---

## 六、输出级别规则

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

---

## 七、实现步骤

### 阶段1：基础架构

1. 创建 `env/modules/` 目录结构
2. 实现 `base.py` 模块基类
3. 实现 `registry.py` 模块注册表
4. 更新 `core/config.py` 默认配置

### 阶段2：核心模块实现

5. 实现 `python.py` 模块
6. 实现 `uv.py` 模块
7. 实现 `venv.py` 模块
8. 实现 `requirements.py` 模块

### 阶段3：管理器与 CLI

9. 实现 `manager.py` 环境管理器
10. 更新 `main.py` CLI 路由
11. 删除旧的 `ensure.py`

### 阶段4：扩展模块（可选）

12. 实现 `node.py` 模块
13. 实现 `npm.py` 模块
14. 其他模块按需添加

### 阶段5：文档与测试

15. 更新 `docs/commands/env.md` 设计文档
16. 更新 `docs/formats/config.md` 配置文档
17. 添加测试用例

---

## 八、向后兼容

### 8.1 命令兼容

| 旧命令 | 新行为 |
|--------|--------|
| `aide env ensure` | 保持不变 |
| `aide env ensure --runtime` | 保持不变 |

### 8.2 配置兼容

读取配置时检测旧格式并转换：

```python
def _migrate_config(config: dict) -> dict:
    """兼容旧配置格式"""
    env = config.get("env", {})

    # 如果没有 modules 字段，使用默认值
    if "modules" not in env:
        env["modules"] = ["python", "uv", "venv", "requirements"]

    # 如果使用旧的 venv/requirements 字段
    if "venv" in env and not isinstance(env["venv"], dict):
        old_venv = env.pop("venv")
        env.setdefault("venv", {})["path"] = old_venv

    if "requirements" in env and not isinstance(env["requirements"], dict):
        old_req = env.pop("requirements")
        env.setdefault("requirements", {})["path"] = old_req

    return config
```

---

## 九、相关文档

- [aide env 设计文档](./env.md) - 需更新
- [配置格式文档](../formats/config.md) - 需更新
- [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md) - 需更新
