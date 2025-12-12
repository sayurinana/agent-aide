# aide config 设计

## 一、命令概述

### 1.1 功能定位

`aide config` 命令用于读取和修改项目配置文件，提供命令行方式访问配置。

### 1.2 执行时机

- 用户需要查看配置值时
- 用户需要修改配置值时
- Commands 中需要获取配置时（如 prep/exec 的默认文档路径）

### 1.3 命令格式

```bash
aide config get <key>
aide config set <key> <value>
```

**参数**：
- `<key>`：配置键，支持点号分隔（如 `task.source`）
- `<value>`：配置值（仅 set 命令需要）

---

## 二、功能需求

### 2.1 aide config get

**功能**：获取配置值

**输出格式**：

单个值：
```
task.source = "task-now.md"
```

数组值：
```
flow.phases = ["flow-design", "impl", "verify", "docs", "finish"]
```

布尔值：
```
env.tools.git = true
```

**错误处理**：

配置键不存在：
```
✗ 配置键不存在: invalid.key
  可用的配置键: task.source, task.spec, env.python.version
```

配置文件不存在：
```
✗ 配置文件不存在
  位置: .aide/config.toml
  建议: 运行 'aide init' 创建配置文件
```

### 2.2 aide config set

**功能**：设置配置值

**输出**：

成功时无输出（静默原则）

**错误处理**：

配置键不存在：
```
✗ 配置键不存在: invalid.key
  可用的配置键: task.source, task.spec, env.python.version
```

配置值类型错误：
```
✗ 配置值类型错误
  键: env.tools.git
  期望类型: boolean
  实际值: "yes"
  建议: 使用 true 或 false
```

---

## 三、实现设计

### 3.1 函数接口

```python
def cmd_config(args: list[str]) -> int:
    """aide config 命令处理

    Args:
        args: 命令参数

    Returns:
        退出码（0 表示成功）
    """
    pass

def config_get(key: str) -> int:
    """获取配置值

    Args:
        key: 配置键

    Returns:
        退出码
    """
    pass

def config_set(key: str, value: str) -> int:
    """设置配置值

    Args:
        key: 配置键
        value: 配置值（字符串形式）

    Returns:
        退出码
    """
    pass
```

### 3.2 实现流程

#### 3.2.1 cmd_config

```python
from core.output import err

def cmd_config(args: list[str]) -> int:
    """aide config 命令处理"""

    if not args:
        err(
            "缺少子命令",
            [
                "用法: aide config get <key>",
                "      aide config set <key> <value>"
            ]
        )
        return 2

    subcommand = args[0]

    if subcommand == "get":
        if len(args) < 2:
            err("缺少参数: key")
            return 2
        return config_get(args[1])

    elif subcommand == "set":
        if len(args) < 3:
            err("缺少参数: key 或 value")
            return 2
        return config_set(args[1], args[2])

    else:
        err(
            f"未知子命令: {subcommand}",
            ["可用子命令: get, set"]
        )
        return 2
```

#### 3.2.2 config_get

```python
from pathlib import Path
from core.config import Config
from core.output import err

def config_get(key: str) -> int:
    """获取配置值"""

    try:
        # 加载配置
        config = Config(Path.cwd())
        config.load()

        # 获取值
        value = config.get(key)

        if value is None:
            # 配置键不存在
            available_keys = get_available_keys(config._config)
            err(
                f"配置键不存在: {key}",
                [f"可用的配置键: {', '.join(available_keys)}"]
            )
            return 4

        # 格式化输出
        formatted_value = format_config_value(value)
        print(f"{key} = {formatted_value}")

        return 0

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
            "配置读取失败",
            [f"原因: {str(e)}"]
        )
        return 4
```

#### 3.2.3 config_set

```python
def config_set(key: str, value_str: str) -> int:
    """设置配置值"""

    try:
        # 加载配置
        config = Config(Path.cwd())
        config.load()

        # 验证配置键是否存在
        if not is_valid_config_key(key):
            available_keys = get_available_keys(config._config)
            err(
                f"配置键不存在: {key}",
                [f"可用的配置键: {', '.join(available_keys)}"]
            )
            return 4

        # 解析值
        try:
            value = parse_config_value(key, value_str, config._config)
        except ValueError as e:
            err(
                "配置值类型错误",
                [
                    f"键: {key}",
                    f"原因: {str(e)}"
                ]
            )
            return 4

        # 设置值
        config.set(key, value)

        # 成功时无输出（静默原则）
        return 0

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
            "配置写入失败",
            [f"原因: {str(e)}"]
        )
        return 4
```

### 3.3 辅助函数

#### 3.3.1 格式化配置值

```python
import json

def format_config_value(value) -> str:
    """格式化配置值用于输出

    Args:
        value: 配置值

    Returns:
        格式化后的字符串
    """
    if isinstance(value, str):
        return f'"{value}"'
    elif isinstance(value, bool):
        return "true" if value else "false"
    elif isinstance(value, (list, dict)):
        return json.dumps(value, ensure_ascii=False)
    else:
        return str(value)
```

#### 3.3.2 解析配置值

```python
def parse_config_value(key: str, value_str: str, config: dict):
    """解析配置值

    Args:
        key: 配置键
        value_str: 值字符串
        config: 当前配置（用于推断类型）

    Returns:
        解析后的值

    Raises:
        ValueError: 值类型错误
    """
    # 获取当前值以推断类型
    current_value = get_config_value(config, key)

    if current_value is None:
        # 新键，尝试自动推断类型
        return auto_parse_value(value_str)

    # 根据当前值的类型解析
    if isinstance(current_value, bool):
        return parse_bool(value_str)
    elif isinstance(current_value, int):
        return parse_int(value_str)
    elif isinstance(current_value, float):
        return parse_float(value_str)
    elif isinstance(current_value, list):
        return parse_list(value_str)
    elif isinstance(current_value, dict):
        return parse_dict(value_str)
    else:
        # 字符串
        return value_str

def parse_bool(value_str: str) -> bool:
    """解析布尔值"""
    lower = value_str.lower()
    if lower in ["true", "yes", "1"]:
        return True
    elif lower in ["false", "no", "0"]:
        return False
    else:
        raise ValueError(f"无效的布尔值: {value_str}，使用 true 或 false")

def parse_int(value_str: str) -> int:
    """解析整数"""
    try:
        return int(value_str)
    except ValueError:
        raise ValueError(f"无效的整数: {value_str}")

def parse_float(value_str: str) -> float:
    """解析浮点数"""
    try:
        return float(value_str)
    except ValueError:
        raise ValueError(f"无效的浮点数: {value_str}")

def parse_list(value_str: str) -> list:
    """解析列表（JSON 格式）"""
    try:
        value = json.loads(value_str)
        if not isinstance(value, list):
            raise ValueError("不是列表")
        return value
    except json.JSONDecodeError:
        raise ValueError(f"无效的列表格式: {value_str}，使用 JSON 格式")

def parse_dict(value_str: str) -> dict:
    """解析字典（JSON 格式）"""
    try:
        value = json.loads(value_str)
        if not isinstance(value, dict):
            raise ValueError("不是字典")
        return value
    except json.JSONDecodeError:
        raise ValueError(f"无效的字典格式: {value_str}，使用 JSON 格式")

def auto_parse_value(value_str: str):
    """自动推断并解析值"""
    # 尝试 JSON
    try:
        return json.loads(value_str)
    except json.JSONDecodeError:
        pass

    # 尝试布尔值
    if value_str.lower() in ["true", "false", "yes", "no"]:
        return parse_bool(value_str)

    # 尝试数字
    try:
        if "." in value_str:
            return float(value_str)
        else:
            return int(value_str)
    except ValueError:
        pass

    # 默认为字符串
    return value_str
```

#### 3.3.3 获取可用配置键

```python
def get_available_keys(config: dict, prefix: str = "") -> list[str]:
    """获取所有可用的配置键

    Args:
        config: 配置字典
        prefix: 键前缀

    Returns:
        配置键列表
    """
    keys = []

    for key, value in config.items():
        full_key = f"{prefix}{key}" if prefix else key

        if isinstance(value, dict):
            # 递归获取嵌套键
            keys.extend(get_available_keys(value, f"{full_key}."))
        else:
            keys.append(full_key)

    return sorted(keys)

def is_valid_config_key(key: str) -> bool:
    """验证配置键是否有效

    Args:
        key: 配置键

    Returns:
        是否有效
    """
    # 定义允许的配置键
    valid_keys = [
        "task.source",
        "task.spec",
        "env.python.version",
        "env.python.venv",
        "env.tools.uv",
        "env.tools.git",
        "flow.phases",
        "flow.flowchart_dir",
        "decide.port",
        "decide.decisions_dir",
        "output.color",
        "output.language"
    ]

    return key in valid_keys
```

---

## 四、错误处理

### 4.1 错误分类

| 错误类型 | 退出码 | 处理方式 |
|---------|--------|---------|
| 缺少参数 | 2 | 显示用法 |
| 配置文件不存在 | 4 | 提示运行 aide init |
| 配置键不存在 | 4 | 显示可用的键 |
| 配置值类型错误 | 4 | 显示期望类型和建议 |
| 配置文件格式错误 | 4 | 显示错误位置 |

### 4.2 类型验证

```python
def validate_config_value(key: str, value) -> list[str]:
    """验证配置值

    Args:
        key: 配置键
        value: 配置值

    Returns:
        错误列表（空列表表示无错误）
    """
    errors = []

    # 验证特定键的值
    if key == "env.python.version":
        if not isinstance(value, str):
            errors.append("env.python.version 必须是字符串")
        elif not is_valid_version_spec(value):
            errors.append(f"无效的版本格式: {value}")

    elif key == "env.python.venv":
        if not isinstance(value, str):
            errors.append("env.python.venv 必须是字符串")

    elif key in ["env.tools.uv", "env.tools.git", "output.color"]:
        if not isinstance(value, bool):
            errors.append(f"{key} 必须是布尔值")

    elif key == "flow.phases":
        if not isinstance(value, list):
            errors.append("flow.phases 必须是数组")
        elif not all(isinstance(p, str) for p in value):
            errors.append("flow.phases 的元素必须是字符串")

    elif key == "decide.port":
        if not isinstance(value, int):
            errors.append("decide.port 必须是整数")
        elif not (1024 <= value <= 65535):
            errors.append("decide.port 必须在 1024-65535 之间")

    return errors
```

---

## 五、测试用例

### 5.1 config get 测试

```python
def test_config_get_success(tmp_path, monkeypatch, capsys):
    """测试获取配置成功"""
    monkeypatch.chdir(tmp_path)

    # 初始化配置
    cmd_init([])

    # 获取配置
    result = config_get("task.source")

    # 验证退出码
    assert result == 0

    # 验证输出
    captured = capsys.readouterr()
    assert 'task.source = "task-now.md"' in captured.out

def test_config_get_not_found(tmp_path, monkeypatch, capsys):
    """测试配置键不存在"""
    monkeypatch.chdir(tmp_path)

    # 初始化配置
    cmd_init([])

    # 获取不存在的配置
    result = config_get("invalid.key")

    # 验证退出码
    assert result == 4

    # 验证输出
    captured = capsys.readouterr()
    assert "配置键不存在" in captured.out

def test_config_get_no_config(tmp_path, monkeypatch, capsys):
    """测试配置文件不存在"""
    monkeypatch.chdir(tmp_path)

    # 获取配置（未初始化）
    result = config_get("task.source")

    # 验证退出码
    assert result == 4

    # 验证输出
    captured = capsys.readouterr()
    assert "配置文件不存在" in captured.out
```

### 5.2 config set 测试

```python
def test_config_set_success(tmp_path, monkeypatch):
    """测试设置配置成功"""
    monkeypatch.chdir(tmp_path)

    # 初始化配置
    cmd_init([])

    # 设置配置
    result = config_set("task.source", "new-task.md")

    # 验证退出码
    assert result == 0

    # 验证配置已更新
    config = Config(tmp_path)
    config.load()
    assert config.get("task.source") == "new-task.md"

def test_config_set_bool(tmp_path, monkeypatch):
    """测试设置布尔值"""
    monkeypatch.chdir(tmp_path)

    # 初始化配置
    cmd_init([])

    # 设置布尔值
    result = config_set("env.tools.uv", "true")
    assert result == 0

    # 验证
    config = Config(tmp_path)
    config.load()
    assert config.get("env.tools.uv") == True

def test_config_set_invalid_type(tmp_path, monkeypatch, capsys):
    """测试设置错误类型"""
    monkeypatch.chdir(tmp_path)

    # 初始化配置
    cmd_init([])

    # 设置错误类型
    result = config_set("env.tools.git", "yes")

    # 验证退出码
    assert result == 4

    # 验证输出
    captured = capsys.readouterr()
    assert "类型错误" in captured.out
```

### 5.3 值解析测试

```python
def test_parse_bool():
    """测试布尔值解析"""
    assert parse_bool("true") == True
    assert parse_bool("false") == False
    assert parse_bool("yes") == True
    assert parse_bool("no") == False

    with pytest.raises(ValueError):
        parse_bool("invalid")

def test_parse_list():
    """测试列表解析"""
    result = parse_list('["a", "b", "c"]')
    assert result == ["a", "b", "c"]

    with pytest.raises(ValueError):
        parse_list("not a list")

def test_auto_parse_value():
    """测试自动解析"""
    assert auto_parse_value("true") == True
    assert auto_parse_value("123") == 123
    assert auto_parse_value("3.14") == 3.14
    assert auto_parse_value('["a"]') == ["a"]
    assert auto_parse_value("text") == "text"
```

---

## 六、使用示例

### 6.1 查看配置

```bash
# 查看任务文档路径
aide config get task.source

# 查看 Python 版本要求
aide config get env.python.version

# 查看流程环节列表
aide config get flow.phases
```

### 6.2 修改配置

```bash
# 修改任务文档路径
aide config set task.source "my-task.md"

# 修改 Python 版本要求
aide config set env.python.version ">=3.11"

# 启用 uv
aide config set env.tools.uv true

# 修改端口
aide config set decide.port 8080
```

---

## 七、性能要求

### 7.1 执行时间

- config get：< 100ms
- config set：< 200ms

### 7.2 资源占用

- 内存：< 20MB
- 磁盘 I/O：最小化

---

## 八、总结

### 8.1 核心要点

1. 支持 get 和 set 两个子命令
2. 点号分隔的键访问
3. 自动类型推断和验证
4. 静默原则（set 成功时无输出）
5. 清晰的错误信息

### 8.2 实现检查清单

- [ ] 实现 cmd_config 函数
- [ ] 实现 config_get 函数
- [ ] 实现 config_set 函数
- [ ] 实现值格式化函数
- [ ] 实现值解析函数
- [ ] 实现类型验证
- [ ] 编写单元测试
- [ ] 编写集成测试
- [ ] 性能测试

---

**版本**：v1.0
**更新日期**：2025-12-13
