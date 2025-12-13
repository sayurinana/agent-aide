# aide init 子命令设计文档

## 一、背景

### 1.1 解决的问题

| 问题 | 影响 |
|------|------|
| 配置文件缺失 | 其他 aide 命令无法正常工作 |
| 目录结构不存在 | 状态文件、决策记录无处存放 |
| .gitignore 未配置 | aide 数据文件被提交到仓库 |

### 1.2 设计目标

提供**一键初始化**：
- 创建 .aide/ 目录结构
- 生成默认配置文件
- 配置 .gitignore

---

## 二、职责

### 2.1 做什么

1. 创建 `.aide/` 目录
2. 创建 `.aide/decisions/` 子目录
3. 创建 `.aide/logs/` 子目录
4. 生成默认 `config.toml`
5. 检查并更新 `.gitignore`

### 2.2 不做什么

- 不检测环境（那是 env 的职责）
- 不执行业务逻辑
- 不修改业务代码

---

## 三、接口规格

### 3.1 命令语法

```
aide init
```

### 3.2 参数

无参数。

### 3.3 输出

**首次初始化**：
```
✓ 已创建默认配置 .aide/config.toml
✓ 初始化完成，.aide/ 与默认配置已准备就绪
```

**已存在时**：
```
✓ 初始化完成，.aide/ 与默认配置已准备就绪
```

---

## 四、业务流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:创建 .aide/ 目录;
note right: 如已存在则跳过

:创建 .aide/decisions/ 目录;

:创建 .aide/logs/ 目录;

if (config.toml 存在?) then (是)
  :加载现有配置;
else (否)
  :生成默认配置;
  :写入 config.toml;
  :输出创建提示;
endif

:检查 .gitignore;
if (.aide/ 已在忽略列表?) then (是)
else (否)
  :添加 .aide/ 到 .gitignore;
endif

:输出初始化完成;

stop
@enduml
```

---

## 五、数据结构

### 5.1 目录结构

```
.aide/
├── config.toml          # 项目配置
├── flow-status.json     # 当前任务进度（由 flow 创建）
├── decisions/           # 待定项决策记录
│   └── {timestamp}.json
└── logs/                # 操作日志
```

### 5.2 默认配置内容

```toml
# Aide 默认配置（由 aide init 生成）
# runtime: aide 自身运行要求
# task: 任务文档路径
# env: 虚拟环境与依赖配置
# flow: 环节名称列表，供流程校验使用

[runtime]
python_min = "3.11"
use_uv = true

[task]
source = "task-now.md"
spec = "task-spec.md"

[env]
venv = ".venv"
requirements = "requirements.txt"

[flow]
phases = ["task-optimize", "flow-design", "impl", "verify", "docs", "finish"]
```

### 5.3 方法签名原型

```
class ConfigManager:
    root: Path
    aide_dir: Path            # .aide/
    config_path: Path         # .aide/config.toml
    decisions_dir: Path       # .aide/decisions/
    logs_dir: Path            # .aide/logs/

    ensure_base_dirs() -> None
        # 创建基础目录结构

    ensure_gitignore() -> None
        # 确保 .gitignore 包含 .aide/

    ensure_config() -> dict
        # 确保配置文件存在，返回配置内容

    load_config() -> dict
        # 加载配置文件

    get_value(key: str) -> Any
        # 获取配置值（点号分隔的键）

    set_value(key: str, value: Any) -> None
        # 设置配置值
```

---

## 六、依赖

| 依赖项 | 类型 | 说明 |
|--------|------|------|
| output | 内部模块 | 输出格式化 |
| tomllib | 标准库 | TOML 读取 |
| tomli_w | 第三方库 | TOML 写入 |

---

## 七、被依赖

| 依赖方 | 说明 |
|--------|------|
| /aide:init | 调用 aide init 初始化配置 |
| aide env ensure | 依赖配置文件存在 |
| aide flow | 依赖目录结构存在 |
| aide decide | 依赖 decisions/ 目录存在 |

---

## 八、修改指南

### 8.1 修改默认配置

1. 更新本文档的"默认配置内容"章节
2. 修改 `ConfigManager` 中的 `DEFAULT_CONFIG`
3. 同步更新 [配置格式文档](../formats/config.md)

### 8.2 修改目录结构

1. 更新本文档的"目录结构"章节
2. 修改 `ensure_base_dirs()` 方法
3. 同步更新相关文档

### 8.3 添加新的初始化步骤

1. 在本文档添加步骤说明
2. 在 `ensure_config()` 或新方法中实现
3. 更新业务流程图

---

## 九、相关文档

- [program 导览](../README.md)
- [配置格式文档](../formats/config.md)
- [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)
- [/aide:init 命令设计](../../../aide-marketplace/aide-plugin/docs/commands/init.md)
