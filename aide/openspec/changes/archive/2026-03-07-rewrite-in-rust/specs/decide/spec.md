## ADDED Requirements

### Requirement: 待定项数据结构

系统 SHALL 使用以下 JSON 数据结构：

**DecideInput（提交格式）**：
```
{
  task: string,                  // 任务描述（非空）
  source: string,                // 来源文档路径（非空）
  items: DecideItem[],           // 至少 1 项
  _meta?: { created_at, session_id }  // 系统自动添加
}
```

**DecideItem**：
```
{
  id: number,                    // 正整数，items 内唯一
  title: string,                 // 标题（非空）
  options: Option[],             // 至少 2 个选项
  location?: { file, start, end },  // 源码位置
  context?: string,              // 详细说明
  recommend?: string             // 推荐选项的 value
}
```

**Option**：
```
{
  value: string,                 // 选项标识（item 内唯一）
  label: string,                 // 显示文本
  score?: number,                // 0-100 评分
  pros?: string[],               // 优势列表
  cons?: string[]                // 劣势列表
}
```

**DecideOutput（结果格式）**：
```
{
  decisions: Decision[]          // 数量必须与 items 一致
}
```

**Decision**：
```
{
  id: number,                    // 必须匹配某个 item 的 id
  chosen: string,                // 必须匹配对应 item 的某个 option value
  note?: string                  // 可选用户备注
}
```

#### Scenario: 有效的输入数据
- **WHEN** 提交的 JSON 包含 task、source 和至少一个 item
- **AND** 每个 item 有唯一 id、非空 title 和至少 2 个选项
- **THEN** 验证通过

#### Scenario: 无效的输入数据
- **WHEN** 提交的 JSON 中 items 为空数组
- **THEN** 验证失败并输出错误信息

### Requirement: 输入验证规则

系统 SHALL 对 DecideInput 执行以下验证：
- `task`：非空字符串
- `source`：非空字符串
- `items`：至少 1 个元素
- 每个 item 的 `id`：正整数，items 内唯一
- 每个 item 的 `title`：非空字符串
- 每个 item 的 `options`：至少 2 个元素
- 每个 option 的 `value`：item 内唯一
- `recommend`（如存在）：必须匹配某个 option 的 value
- `location`（如存在）：file、start、end 均必填
- `score`（如存在）：0-100 范围

对 DecideOutput 的验证：
- `decisions` 数量必须与 items 一致
- 每个 decision 的 `id` 必须匹配一个 item
- 每个 decision 的 `chosen` 必须匹配对应 item 的某个 option value
- 不允许重复的 decision id

#### Scenario: recommend 不匹配 option
- **WHEN** item 的 recommend 值不在 options 的 value 列表中
- **THEN** 验证失败

#### Scenario: 输出验证通过
- **WHEN** decisions 数量与 items 一致
- **AND** 每个 decision 的 id 和 chosen 均有效
- **THEN** 验证通过

### Requirement: 提交待定项

`aide decide submit <file>` SHALL：
1. 读取并解析 JSON 文件
2. 验证 DecideInput 结构
3. 生成 session_id（时间戳格式 YYYY-MM-DDTHH-MM-SS）
4. 添加 `_meta` 信息并保存到 `.aide/decisions/pending.json`
5. 启动后台 HTTP 服务器子进程
6. 等待 `server.json` 出现（包含端口信息）
7. 输出访问 URL

输出：
```
→ Web 服务已启动
→ 请访问: http://localhost:<port>
→ 用户完成决策后执行 aide decide result 获取结果
```

#### Scenario: 正常提交
- **WHEN** 运行 `aide decide submit decisions.json`
- **AND** JSON 文件格式正确
- **THEN** 保存 pending.json
- **AND** 启动 HTTP 服务器
- **AND** 输出访问 URL

#### Scenario: 无效 JSON
- **WHEN** 提交的文件不是有效 JSON
- **THEN** 输出解析错误信息

#### Scenario: 验证失败
- **WHEN** JSON 结构不符合 DecideInput 规范
- **THEN** 输出具体的验证错误信息

### Requirement: 获取决策结果

`aide decide result` SHALL：
1. 读取 `.aide/decisions/pending.json` 获取 session_id
2. 检查 `.aide/decisions/{session_id}.json` 是否存在
3. 如存在：输出 DecideOutput JSON 并清理 server.json
4. 如不存在：检查服务器是否仍在运行并提示用户

#### Scenario: 结果已提交
- **WHEN** 用户已在 Web UI 中完成决策
- **AND** 运行 `aide decide result`
- **THEN** 输出 DecideOutput JSON

#### Scenario: 结果未就绪
- **WHEN** 用户尚未完成决策
- **AND** 运行 `aide decide result`
- **THEN** 提示用户访问 Web UI 完成决策

### Requirement: HTTP 服务器

系统 SHALL 提供异步 HTTP 服务器（基于 tokio + axum）用于 decide Web UI：

**启动流程**：
1. 端口探测：从 `decide.port` 配置开始，依次尝试 port 到 port+9
2. 绑定到 `decide.bind` 配置地址（默认 `127.0.0.1`）
3. 将 PID、端口、URL 写入 `.aide/decisions/server.json`

**关闭触发**：
- POST `/api/submit` 成功后自动关闭
- 收到 SIGTERM 信号
- 超时（`decide.timeout` 配置，0 = 不超时）

**CORS**：所有响应 SHALL 包含 `Access-Control-Allow-*` headers。

#### Scenario: 端口被占用
- **WHEN** 默认端口 3721 被占用
- **THEN** 自动尝试 3722、3723...直到 3730
- **AND** 使用第一个可用端口

#### Scenario: 决策完成后自动关闭
- **WHEN** 用户在 Web UI 提交决策
- **THEN** 保存结果后服务器自动关闭

### Requirement: HTTP API 路由

系统 SHALL 提供以下 API 路由：

**GET `/api/items`**
- 返回 DecideInput 数据（不含 _meta）
- 如果 item 有 location，读取源文件对应行范围的内容一并返回

**POST `/api/submit`**
- 接收 DecideOutput JSON（最大 1MB）
- 验证 decisions 与 pending items 的一致性
- 保存到 `.aide/decisions/{session_id}.json`
- 返回 `{"success": true, "message": "决策已保存"}`
- 触发服务器关闭

**静态文件服务**
- GET `/` 或 `/index.html` → HTML 文件
- GET `/style.css` → CSS 文件
- GET `/app.js` → JavaScript 文件
- 静态文件从 Web 目录加载（默认为可执行文件相对路径 `web/`，可通过 `--web-dir` 参数覆盖）

**OPTIONS `*`**
- 返回 200 + CORS headers

**错误响应**：
- 400: 无效 JSON 或验证错误
- 404: 未知路由
- 405: 不支持的 HTTP 方法
- 413: 请求体超过 1MB
- 500: 服务器内部错误

#### Scenario: 获取待定项列表
- **WHEN** GET `/api/items`
- **THEN** 返回 JSON 包含 task、source、items
- **AND** 有 location 的 item 包含 source_content 字段

#### Scenario: 提交决策
- **WHEN** POST `/api/submit` 携带有效 DecideOutput JSON
- **THEN** 返回成功响应
- **AND** 结果保存到文件
- **AND** 服务器计划关闭

#### Scenario: 请求体过大
- **WHEN** POST 请求体超过 1MB
- **THEN** 返回 413 状态码

### Requirement: Web 前端

系统 SHALL 提供 Web 前端界面用于用户决策，文件从 `--web-dir` 参数指定的目录加载（默认为可执行文件所在目录下的 `web/`）。

前端 SHALL 复用现有的 HTML/CSS/JS 文件（`aide/decide/web/`），功能包括：

**页面结构**：
- 标题："Aide 待定项确认"
- 任务信息展示（task、source）
- 决策项卡片列表
- 进度计数器和提交按钮

**决策项卡片**：
- 标题（带编号）
- 推荐标记（如有 recommend）
- 上下文说明（如有 context）
- 源码位置和代码片段（如有 location）
- 选项列表（单选按钮）
  - 评分（如有 score）
  - 优劣势（如有 pros/cons）
- 备注文本框

**交互行为**：
- 所有项选择完成后才能提交
- 提交中按钮显示"提交中..."并禁用
- 提交成功后显示覆盖层："决策已提交\n您可以关闭此页面"
- 提交失败显示右下角错误提示（4 秒自动消失）

#### Scenario: 所有项选择后提交
- **WHEN** 用户为每个决策项选择了一个选项
- **THEN** 提交按钮变为可用
- **AND** 点击后提交决策到 POST `/api/submit`

#### Scenario: 提交成功
- **WHEN** 决策提交成功
- **THEN** 显示成功覆盖层
- **AND** 提交按钮永久禁用

### Requirement: 决策记录存储

系统 SHALL 将完成的决策记录保存为：

**文件路径**：`.aide/decisions/{session_id}.json`

**格式**：
```
{
  input: DecideInput,      // 不含 _meta
  output: DecideOutput,
  completed_at: string     // ISO 8601
}
```

**服务器信息**：`.aide/decisions/server.json`
```
{
  pid: number,
  port: number,
  url: string,
  started_at: string
}
```

#### Scenario: 保存决策记录
- **WHEN** 用户通过 Web UI 提交决策
- **THEN** 保存完整的 input + output + 时间戳
- **AND** session_id 与 pending.json 中的一致

#### Scenario: 服务器信息记录
- **WHEN** HTTP 服务器启动
- **THEN** 写入 server.json 包含 PID、端口、URL
- **WHEN** 服务器关闭后
- **AND** 用户运行 `aide decide result`
- **THEN** 清理 server.json
