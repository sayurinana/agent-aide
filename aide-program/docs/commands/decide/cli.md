# aide decide CLI 规格

## 一、命令一览

`aide decide` 提供两个子命令：

| 子命令 | 语法（API 约定） | 成功输出 | 主要用途 |
|--------|------------------|----------|----------|
| (默认) | `aide decide '<json>'` | 输出访问链接，阻塞等待 | 提交待定项数据并启动 Web 服务 |
| result | `aide decide result` | 输出 JSON 结果 | 获取用户决策结果 |

## 二、命令详细规格

### 2.1 aide decide（提交数据并启动服务）

**语法**：

```
aide decide '<json_data>'
```

**参数**：

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `<json_data>` | string | 是 | 待定项 JSON 数据，需用引号包裹 |

**输入数据格式**：

见 `aide-program/docs/formats/data.md` 的"待定项数据格式"章节。

**成功输出**：

```
→ Web 服务已启动
→ 请访问: http://localhost:3721
→ 等待用户完成决策...
```

服务关闭后：

```
✓ 决策已完成
```

**错误输出**：

```
✗ JSON 解析失败: <具体错误>
  建议: 检查 JSON 格式是否正确
```

```
✗ 数据验证失败: <具体错误>
  建议: 检查必填字段是否完整
```

```
✗ 无法启动服务: 端口 3721-3730 均被占用
  建议: 关闭占用端口的程序，或在配置中指定其他端口
```

**行为流程**：

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:接收 JSON 参数;

:解析 JSON;
if (解析成功?) then (是)
else (否)
  :输出错误: JSON 解析失败;
  stop
endif

:验证数据格式;
if (验证通过?) then (是)
else (否)
  :输出错误: 数据验证失败;
  stop
endif

:保存到 pending.json;

:探测可用端口;
if (找到可用端口?) then (是)
else (否)
  :输出错误: 端口均被占用;
  stop
endif

:启动 HTTP 服务;

:输出访问链接;

:阻塞等待用户操作;

:用户提交决策;

:保存决策结果;

:关闭服务;

:输出: 决策已完成;

stop
@enduml
```

### 2.2 aide decide result（获取决策结果）

**语法**：

```
aide decide result
```

**参数**：无

**成功输出**：

直接输出 JSON 格式的决策结果（便于 LLM 解析）：

```json
{
  "decisions": [
    {"id": 1, "chosen": "option_a"},
    {"id": 2, "chosen": "option_b", "note": "用户的补充说明"}
  ]
}
```

**错误输出**：

```
✗ 尚无决策结果
  建议: 请等待用户在 Web 界面完成操作
```

```
✗ 未找到待定项数据
  建议: 请先执行 aide decide '<json>'
```

```
✗ 决策结果已过期
  建议: 请重新执行 aide decide '<json>'
```

**行为流程**：

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:检查 pending.json 是否存在;
if (存在?) then (是)
else (否)
  :输出错误: 未找到待定项数据;
  stop
endif

:查找最新决策记录;
if (存在决策记录?) then (是)
else (否)
  :输出错误: 尚无决策结果;
  stop
endif

:验证决策记录与 pending 匹配;
if (匹配?) then (是)
else (否)
  :输出错误: 决策结果已过期;
  stop
endif

:输出 JSON 结果;

stop
@enduml
```

## 三、参数校验规则

### 3.1 JSON 数据校验

**必填字段**：

| 字段 | 类型 | 校验规则 |
|------|------|----------|
| `task` | string | 非空字符串 |
| `source` | string | 非空字符串 |
| `items` | array | 非空数组，至少包含 1 个待定项 |

**待定项（DecideItem）校验**：

| 字段 | 类型 | 校验规则 |
|------|------|----------|
| `id` | number | 正整数，在 items 中唯一 |
| `title` | string | 非空字符串 |
| `options` | array | 非空数组，至少包含 2 个选项 |

**选项（Option）校验**：

| 字段 | 类型 | 校验规则 |
|------|------|----------|
| `value` | string | 非空字符串，在同一 item 的 options 中唯一 |
| `label` | string | 非空字符串 |

**可选字段**：

| 字段 | 类型 | 默认值 |
|------|------|--------|
| `location` | object | null |
| `context` | string | null |
| `recommend` | string | null（若提供，必须是 options 中某个 value） |
| `score` | number | null（若提供，范围 0-100） |
| `pros` | array | null |
| `cons` | array | null |

### 3.2 校验错误信息

校验失败时，错误信息应明确指出：

1. 哪个字段出错
2. 期望的格式/值
3. 实际收到的值（如适用）

**示例**：

```
✗ 数据验证失败: items[0].options 至少需要 2 个选项，当前只有 1 个
  建议: 为每个待定项提供至少 2 个可选方案
```

```
✗ 数据验证失败: items[1].recommend 值 "invalid" 不在 options 中
  建议: recommend 必须是 options 中某个选项的 value
```

## 四、输出规范

### 4.1 静默原则

- 成功时输出必要的状态信息（访问链接、完成提示）
- 错误时输出详细的错误信息和建议
- `aide decide result` 成功时仅输出 JSON，便于程序解析

### 4.2 固定前缀

沿用 `aide-program/docs/README.md` 的输出规范：

| 前缀 | 函数 | 用途 |
|------|------|------|
| `✓` | `output.ok()` | 成功 |
| `✗` | `output.err()` | 失败 |
| `→` | `output.info()` | 进行中/信息 |

### 4.3 JSON 输出格式

`aide decide result` 的 JSON 输出：

- 使用 UTF-8 编码
- 紧凑格式（无缩进），便于程序解析
- 输出到 stdout，错误信息输出到 stderr

## 五、退出码

| 退出码 | 含义 |
|-------:|------|
| 0 | 成功 |
| 1 | 失败（参数错误、校验失败、服务启动失败等） |

## 六、典型调用序列

### 6.1 正常流程

```bash
# 1. LLM 提交待定项数据
$ aide decide '{"task":"实现用户认证","source":"task.md","items":[...]}'
→ Web 服务已启动
→ 请访问: http://localhost:3721
→ 等待用户完成决策...
✓ 决策已完成

# 2. LLM 获取决策结果
$ aide decide result
{"decisions":[{"id":1,"chosen":"jwt"},{"id":2,"chosen":"bcrypt"}]}
```

### 6.2 服务超时

```bash
# 配置了超时时间的情况
$ aide decide '{"task":"...","source":"...","items":[...]}'
→ Web 服务已启动
→ 请访问: http://localhost:3721
→ 等待用户完成决策...
⚠ 服务超时，已自动关闭

$ aide decide result
✗ 尚无决策结果
  建议: 请等待用户在 Web 界面完成操作
```

### 6.3 与 /aide:prep 的集成

```
/aide:prep 流程中：
1. LLM 分析任务，识别待定项
2. LLM 构造 JSON 数据
3. LLM 调用 aide decide '<json>'
4. LLM 告知用户访问链接
5. 用户在 Web 界面完成决策
6. LLM 调用 aide decide result 获取结果
7. LLM 根据决策结果继续任务
```

## 七、方法签名原型

```
# CLI 入口（在 main.py 中）
def cmd_decide(args: list[str]) -> int:
    """
    处理 aide decide 命令

    args[0] 为 JSON 数据或 "result"
    返回退出码
    """

def cmd_decide_submit(json_data: str) -> int:
    """
    提交待定项数据并启动服务

    1. 解析并验证 JSON
    2. 保存到 pending.json
    3. 启动 HTTP 服务
    4. 等待用户操作
    5. 返回退出码
    """

def cmd_decide_result() -> int:
    """
    获取决策结果

    1. 检查 pending.json
    2. 查找最新决策记录
    3. 验证匹配性
    4. 输出 JSON 结果
    5. 返回退出码
    """
```

## 八、相关文档

- [decide 详细设计入口](README.md)
- [HTTP 服务设计](server.md)
- [数据存储设计](storage.md)
- [数据格式文档](../../formats/data.md)
