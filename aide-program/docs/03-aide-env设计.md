# aide env 设计

## 一、命令概述

### 1.1 功能定位

`aide env` 命令用于检测和修复项目开发环境，确保所有必需的工具和依赖都已正确安装。

### 1.2 执行时机

- `/aide:init` 命令中调用（两次）
- 用户手动检查环境时
- 环境配置变更后

### 1.3 命令格式

```bash
aide env ensure [--runtime]
```

**选项**：
- `--runtime`：仅检查 aide 运行时环境（不依赖配置文件）

---

## 二、功能需求

### 2.1 aide env ensure --runtime

**用途**：检查 aide 程序自身运行所需的环境

**检查项**：
1. Python 版本（>= 3.10）
2. 必需的 Python 库（tomli/tomllib, tomli-w）

**特点**：
- 不读取项目配置文件
- 在 `aide init` 之前执行
- 失败时给出明确的安装建议

**输出示例**：

成功：
```
✓ 环境就绪 (python:3.12)
```

失败：
```
✗ Python 版本不满足要求 (需要 >=3.10, 当前 3.8)
  建议: 安装 Python 3.10+ 或使用 pyenv 管理版本
  文档: https://www.python.org/downloads/
```

### 2.2 aide env ensure

**用途**：检查项目开发环境

**检查项**：
1. Python 版本（根据配置文件）
2. 虚拟环境（根据配置文件）
3. 可选工具（git, uv 等，根据配置文件）

**特点**：
- 读取 `.aide/config.toml`
- 在 `aide init` 之后执行
- 可以自动修复部分问题

**输出示例**：

成功（无问题）：
```
✓ 环境就绪 (python:3.12, git:2.40.0)
```

成功（自动修复）：
```
⚠ 已修复: 创建虚拟环境 .venv
✓ 环境就绪 (python:3.12)
```

失败（无法修复）：
```
✗ Python 版本不满足要求 (需要 >=3.11, 当前 3.10)
  建议: 升级 Python 到 3.11+ 或修改配置文件
  配置: .aide/config.toml (env.python.version)
```

---

## 三、实现设计

### 3.1 函数接口

```python
def cmd_env(args: list[str]) -> int:
    """aide env 命令处理

    Args:
        args: 命令参数

    Returns:
        退出码（0 表示成功）
    """
    pass

def env_ensure(runtime_only: bool = False) -> int:
    """环境检测和修复

    Args:
        runtime_only: 是否仅检查运行时环境

    Returns:
        退出码（0 表示成功）
    """
    pass
```

### 3.2 实现流程

#### 3.2.1 aide env ensure --runtime

```python
import sys
import subprocess
from pathlib import Path
from core.output import ok, err

def env_ensure_runtime() -> int:
    """检查 aide 运行时环境"""

    # 1. 检查 Python 版本
    python_version = sys.version_info
    if python_version.major < 3 or (python_version.major == 3 and python_version.minor < 10):
        err(
            f"Python 版本不满足要求 (需要 >=3.10, 当前 {python_version.major}.{python_version.minor})",
            [
                "建议: 安装 Python 3.10+ 或使用 pyenv 管理版本",
                "文档: https://www.python.org/downloads/"
            ]
        )
        return 3

    # 2. 检查必需的库
    try:
        # Python 3.11+ 内置 tomllib
        if python_version.minor >= 11:
            import tomllib
        else:
            import tomli as tomllib

        import tomli_w
    except ImportError as e:
        err(
            f"缺少必需的 Python 库: {e.name}",
            [
                "建议: 运行 'pip install tomli tomli-w'",
                "或使用 uv: 'uv pip install tomli tomli-w'"
            ]
        )
        return 3

    # 3. 输出成功信息
    version_str = f"{python_version.major}.{python_version.minor}.{python_version.micro}"
    ok(f"环境就绪 (python:{version_str})")

    return 0
```

#### 3.2.2 aide env ensure

```python
from core.config import Config
from core.output import ok, warn, err

def env_ensure_project() -> int:
    """检查项目开发环境"""

    # 1. 加载配置
    try:
        config = Config(Path.cwd())
        config.load()
    except FileNotFoundError:
        err(
            "配置文件不存在",
            [
                "位置: .aide/config.toml",
                "建议: 运行 'aide init' 创建配置文件"
            ]
        )
        return 4
    except Exception as e:
        err(
            "配置文件读取失败",
            [
                f"原因: {str(e)}",
                "建议: 检查配置文件格式"
            ]
        )
        return 4

    # 2. 检查 Python 版本
    required_version = config.get("env.python.version", ">=3.10")
    if not check_python_version(required_version):
        err(
            f"Python 版本不满足要求 (需要 {required_version}, 当前 {get_python_version()})",
            [
                "建议: 升级 Python 或修改配置文件",
                "配置: .aide/config.toml (env.python.version)"
            ]
        )
        return 3

    # 3. 检查虚拟环境
    venv_path = config.get("env.python.venv", ".venv")
    venv_result = check_venv(venv_path)

    if venv_result == "missing":
        # 尝试创建虚拟环境
        if create_venv(venv_path):
            warn(f"已修复: 创建虚拟环境 {venv_path}")
        else:
            err(
                f"虚拟环境不存在且无法创建 ({venv_path})",
                [
                    "建议: 手动创建虚拟环境",
                    f"命令: python3 -m venv {venv_path}"
                ]
            )
            return 3

    # 4. 检查可选工具
    tools_info = []

    # 检查 git
    if config.get("env.tools.git", True):
        git_version = get_tool_version("git")
        if git_version:
            tools_info.append(f"git:{git_version}")
        else:
            warn("git 未安装（可选）")

    # 检查 uv
    if config.get("env.tools.uv", False):
        uv_version = get_tool_version("uv")
        if uv_version:
            tools_info.append(f"uv:{uv_version}")
        else:
            warn("uv 未安装（可选）")

    # 5. 输出成功信息
    python_version = get_python_version()
    info_parts = [f"python:{python_version}"] + tools_info
    ok(f"环境就绪 ({', '.join(info_parts)})")

    return 0
```

### 3.3 辅助函数

#### 3.3.1 版本检查

```python
import re
from packaging import version

def check_python_version(requirement: str) -> bool:
    """检查 Python 版本是否满足要求

    Args:
        requirement: 版本要求（如 ">=3.10"）

    Returns:
        是否满足要求
    """
    current = get_python_version()

    # 解析要求
    match = re.match(r'(>=|<=|>|<|==)?(\d+\.\d+(?:\.\d+)?)', requirement)
    if not match:
        return True  # 无法解析，假设满足

    operator, required = match.groups()
    operator = operator or "=="

    # 比较版本
    try:
        current_ver = version.parse(current)
        required_ver = version.parse(required)

        if operator == ">=":
            return current_ver >= required_ver
        elif operator == "<=":
            return current_ver <= required_ver
        elif operator == ">":
            return current_ver > required_ver
        elif operator == "<":
            return current_ver < required_ver
        elif operator == "==":
            return current_ver == required_ver
    except Exception:
        return True

    return True

def get_python_version() -> str:
    """获取当前 Python 版本

    Returns:
        版本字符串（如 "3.12.0"）
    """
    v = sys.version_info
    return f"{v.major}.{v.minor}.{v.micro}"
```

#### 3.3.2 虚拟环境检查

```python
import subprocess

def check_venv(venv_path: str) -> str:
    """检查虚拟环境状态

    Args:
        venv_path: 虚拟环境路径

    Returns:
        状态：'ok', 'missing', 'invalid'
    """
    venv_dir = Path(venv_path)

    if not venv_dir.exists():
        return "missing"

    # 检查是否是有效的虚拟环境
    if sys.platform == "win32":
        python_exe = venv_dir / "Scripts" / "python.exe"
    else:
        python_exe = venv_dir / "bin" / "python"

    if python_exe.exists():
        return "ok"
    else:
        return "invalid"

def create_venv(venv_path: str) -> bool:
    """创建虚拟环境

    Args:
        venv_path: 虚拟环境路径

    Returns:
        是否创建成功
    """
    try:
        subprocess.run(
            [sys.executable, "-m", "venv", venv_path],
            check=True,
            capture_output=True,
            text=True
        )
        return True
    except subprocess.CalledProcessError:
        return False
    except Exception:
        return False
```

#### 3.3.3 工具版本检查

```python
def get_tool_version(tool: str) -> str | None:
    """获取工具版本

    Args:
        tool: 工具名称（如 "git", "uv"）

    Returns:
        版本字符串，未安装则返回 None
    """
    try:
        result = subprocess.run(
            [tool, "--version"],
            capture_output=True,
            text=True,
            timeout=5
        )

        if result.returncode == 0:
            # 解析版本号
            output = result.stdout.strip()
            match = re.search(r'(\d+\.\d+\.\d+)', output)
            if match:
                return match.group(1)
            return "unknown"
    except (subprocess.TimeoutExpired, FileNotFoundError):
        pass

    return None
```

---

## 四、错误处理

### 4.1 错误分类

| 错误类型 | 退出码 | 处理方式 |
|---------|--------|---------|
| Python 版本不满足 | 3 | 显示当前版本和要求，给出升级建议 |
| 配置文件不存在 | 4 | 提示运行 aide init |
| 配置文件格式错误 | 4 | 显示错误位置和原因 |
| 虚拟环境无法创建 | 3 | 显示创建命令 |
| 必需工具未安装 | 3 | 显示安装建议 |

### 4.2 错误恢复

```python
def env_ensure_with_retry(runtime_only: bool = False, max_retries: int = 3) -> int:
    """带重试的环境检测

    Args:
        runtime_only: 是否仅检查运行时
        max_retries: 最大重试次数

    Returns:
        退出码
    """
    for attempt in range(max_retries):
        result = env_ensure(runtime_only)

        if result == 0:
            return 0

        if attempt < max_retries - 1:
            info(f"重试 ({attempt + 1}/{max_retries})...")

    return result
```

---

## 五、测试用例

### 5.1 runtime 模式测试

```python
def test_env_ensure_runtime_success():
    """测试运行时环境检测成功"""
    result = env_ensure_runtime()
    assert result == 0

def test_env_ensure_runtime_old_python(monkeypatch):
    """测试 Python 版本过低"""
    # 模拟 Python 3.8
    monkeypatch.setattr(sys, "version_info", (3, 8, 0, "final", 0))

    result = env_ensure_runtime()
    assert result == 3

def test_env_ensure_runtime_missing_lib(monkeypatch):
    """测试缺少必需库"""
    # 模拟 import 失败
    def mock_import(name, *args, **kwargs):
        if name in ["tomli", "tomllib"]:
            raise ImportError(f"No module named '{name}'")
        return __import__(name, *args, **kwargs)

    monkeypatch.setattr("builtins.__import__", mock_import)

    result = env_ensure_runtime()
    assert result == 3
```

### 5.2 项目环境测试

```python
def test_env_ensure_project_success(tmp_path, monkeypatch):
    """测试项目环境检测成功"""
    monkeypatch.chdir(tmp_path)

    # 创建配置文件
    cmd_init([])

    # 检测环境
    result = env_ensure_project()
    assert result == 0

def test_env_ensure_project_no_config(tmp_path, monkeypatch):
    """测试配置文件不存在"""
    monkeypatch.chdir(tmp_path)

    result = env_ensure_project()
    assert result == 4

def test_env_ensure_project_create_venv(tmp_path, monkeypatch):
    """测试自动创建虚拟环境"""
    monkeypatch.chdir(tmp_path)

    # 创建配置文件
    cmd_init([])

    # 确保虚拟环境不存在
    venv_path = tmp_path / ".venv"
    if venv_path.exists():
        shutil.rmtree(venv_path)

    # 检测环境（应该自动创建虚拟环境）
    result = env_ensure_project()
    assert result == 0
    assert venv_path.exists()
```

### 5.3 版本检查测试

```python
def test_check_python_version():
    """测试 Python 版本检查"""
    # 测试各种版本要求
    assert check_python_version(">=3.10") == True  # 假设当前是 3.12
    assert check_python_version(">=3.15") == False
    assert check_python_version("==3.12") == True
    assert check_python_version("<4.0") == True

def test_get_python_version():
    """测试获取 Python 版本"""
    version = get_python_version()
    assert re.match(r'\d+\.\d+\.\d+', version)
```

---

## 六、性能要求

### 6.1 执行时间

- `--runtime` 模式：< 100ms
- 项目环境检测（无修复）：< 500ms
- 项目环境检测（创建虚拟环境）：< 10s

### 6.2 资源占用

- 内存：< 50MB
- CPU：低（主要是 I/O 操作）

---

## 七、集成测试

### 7.1 完整工作流

```python
def test_env_workflow(tmp_path, monkeypatch):
    """测试完整的环境检测工作流"""
    monkeypatch.chdir(tmp_path)

    # 1. 检查运行时环境
    result = env_ensure_runtime()
    assert result == 0

    # 2. 初始化项目
    result = cmd_init([])
    assert result == 0

    # 3. 检查项目环境
    result = env_ensure_project()
    assert result == 0

    # 4. 验证虚拟环境已创建
    assert (tmp_path / ".venv").exists()
```

---

## 八、总结

### 8.1 核心要点

1. 两种模式：runtime 和 project
2. 自动修复部分问题（如创建虚拟环境）
3. 清晰的错误信息和建议
4. 完善的版本检查逻辑

### 8.2 实现检查清单

- [ ] 实现 cmd_env 函数
- [ ] 实现 env_ensure_runtime 函数
- [ ] 实现 env_ensure_project 函数
- [ ] 实现版本检查函数
- [ ] 实现虚拟环境检查和创建
- [ ] 实现工具版本检查
- [ ] 编写单元测试
- [ ] 编写集成测试
- [ ] 性能测试

---

**版本**：v1.0
**更新日期**：2025-12-13
