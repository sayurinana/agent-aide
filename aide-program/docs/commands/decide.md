# aide decide 子命令设计文档

## 零、详细设计文档包

本文档为概览设计；更细的实现交接规格见：

- [aide-program/docs/commands/decide/README.md](decide/README.md)
- [aide-program/docs/commands/decide/cli.md](decide/cli.md)
- [aide-program/docs/commands/decide/server.md](decide/server.md)
- [aide-program/docs/commands/decide/web.md](decide/web.md)
- [aide-program/docs/commands/decide/storage.md](decide/storage.md)
- [aide-program/docs/commands/decide/verification.md](decide/verification.md)

## 一、背景

### 1.1 解决的问题

| 问题 | 影响 |
|------|------|
| 待定项呈现繁琐 | LLM 输出大量文本描述选项 |
| 用户确认不便 | 在终端中逐项确认效率低 |
| 决策记录分散 | 难以追溯历史决策 |

### 1.2 设计目标

提供**程序化的待定项确认机制**：
- LLM 传入精简 JSON 数据
- 程序启动 Web 服务，提供可视化界面
- 用户在 Web 界面操作
- LLM 读取精简决策结果

---

## 二、职责

### 2.1 做什么

1. 接收待定项 JSON 数据
2. 启动 HTTP 服务
3. 提供 Web 界面供用户操作
4. 存储用户决策
5. 返回决策结果

### 2.2 不做什么

- 不分析待定项内容
- 不做决策建议
- 不修改业务代码

---

## 三、接口规格

### 3.1 命令一览

```
aide decide {submit,result} ...

子命令:
  submit <json>  提交待定项数据并启动 Web 服务
  result         获取用户决策结果
```

### 3.2 aide decide submit（提交数据）

**用途**：提交待定项数据并启动 Web 服务

**语法**：
```
aide decide submit '<json数据>'
```

**输入**：待定项 JSON 数据（见数据格式章节）

**输出**：
```
→ Web 服务已启动
→ 请访问: http://localhost:3721
→ 等待用户完成决策...
✓ 决策已完成
```

**配置项**（见 [配置格式文档](../formats/config.md)）：

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `decide.port` | 3721 | 起始端口 |
| `decide.bind` | `"127.0.0.1"` | 监听地址 |
| `decide.url` | `""` | 自定义访问地址 |
| `decide.timeout` | 0 | 超时时间（秒） |

### 3.3 aide decide result

**用途**：获取用户决策结果

**语法**：
```
aide decide result
```

**输出**：
```json
{
  "decisions": [
    {"id": 1, "chosen": "option_a"},
    {"id": 2, "chosen": "option_b", "note": "用户的补充说明"}
  ]
}
```

**错误情况**：
```
✗ 尚无决策结果
  建议: 请等待用户在 Web 界面完成操作
```

```
✗ 未找到待定项数据
  建议: 请先执行 aide decide submit '<json>'
```

---

## 四、业务流程

### 4.1 整体流程

```
@startuml
skinparam defaultFontName "PingFang SC"

participant LLM
participant "aide decide" as Decide
participant "Web Server" as Web
participant User

LLM -> Decide : aide decide submit '<json>'
Decide -> Decide : 解析 JSON
Decide -> Decide : 保存待定项数据
Decide -> Web : 启动 HTTP 服务
Decide --> LLM : 输出访问链接

LLM -> User : 告知访问链接

User -> Web : 访问页面
Web -> User : 渲染待定项界面
User -> Web : 选择选项、添加备注
User -> Web : 提交决策
Web -> Web : 保存决策结果

LLM -> Decide : aide decide result
Decide -> Decide : 读取决策结果
Decide --> LLM : 返回 JSON 结果

@enduml
```

### 4.2 Web 服务流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:接收 JSON 数据;

:解析并验证格式;
if (格式有效?) then (是)
else (否)
  :输出错误信息;
  stop
endif

:保存到 .aide/decisions/pending.json;

:启动 HTTP 服务;
note right: 默认端口 3721

:输出访问链接;

:等待用户操作;

:用户提交决策;

:保存决策结果;
note right: .aide/decisions/{timestamp}.json

:关闭 HTTP 服务;

stop
@enduml
```

---

## 五、数据结构

### 5.1 输入格式（LLM → 程序）

```
DecideInput:
    task: str                    # 任务简述
    source: str                  # 来源文档
    items: list[DecideItem]      # 待定项列表

DecideItem:
    id: int                      # 待定项 ID
    title: str                   # 问题标题
    location: Location | None    # 原文位置（可选）
    context: str | None          # 详细说明（可选）
    options: list[Option]        # 选项列表
    recommend: str | None        # 推荐选项的 value（可选）

Location:
    file: str                    # 文件路径
    start: int                   # 起始行
    end: int                     # 结束行

Option:
    value: str                   # 选项标识
    label: str                   # 选项描述
    score: int | None            # 评分（可选）
    pros: list[str] | None       # 优点列表（可选）
    cons: list[str] | None       # 缺点列表（可选）
```

### 5.2 输出格式（程序 → LLM）

```
DecideOutput:
    decisions: list[Decision]    # 决策列表

Decision:
    id: int                      # 待定项 ID
    chosen: str                  # 选中的选项 value
    note: str | None             # 用户备注（可选）
```

### 5.3 方法签名原型

```
class DecideServer:
    root: Path
    port: int                    # 默认 3721
    pending_path: Path           # .aide/decisions/pending.json
    decisions_dir: Path          # .aide/decisions/

    submit(json_data: str) -> bool
        # 提交待定项数据，启动服务

    get_result() -> DecideOutput | None
        # 获取决策结果

    _parse_input(json_data: str) -> DecideInput
        # 解析输入 JSON

    _validate_input(data: DecideInput) -> bool
        # 验证输入格式

    _start_server() -> None
        # 启动 HTTP 服务

    _stop_server() -> None
        # 停止 HTTP 服务

    _save_pending(data: DecideInput) -> None
        # 保存待定项数据

    _save_result(result: DecideOutput) -> None
        # 保存决策结果

    _load_result() -> DecideOutput | None
        # 加载决策结果
```

### 5.4 Web 界面组件

```
DecideApp:
    # React 前端应用

    state:
        items: list[DecideItem]      # 待定项列表
        decisions: dict[int, str]    # 当前选择
        notes: dict[int, str]        # 用户备注

    methods:
        loadItems() -> None          # 加载待定项
        selectOption(id, value) -> None  # 选择选项
        addNote(id, note) -> None    # 添加备注
        submit() -> None             # 提交决策
```

---

## 六、Web 界面设计

### 6.1 页面结构

```
┌─────────────────────────────────────────────────┐
│  Aide 待定项确认                                 │
│  任务: <task>                                   │
├─────────────────────────────────────────────────┤
│                                                  │
│  ┌─────────────────────────────────────────┐    │
│  │ 1. <title>                               │    │
│  │    <context>                             │    │
│  │                                          │    │
│  │    ○ <option_a.label>  [推荐]            │    │
│  │      优点: ...  缺点: ...                │    │
│  │                                          │    │
│  │    ○ <option_b.label>                    │    │
│  │      优点: ...  缺点: ...                │    │
│  │                                          │    │
│  │    备注: [________________]              │    │
│  └─────────────────────────────────────────┘    │
│                                                  │
│  ┌─────────────────────────────────────────┐    │
│  │ 2. <title>                               │    │
│  │    ...                                   │    │
│  └─────────────────────────────────────────┘    │
│                                                  │
│                              [提交决策]          │
└─────────────────────────────────────────────────┘
```

### 6.2 交互流程

1. 页面加载时从后端获取待定项数据
2. 用户点击选项进行选择
3. 用户可选择性添加备注
4. 点击"提交决策"按钮
5. 前端发送决策到后端
6. 后端保存结果并关闭服务
7. 页面显示"决策已提交"

---

## 七、依赖

| 依赖项 | 类型 | 说明 |
|--------|------|------|
| output | 内部模块 | 输出格式化 |
| http.server | 标准库 | HTTP 服务 |
| json | 标准库 | JSON 解析 |

---

## 八、被依赖

| 依赖方 | 说明 |
|--------|------|
| /aide:prep | 调用 decide 处理待定项 |

---

## 九、修改指南

### 9.1 修改输入/输出格式

1. 更新本文档的数据结构章节
2. 修改代码实现
3. 同步更新 [数据格式文档](../formats/data.md)
4. 同步更新 [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)

### 9.2 修改 Web 界面

1. 更新本文档的界面设计章节
2. 修改 `aide/decide/web/` 下的前端代码

### 9.3 修改端口配置

1. 在配置文件中添加端口配置项
2. 修改 `DecideServer` 读取配置
3. 更新 [配置格式文档](../formats/config.md)

---

## 十、相关文档

- [program 导览](../README.md)
- [数据格式文档](../formats/data.md)
- [aide skill 设计文档](../../../aide-marketplace/aide-plugin/docs/skill/aide.md)
- [/aide:prep 命令设计](../../../aide-marketplace/aide-plugin/docs/commands/prep.md)
