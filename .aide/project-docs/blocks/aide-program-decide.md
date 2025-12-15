# aide-program-decide

> 路径：aide-program/aide/decide/
> 最后更新：2025-12-15

## 概述

待定项确认模块，提供 Web 界面让用户对任务中的待定项进行决策。服务在后台运行，用户提交后自动关闭。

## 文件清单

| 文件 | 说明 |
|------|------|
| `__init__.py` | 模块初始化，导出 cmd 函数 |
| `cli.py` | CLI 入口（~134 行） |
| `server.py` | HTTP 服务器管理（~280 行） |
| `daemon.py` | 后台进程启动器（~30 行） |
| `handlers.py` | HTTP 请求处理器（~170 行） |
| `storage.py` | 数据存储（~155 行） |
| `types.py` | 数据结构定义（~300 行） |
| `errors.py` | 自定义异常 |
| `web/` | Web 前端资源 |

## 核心组件

### CLI 入口

- **职责**：解析参数并调度功能
- **位置**：`cli.py`
- **关键函数**：
  - `cmd_decide_submit(file_path)` - 提交待定项数据
  - `cmd_decide_result()` - 获取决策结果

### DecideServer

- **职责**：HTTP 服务器生命周期管理
- **位置**：`server.py:26`
- **关键方法**：
  - `start()` - 前台启动服务
  - `start_daemon(pid)` - 后台启动服务
  - `stop(reason)` - 停止服务

### DecideStorage

- **职责**：管理待定项和决策结果的存储
- **位置**：`storage.py`
- **关键方法**：
  - `save_pending(input)` - 保存待定项到 pending.json
  - `load_pending()` - 加载待定项
  - `save_result(result)` - 保存决策结果
  - `load_result()` - 加载决策结果
  - `is_server_running()` - 检查服务是否运行

### 数据结构

- `DecideInput` - 待定项输入数据
- `DecideItem` - 单个待定项（含多个选项）
- `DecideOption` - 选项（value, label, score, pros, cons）
- `DecideResult` - 决策结果
- `DecidedItem` - 单个决策（chosen, note）

## 接口说明

```python
# CLI 入口
aide decide submit <file>  # 提交待定项，启动后台服务
aide decide result         # 获取决策结果

# Web 界面
GET /                      # 主页面
GET /api/data              # 获取待定项数据
POST /api/submit           # 提交决策
```

## 配置项

在 `.aide/config.toml` 的 `[decide]` 节：

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `port` | 3721 | 起始端口 |
| `bind` | 127.0.0.1 | 监听地址 |
| `url` | "" | 自定义访问地址 |
| `timeout` | 0 | 超时时间（秒） |

## 数据文件

- `.aide/decisions/pending.json` - 当前待定项数据
- `.aide/decisions/{timestamp}.json` - 决策记录归档

## 依赖关系

- 依赖：core（output, config）
- 被依赖：main.py

## 注意事项

- 服务作为后台进程运行，与 CLI 脱离
- 用户提交决策后服务自动关闭
- 如果 recommend 字段存在，对应选项默认选中
