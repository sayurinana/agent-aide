# aide-program-decide

> 路径：aide-program/aide/decide/
> 最后更新：2025-12-17

## 概述

待定项确认模块，提供 Web 界面让用户对任务中的待定项进行决策。采用后台服务模式，支持 JSON 数据输入、Web UI 交互和决策结果输出。

## 目录结构

```
aide-program/aide/decide/
├── __init__.py              模块入口
├── types.py                 数据结构与校验
├── errors.py                错误类型
├── cli.py                   CLI 入口
├── storage.py               数据读写与管理
├── daemon.py                后台服务入口点
├── server.py                HTTP 服务器管理
├── handlers.py              HTTP 请求处理
└── web/
    ├── index.html           Web 界面 HTML
    ├── style.css            CSS 样式
    └── app.js               前端 JavaScript
```

## 文件清单

| 文件 | 类型 | 说明 |
|------|------|------|
| __init__.py | 源码 | 模块入口，导出 CLI 函数 |
| types.py | 源码 | 数据类定义（DecideInput、DecideOutput 等） |
| errors.py | 源码 | DecideError 异常类 |
| cli.py | 源码 | CLI 命令处理（submit、result） |
| storage.py | 源码 | DecideStorage 类，数据读写与归档 |
| daemon.py | 源码 | 后台服务入口点，供 subprocess 启动 |
| server.py | 源码 | DecideServer 类，HTTP 服务生命周期 |
| handlers.py | 源码 | DecideHandlers 类，HTTP 路由和处理 |
| web/index.html | HTML | Web 界面结构 |
| web/style.css | CSS | 响应式样式定义 |
| web/app.js | JavaScript | 前端交互逻辑 |

## 核心组件

### DecideStorage 类

- **职责**：管理 pending.json、决策记录和服务状态
- **位置**：`aide/decide/storage.py:16`
- **关键方法**：
  - `save_pending(data)` - 保存待定项数据并生成 session_id
  - `load_pending()` - 读取待定项
  - `save_result(output)` - 保存决策结果为历史记录
  - `load_result()` - 读取当前会话的决策结果
  - `save_server_info()` - 保存服务状态信息
  - `is_server_running()` - 检查服务是否运行中

### DecideServer 类

- **职责**：HTTP 服务器生命周期管理
- **位置**：`aide/decide/server.py:26`
- **关键方法**：
  - `start()` - 前台启动服务
  - `start_daemon(pid)` - 后台启动服务
  - `stop(reason)` - 停止服务
  - `_find_available_port()` - 自动探测可用端口
  - `_serve_forever()` - 服务循环

### DecideHandlers 类

- **职责**：HTTP 请求路由和处理
- **位置**：`aide/decide/handlers.py:17`
- **关键方法**：
  - `handle(method, path, body)` - 请求分发
  - `handle_get_items()` - GET /api/items
  - `handle_submit(body)` - POST /api/submit
  - `_validate_decisions()` - 验证决策数据完整性

### DecideInput 数据类

- **职责**：待定项输入数据封装
- **位置**：`aide/decide/types.py:151`
- **字段**：
  - `task: str` - 任务名称
  - `source: str` - 来源文件
  - `items: list[DecideItem]` - 待定项列表
  - `meta: MetaInfo | None` - 元信息（session_id、created_at）

### DecideItem 数据类

- **职责**：单个待定项封装
- **位置**：`aide/decide/types.py:66`
- **字段**：
  - `id: int` - 项目 ID
  - `title: str` - 标题
  - `options: list[Option]` - 选项列表
  - `location: Location | None` - 位置信息
  - `context: str | None` - 上下文
  - `recommend: str | None` - 推荐选项值

### Option 数据类

- **职责**：选项封装
- **位置**：`aide/decide/types.py:31`
- **字段**：
  - `value: str` - 选项值
  - `label: str` - 显示标签
  - `score: float | None` - 评分（0-100）
  - `pros: list[str] | None` - 优点列表
  - `cons: list[str] | None` - 缺点列表

## API 接口

### REST API

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | /api/items | 获取待定项数据 |
| POST | /api/submit | 提交决策结果 |
| GET | / | Web 界面 HTML |
| GET | /style.css | CSS 样式 |
| GET | /app.js | JavaScript |

### 数据格式

**输入格式（submit 命令）：**
```json
{
  "task": "任务名称",
  "source": "task-now.md",
  "items": [
    {
      "id": 1,
      "title": "问题标题",
      "options": [
        {"value": "a", "label": "选项A", "score": 85, "pros": ["优点1"], "cons": ["缺点1"]},
        {"value": "b", "label": "选项B"}
      ],
      "recommend": "a",
      "context": "问题背景",
      "location": {"file": "task.md", "start": 5, "end": 7}
    }
  ]
}
```

**输出格式（result 命令）：**
```json
{
  "decisions": [
    {"id": 1, "chosen": "a", "note": "用户备注"}
  ]
}
```

## 接口说明

### CLI 使用

```bash
# 提交待定项，启动 Web 服务
aide decide submit ./pending-items.json

# 获取决策结果
aide decide result
```

### 配置项

在 `[decide]` 节：
- `port` - 起始端口（默认 3721）
- `bind` - 监听地址（默认 127.0.0.1）
- `url` - 自定义访问地址
- `timeout` - 超时时间（秒，0 表示不超时）

## 依赖关系

- 依赖：aide/core（ConfigManager、output）、aide/flow/utils（now_task_id）
- 被依赖：aide/main.py

## 注意事项

- 服务以后台进程运行，用户提交后自动关闭
- 决策记录保存在 .aide/decisions/{session_id}.json
- Web 界面支持响应式设计，移动端友好
- 支持推荐选项默认选中
- 选项的 value 在同一待定项中必须唯一
