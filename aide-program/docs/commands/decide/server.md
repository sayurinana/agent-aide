# aide decide HTTP 服务设计

## 一、概述

aide decide 使用内置 HTTP 服务器提供 Web 界面，供用户确认待定项。本文档定义服务器的设计规格。

### 1.1 技术选型

- 使用 Python 标准库 `http.server`
- 无需额外依赖
- 单线程阻塞模式（简化实现，满足单用户场景）

### 1.2 服务职责

| 职责 | 说明 |
|------|------|
| 静态资源服务 | 提供 HTML/CSS/JS 文件 |
| API 服务 | 提供数据获取和提交接口 |
| 生命周期管理 | 启动、等待、关闭 |

## 二、服务生命周期

### 2.1 状态机

```
@startuml
skinparam defaultFontName "PingFang SC"

[*] --> 初始化 : 调用 start()
初始化 --> 端口探测 : 配置加载完成
端口探测 --> 启动失败 : 无可用端口
端口探测 --> 运行中 : 找到可用端口
启动失败 --> [*]
运行中 --> 关闭中 : 收到提交/超时/中断
关闭中 --> 已关闭
已关闭 --> [*]

@enduml
```

### 2.2 启动流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:加载配置;
note right: decide.port, decide.timeout

:确定起始端口;
if (配置了固定端口?) then (是)
  :使用配置端口;
else (否)
  :使用默认端口 3721;
endif

:端口探测循环;
repeat
  :尝试绑定端口;
  if (绑定成功?) then (是)
    :记录实际端口;
    break
  else (否)
    :端口 += 1;
  endif
repeat while (尝试次数 < 10?)

if (找到可用端口?) then (是)
else (否)
  :返回错误;
  stop
endif

:创建 HTTP 服务器;

:注册请求处理器;

:输出访问链接;

:进入服务循环;

stop
@enduml
```

### 2.3 关闭流程

服务在以下情况下关闭：

| 触发条件 | 处理方式 |
|----------|----------|
| 用户提交决策 | 正常关闭，返回成功 |
| 超时（若配置） | 正常关闭，返回警告 |
| 键盘中断（Ctrl+C） | 正常关闭，返回中断 |
| 异常错误 | 异常关闭，返回错误 |

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:收到关闭信号;

:停止接受新请求;

:等待当前请求完成;
note right: 最多等待 5 秒

:关闭 socket;

:清理资源;

:返回关闭原因;

stop
@enduml
```

## 三、端口配置

### 3.1 端口探测策略

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `decide.port` | 3721 | 起始端口 |
| 最大尝试次数 | 10 | 从起始端口开始尝试 |

**探测逻辑**：

1. 从 `decide.port` 开始
2. 尝试绑定端口
3. 若失败，尝试下一个端口
4. 最多尝试 10 次
5. 全部失败则返回错误

### 3.2 端口占用检测

```
check_port_available(port: int) -> bool:
    """
    检查端口是否可用

    1. 创建 socket
    2. 尝试绑定到 127.0.0.1:port
    3. 成功则端口可用，关闭 socket 返回 True
    4. 失败则端口被占用，返回 False
    """
```

## 四、API 端点设计

### 4.1 端点一览

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/` | 返回主页面（index.html） |
| GET | `/style.css` | 返回样式文件 |
| GET | `/app.js` | 返回脚本文件 |
| GET | `/api/items` | 获取待定项数据 |
| POST | `/api/submit` | 提交决策结果 |

### 4.2 GET /api/items

**请求**：无参数

**响应**：

成功（200）：
```json
{
  "task": "实现用户认证模块",
  "source": "task-now.md",
  "items": [
    {
      "id": 1,
      "title": "认证方式选择",
      "context": "任务描述中未明确指定认证方式",
      "options": [
        {"value": "jwt", "label": "JWT Token 认证", "score": 85, "pros": [...], "cons": [...]},
        {"value": "session", "label": "Session 认证", "score": 70, "pros": [...], "cons": [...]}
      ],
      "recommend": "jwt"
    }
  ]
}
```

失败（500）：
```json
{
  "error": "无法读取待定项数据",
  "detail": "文件不存在或格式错误"
}
```

### 4.3 POST /api/submit

**请求**：

Content-Type: `application/json`

```json
{
  "decisions": [
    {"id": 1, "chosen": "jwt"},
    {"id": 2, "chosen": "bcrypt", "note": "团队更熟悉 bcrypt"}
  ]
}
```

**响应**：

成功（200）：
```json
{
  "success": true,
  "message": "决策已保存"
}
```

校验失败（400）：
```json
{
  "error": "决策数据无效",
  "detail": "缺少待定项 2 的决策"
}
```

服务器错误（500）：
```json
{
  "error": "保存失败",
  "detail": "无法写入文件"
}
```

**提交后行为**：

1. 验证决策数据完整性（所有待定项都有决策）
2. 保存决策结果到历史记录
3. 设置关闭标志
4. 返回成功响应
5. 服务器在响应发送后关闭

### 4.4 静态资源服务

| 路径 | 文件 | Content-Type |
|------|------|--------------|
| `/` | `web/index.html` | `text/html; charset=utf-8` |
| `/style.css` | `web/style.css` | `text/css; charset=utf-8` |
| `/app.js` | `web/app.js` | `application/javascript; charset=utf-8` |

**资源加载方式**：

- 方案A：从文件系统读取（开发时便于修改）
- 方案B：嵌入 Python 代码（部署时无需额外文件）

建议：使用方案A，资源文件放在 `aide/decide/web/` 目录下。

## 五、请求处理

### 5.1 请求处理流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:接收 HTTP 请求;

:解析请求路径和方法;

switch (路径)
case (/)
  :返回 index.html;
case (/style.css)
  :返回 style.css;
case (/app.js)
  :返回 app.js;
case (/api/items)
  if (方法 == GET?) then (是)
    :读取 pending.json;
    :返回 JSON 数据;
  else (否)
    :返回 405 Method Not Allowed;
  endif
case (/api/submit)
  if (方法 == POST?) then (是)
    :解析请求体;
    :验证决策数据;
    :保存决策结果;
    :设置关闭标志;
    :返回成功响应;
  else (否)
    :返回 405 Method Not Allowed;
  endif
case (其他)
  :返回 404 Not Found;
endswitch

stop
@enduml
```

### 5.2 CORS 处理

由于前端和后端在同一服务器，通常不需要 CORS。但为了开发调试方便，建议添加以下响应头：

```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

### 5.3 错误处理

| HTTP 状态码 | 场景 |
|-------------|------|
| 200 | 请求成功 |
| 400 | 请求数据无效 |
| 404 | 路径不存在 |
| 405 | 方法不允许 |
| 500 | 服务器内部错误 |

## 六、超时处理

### 6.1 配置项

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `decide.timeout` | 0 | 超时时间（秒），0 表示无超时 |

### 6.2 超时实现

```
@startuml
skinparam defaultFontName "PingFang SC"

start

fork
  :服务循环;
  :处理请求;
fork again
  :超时计时器;
  :等待 timeout 秒;
  :设置超时标志;
end fork

if (超时?) then (是)
  :关闭服务;
  :输出警告;
else (否)
  :正常关闭;
endif

stop
@enduml
```

**注意**：由于使用单线程模型，超时检测需要在请求处理间隙进行，或使用 `select` 实现非阻塞等待。

## 七、方法签名原型

```
class DecideServer:
    """HTTP 服务器"""

    root: Path                    # 项目根目录
    port: int                     # 实际使用的端口
    timeout: int                  # 超时时间（秒）
    pending_path: Path            # pending.json 路径
    web_dir: Path                 # 静态资源目录
    should_close: bool            # 关闭标志
    close_reason: str             # 关闭原因

    def __init__(self, root: Path) -> None:
        """初始化服务器"""

    def start(self) -> bool:
        """
        启动服务器

        1. 加载配置
        2. 探测可用端口
        3. 创建 HTTP 服务器
        4. 输出访问链接
        5. 进入服务循环
        6. 返回是否成功完成
        """

    def stop(self, reason: str) -> None:
        """
        停止服务器

        1. 设置关闭标志和原因
        2. 关闭 socket
        """

    def _find_available_port(self) -> int | None:
        """
        探测可用端口

        从配置端口开始，最多尝试 10 次
        返回可用端口或 None
        """

    def _serve_forever(self) -> None:
        """
        服务循环

        处理请求直到 should_close 为 True
        """


class DecideHandler:
    """请求处理器"""

    server: DecideServer          # 服务器引用

    def handle_request(self, method: str, path: str, body: bytes) -> Response:
        """
        处理请求

        根据 method 和 path 分发到具体处理函数
        """

    def handle_index(self) -> Response:
        """返回主页面"""

    def handle_static(self, filename: str) -> Response:
        """返回静态资源"""

    def handle_get_items(self) -> Response:
        """处理 GET /api/items"""

    def handle_submit(self, body: bytes) -> Response:
        """处理 POST /api/submit"""


Response = tuple[int, dict[str, str], bytes]
# (状态码, 响应头, 响应体)
```

## 八、安全考虑

### 8.1 绑定地址

- 默认绑定到 `127.0.0.1`（仅本地访问）
- 不绑定到 `0.0.0.0`（避免外部访问）

### 8.2 输入验证

- 验证 JSON 格式
- 验证决策数据完整性
- 限制请求体大小（建议 1MB）

### 8.3 路径遍历防护

- 静态资源只从 `web/` 目录提供
- 不允许 `..` 路径

## 九、相关文档

- [decide 详细设计入口](README.md)
- [CLI 规格](cli.md)
- [Web 前端设计](web.md)
- [数据存储设计](storage.md)
