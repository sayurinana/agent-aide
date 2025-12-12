# aide init 设计

## 一、命令概述

### 1.1 功能定位

`aide init` 命令用于初始化项目的 aide 工作环境，创建必要的目录和配置文件。

### 1.2 执行时机

- 首次在项目中使用 aide 时
- 配置文件丢失需要重新创建时
- 作为 `/aide:init` 命令的一部分

### 1.3 命令格式

```bash
aide init [options]
```

**选项**：
- 无（当前版本不需要选项）

---

## 二、功能需求

### 2.1 核心功能

1. **创建 .aide 目录**
   - 检查目录是否存在
   - 不存在则创建
   - 已存在则跳过（幂等性）

2. **生成配置文件**
   - 创建 `config.toml`
   - 包含详细注释
   - 使用合理的默认值

3. **更新 .gitignore**
   - 检查 `.gitignore` 是否存在
   - 添加 `.aide/` 到忽略列表
   - 避免重复添加

### 2.2 输出要求

**首次初始化**：
```
✓ 已创建 .aide/ 目录
✓ 已生成默认配置
✓ 已添加 .aide/ 到 .gitignore
```

**重复初始化**：
```
⚠ .aide/ 目录已存在
⚠ 配置文件已存在，跳过生成
```

**部分已存在**：
```
⚠ .aide/ 目录已存在
✓ 已生成默认配置
✓ 已添加 .aide/ 到 .gitignore
```

---

## 三、实现设计

### 3.1 函数接口

```python
def cmd_init(args: list[str]) -> int:
    """aide init 命令处理

    Args:
        args: 命令参数（当前版本为空）

    Returns:
        退出码（0 表示成功）
    """
    pass
```

### 3.2 实现流程

```python
from pathlib import Path
from core.output import ok, warn, err
from core.config import generate_default_config

def cmd_init(args: list[str]) -> int:
    """aide init 命令实现"""

    # 1. 获取项目根目录（当前工作目录）
    project_root = Path.cwd()
    aide_dir = project_root / ".aide"
    config_path = aide_dir / "config.toml"
    gitignore_path = project_root / ".gitignore"

    # 2. 创建 .aide 目录
    if aide_dir.exists():
        warn(".aide/ 目录已存在")
    else:
        aide_dir.mkdir(parents=True, exist_ok=True)
        ok("已创建 .aide/ 目录")

    # 3. 生成配置文件
    if config_path.exists():
        warn("配置文件已存在，跳过生成")
    else:
        config_content = generate_default_config()
        config_path.write_text(config_content, encoding="utf-8")
        ok("已生成默认配置")

    # 4. 更新 .gitignore
    gitignore_updated = update_gitignore(gitignore_path)
    if gitignore_updated:
        ok("已添加 .aide/ 到 .gitignore")

    return 0
```

### 3.3 辅助函数

#### 3.3.1 更新 .gitignore

```python
def update_gitignore(gitignore_path: Path) -> bool:
    """更新 .gitignore 文件

    Args:
        gitignore_path: .gitignore 文件路径

    Returns:
        是否进行了更新
    """
    aide_ignore = ".aide/"

    # 读取现有内容
    if gitignore_path.exists():
        content = gitignore_path.read_text(encoding="utf-8")
        lines = content.splitlines()
    else:
        lines = []

    # 检查是否已存在
    if aide_ignore in lines or ".aide" in lines:
        return False

    # 添加到末尾
    if lines and not lines[-1].strip():
        # 最后一行是空行，直接添加
        lines.append(aide_ignore)
    else:
        # 添加空行和 .aide/
        lines.extend(["", aide_ignore])

    # 写回文件
    gitignore_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return True
```

#### 3.3.2 生成默认配置

```python
def generate_default_config() -> str:
    """生成默认配置文件内容

    Returns:
        TOML 格式的配置内容
    """
    return '''# Aide 项目配置文件
# 由 aide init 自动生成
# 文档: https://github.com/your-org/aide

[task]
# 任务原文档路径（prep 阶段使用）
# 可通过 /aide:prep [路径] 覆盖
source = "task-now.md"

# 任务细则文档路径（exec 阶段使用）
# 可通过 /aide:exec [路径] 覆盖
spec = "task-spec.md"

[env]
# 环境配置

[env.python]
# Python 版本要求（语义化版本）
# 格式: ">=3.10" 或 ">=3.10,<4.0"
version = ">=3.10"

# 虚拟环境路径（相对于项目根目录）
venv = ".venv"

[env.tools]
# 可选工具配置

# 是否需要 uv（Python 包管理器）
uv = false

# 是否需要 git
git = true

[flow]
# 流程追踪配置（aide flow 命令使用）

# 环节列表（不建议修改）
phases = ["flow-design", "impl", "verify", "docs", "finish"]

# PlantUML 流程图目录
flowchart_dir = "program_flowchart"

[decide]
# 待定项确认配置（aide decide 命令使用）

# Web 服务端口
port = 3721

# 决策记录保存目录
decisions_dir = ".aide/decisions"

[output]
# 输出配置

# 是否启用颜色输出
# 可通过环境变量 NO_COLOR 禁用
color = true

# 输出语言（当前仅支持 zh-CN）
language = "zh-CN"
'''
```

---

## 四、错误处理

### 4.1 可能的错误

| 错误类型 | 处理方式 | 退出码 |
|---------|---------|--------|
| 无写入权限 | 显示错误信息和建议 | 1 |
| 磁盘空间不足 | 显示错误信息 | 1 |
| 配置文件格式错误 | 不应该发生（生成的内容固定） | - |

### 4.2 错误处理示例

```python
def cmd_init(args: list[str]) -> int:
    """aide init 命令实现（带错误处理）"""

    try:
        project_root = Path.cwd()
        aide_dir = project_root / ".aide"

        # 创建目录
        try:
            aide_dir.mkdir(parents=True, exist_ok=True)
        except PermissionError:
            err(
                "无法创建 .aide/ 目录",
                [
                    "原因: 权限不足",
                    f"位置: {aide_dir}",
                    "建议: 检查当前目录的写入权限"
                ]
            )
            return 1
        except OSError as e:
            err(
                "无法创建 .aide/ 目录",
                [
                    f"原因: {str(e)}",
                    f"位置: {aide_dir}"
                ]
            )
            return 1

        # 生成配置文件
        config_path = aide_dir / "config.toml"
        if not config_path.exists():
            try:
                config_content = generate_default_config()
                config_path.write_text(config_content, encoding="utf-8")
                ok("已生成默认配置")
            except OSError as e:
                err(
                    "无法创建配置文件",
                    [
                        f"原因: {str(e)}",
                        f"位置: {config_path}"
                    ]
                )
                return 1
        else:
            warn("配置文件已存在，跳过生成")

        # 更新 .gitignore
        gitignore_path = project_root / ".gitignore"
        try:
            if update_gitignore(gitignore_path):
                ok("已添加 .aide/ 到 .gitignore")
        except OSError as e:
            # .gitignore 更新失败不是致命错误
            warn(f"无法更新 .gitignore: {str(e)}")

        return 0

    except Exception as e:
        err(
            "初始化失败",
            [
                f"原因: {str(e)}",
                "建议: 检查目录权限和磁盘空间"
            ]
        )
        return 1
```

---

## 五、幂等性设计

### 5.1 幂等性要求

多次执行 `aide init` 应该：
1. 不破坏已有的配置
2. 不重复添加 .gitignore 条目
3. 给出清晰的提示信息

### 5.2 幂等性测试

```python
def test_init_idempotent():
    """测试 aide init 的幂等性"""

    # 第一次执行
    result1 = cmd_init([])
    assert result1 == 0

    # 验证文件已创建
    assert Path(".aide").exists()
    assert Path(".aide/config.toml").exists()

    # 第二次执行
    result2 = cmd_init([])
    assert result2 == 0

    # 验证配置文件内容未改变
    config1 = Path(".aide/config.toml").read_text()

    # 第三次执行
    result3 = cmd_init([])
    assert result3 == 0

    config2 = Path(".aide/config.toml").read_text()
    assert config1 == config2
```

---

## 六、测试用例

### 6.1 正常场景

```python
def test_init_success(tmp_path, monkeypatch):
    """测试正常初始化"""
    # 切换到临时目录
    monkeypatch.chdir(tmp_path)

    # 执行初始化
    result = cmd_init([])

    # 验证退出码
    assert result == 0

    # 验证目录创建
    assert (tmp_path / ".aide").exists()
    assert (tmp_path / ".aide").is_dir()

    # 验证配置文件创建
    config_path = tmp_path / ".aide" / "config.toml"
    assert config_path.exists()
    assert config_path.is_file()

    # 验证配置文件内容
    content = config_path.read_text()
    assert "[task]" in content
    assert "[env]" in content
    assert "source = " in content

    # 验证 .gitignore 更新
    gitignore_path = tmp_path / ".gitignore"
    assert gitignore_path.exists()
    gitignore_content = gitignore_path.read_text()
    assert ".aide/" in gitignore_content
```

### 6.2 重复初始化

```python
def test_init_already_exists(tmp_path, monkeypatch, capsys):
    """测试重复初始化"""
    monkeypatch.chdir(tmp_path)

    # 第一次初始化
    cmd_init([])

    # 第二次初始化
    result = cmd_init([])

    # 验证退出码
    assert result == 0

    # 验证输出包含警告
    captured = capsys.readouterr()
    assert "已存在" in captured.out
```

### 6.3 权限错误

```python
def test_init_permission_error(tmp_path, monkeypatch, capsys):
    """测试权限错误"""
    monkeypatch.chdir(tmp_path)

    # 创建只读目录
    tmp_path.chmod(0o444)

    # 执行初始化
    result = cmd_init([])

    # 恢复权限
    tmp_path.chmod(0o755)

    # 验证退出码
    assert result == 1

    # 验证错误信息
    captured = capsys.readouterr()
    assert "权限" in captured.out
```

### 6.4 .gitignore 已存在

```python
def test_init_gitignore_exists(tmp_path, monkeypatch):
    """测试 .gitignore 已存在的情况"""
    monkeypatch.chdir(tmp_path)

    # 创建已有的 .gitignore
    gitignore_path = tmp_path / ".gitignore"
    gitignore_path.write_text("*.pyc\n__pycache__/\n")

    # 执行初始化
    result = cmd_init([])

    # 验证退出码
    assert result == 0

    # 验证 .gitignore 内容
    content = gitignore_path.read_text()
    assert "*.pyc" in content
    assert ".aide/" in content

    # 验证不重复添加
    lines = content.splitlines()
    aide_count = sum(1 for line in lines if ".aide" in line)
    assert aide_count == 1
```

---

## 七、集成测试

### 7.1 完整工作流测试

```python
def test_init_workflow(tmp_path, monkeypatch):
    """测试完整的初始化工作流"""
    monkeypatch.chdir(tmp_path)

    # 1. 执行初始化
    result = cmd_init([])
    assert result == 0

    # 2. 验证可以读取配置
    from core.config import Config
    config = Config(tmp_path)
    config.load()

    # 3. 验证配置值
    assert config.get("task.source") == "task-now.md"
    assert config.get("task.spec") == "task-spec.md"
    assert config.get("env.python.version") == ">=3.10"

    # 4. 验证可以修改配置
    config.set("task.source", "new-task.md")
    assert config.get("task.source") == "new-task.md"
```

---

## 八、性能要求

### 8.1 执行时间

- 正常情况：< 100ms
- 包含 .gitignore 更新：< 200ms

### 8.2 资源占用

- 内存：< 10MB
- 磁盘空间：< 10KB（配置文件）

---

## 九、总结

### 9.1 核心要点

1. 创建 .aide 目录和配置文件
2. 自动更新 .gitignore
3. 幂等性设计，可重复执行
4. 完善的错误处理

### 9.2 实现检查清单

- [ ] 实现 cmd_init 函数
- [ ] 实现 generate_default_config 函数
- [ ] 实现 update_gitignore 函数
- [ ] 实现错误处理
- [ ] 编写单元测试
- [ ] 编写集成测试
- [ ] 验证幂等性
- [ ] 性能测试

---

**版本**：v1.0
**更新日期**：2025-12-13
