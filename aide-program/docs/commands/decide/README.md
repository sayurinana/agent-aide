# aide decide 详细设计（实现交接包）

本目录为 `aide decide` 子命令的**详细设计**。目标是让接手开发者在不阅读额外上下文的情况下，能够依据本文档集完成实现、联调与验证。

实现位置：
- 核心实现：`aide-program/aide/decide/`
- CLI 入口：`aide-program/aide/main.py` 的 `aide decide ...` 子命令树

上游/关联文档：
- 概览设计：[`aide-program/docs/commands/decide.md`](../decide.md)
- 数据格式规范（输入/输出格式）：[`aide-program/docs/formats/data.md`](../../formats/data.md)
- 配置格式规范：[`aide-program/docs/formats/config.md`](../../formats/config.md)
- 插件侧调用契约：[`/aide:prep`](../../../../aide-marketplace/aide-plugin/docs/commands/prep.md)

## 一、范围与目标

### 1.1 目标

- 提供**程序化的待定项确认机制**，替代终端中的逐项文本确认
- 以 Web 界面呈现待定项，支持选项选择与备注添加
- 存储决策记录，支持历史追溯
- 遵循"静默即成功"的输出原则

### 1.2 非目标

- 不分析待定项内容（这是 LLM 的职责）
- 不做决策建议或推荐排序
- 不修改业务代码
- 不实现复杂的用户认证或多用户支持

## 二、关键约定（必须先统一）

1. **技术选型**：
   - HTTP 服务：使用 Python 标准库 `http.server`，无需额外依赖
   - Web 前端：使用纯 HTML/CSS/JavaScript，无需构建工具，直接嵌入 Python 代码或作为静态资源

2. **服务生命周期**：
   - `aide decide submit '<json>'` 启动服务并阻塞等待
   - 用户在 Web 界面提交决策后，服务自动关闭
   - 服务关闭后，LLM 调用 `aide decide result` 获取结果

3. **端口配置**：
   - 默认端口：3721
   - 端口被占用时：尝试下一个端口（3722、3723...），最多尝试 10 次
   - 可通过配置文件 `decide.port` 指定固定端口

4. **数据存储**：
   - 待处理数据：`.aide/decisions/pending.json`
   - 历史记录：`.aide/decisions/{timestamp}.json`
   - `.aide/` 默认被 gitignore

5. **超时策略**：
   - 默认无超时（等待用户操作）
   - 可通过配置 `decide.timeout` 设置超时时间（秒）
   - 超时后服务关闭，`aide decide result` 返回错误

## 三、文档索引（按实现模块拆分）

| 文档 | 内容 |
|------|------|
| [cli.md](cli.md) | CLI 命令规格、参数校验、输出规范 |
| [server.md](server.md) | HTTP 服务设计、API 端点、生命周期管理 |
| [web.md](web.md) | Web 前端设计、组件结构、交互流程 |
| [storage.md](storage.md) | 数据存储设计、文件格式、生命周期 |
| [verification.md](verification.md) | 验证清单（实现完成后的自检） |

## 四、推荐实现模块划分（仅文件/职责约定）

实现位于 `aide-program/aide/decide/`，按职责拆分为：

```
aide/decide/
├── __init__.py          # 模块导出
├── types.py             # 数据类型定义（DecideInput, DecideOutput 等）
├── storage.py           # 数据存储（pending.json 读写、历史记录）
├── server.py            # HTTP 服务器（启动、路由、关闭）
├── handlers.py          # API 请求处理器
└── web/                 # 静态资源目录
    ├── index.html       # 主页面
    ├── style.css        # 样式
    └── app.js           # 交互逻辑
```

各模块职责：

- `types`：定义数据结构，与 `aide-program/docs/formats/data.md` 保持一致
- `storage`：负责 pending.json 和历史记录的读写、清理
- `server`：HTTP 服务器生命周期管理、端口探测、静态资源服务
- `handlers`：处理 API 请求（获取待定项、提交决策）
- `web/`：纯静态前端资源，由 server 提供服务

> 注：本文档只约定职责与接口，不提供实现代码。

## 五、实现任务拆分（建议顺序）

### 阶段1：基础设施

1. 创建 `aide/decide/` 目录结构
2. 实现 `types.py`：定义 DecideInput、DecideItem、Option、DecideOutput、Decision 等数据类型
3. 实现 `storage.py`：pending.json 读写、历史记录保存

### 阶段2：CLI 入口

4. 在 `main.py` 添加 `aide decide` 子命令路由
5. 实现 JSON 解析与验证逻辑
6. 实现 `aide decide result` 命令

### 阶段3：HTTP 服务

7. 实现 `server.py`：HTTP 服务器基础框架
8. 实现端口探测逻辑
9. 实现 `handlers.py`：API 端点处理
10. 实现服务生命周期管理（启动、等待、关闭）

### 阶段4：Web 前端

11. 创建 `web/index.html`：页面结构
12. 创建 `web/style.css`：样式设计
13. 创建 `web/app.js`：交互逻辑（加载数据、选择选项、提交决策）

### 阶段5：集成与验证

14. 端到端测试：完整流程验证
15. 按 `verification.md` 逐项检查

### 依赖关系

```
@startuml
skinparam defaultFontName "PingFang SC"

[阶段1: 基础设施] as P1
[阶段2: CLI入口] as P2
[阶段3: HTTP服务] as P3
[阶段4: Web前端] as P4
[阶段5: 集成验证] as P5

P1 --> P2
P1 --> P3
P2 --> P3
P3 --> P4
P1 --> P5
P2 --> P5
P3 --> P5
P4 --> P5
@enduml
```

## 六、整体业务流程

```
@startuml
skinparam defaultFontName "PingFang SC"

participant LLM
participant "aide decide" as CLI
participant "HTTP Server" as Server
participant "Web Browser" as Browser
participant User

== 提交待定项 ==
LLM -> CLI : aide decide submit '<json>'
CLI -> CLI : 解析并验证 JSON
CLI -> CLI : 保存到 pending.json
CLI -> Server : 启动 HTTP 服务
CLI --> LLM : 输出访问链接
note right: → Web 服务已启动\n→ 请访问: http://localhost:3721\n→ 等待用户完成决策...

== 用户操作 ==
LLM -> User : 告知访问链接
User -> Browser : 打开链接
Browser -> Server : GET /
Server --> Browser : 返回 index.html
Browser -> Server : GET /api/items
Server --> Browser : 返回待定项数据
Browser -> User : 渲染界面
User -> Browser : 选择选项、添加备注
User -> Browser : 点击提交
Browser -> Server : POST /api/submit
Server -> Server : 保存决策结果
Server --> Browser : 返回成功
Server -> Server : 关闭服务

== 获取结果 ==
CLI --> LLM : 服务已关闭
LLM -> CLI : aide decide result
CLI -> CLI : 读取最新决策
CLI --> LLM : 返回 JSON 结果

@enduml
```

## 七、风险与待定项（需要开发前确认）

### 7.1 需要确认的设计决策

| 问题 | 建议方案 | 备选方案 |
|------|----------|----------|
| 前端是否需要支持移动端 | 否，仅支持桌面浏览器 | 响应式设计 |
| 是否支持多个待定项会话并行 | 否，同一时间只能有一个 pending | 支持多会话 |
| 服务启动后是否自动打开浏览器 | 否，仅输出链接 | 使用 webbrowser 模块自动打开 |

### 7.2 潜在风险

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 端口被占用 | 服务无法启动 | 自动尝试下一个端口 |
| 用户长时间不操作 | 服务一直阻塞 | 可配置超时时间 |
| 浏览器兼容性 | 界面显示异常 | 使用标准 HTML/CSS/JS，避免新特性 |
| JSON 数据过大 | 解析/传输慢 | 限制 items 数量（建议 ≤50） |

### 7.3 后续优化方向（不在本次实现范围）

- 支持键盘快捷键操作
- 支持决策历史查看
- 支持决策导出（Markdown/PDF）
- 支持自定义主题

## 八、相关文档

- [program 导览](../../README.md)
- [decide 概览设计](../decide.md)
- [数据格式文档](../../formats/data.md)
- [配置格式文档](../../formats/config.md)
- [aide skill 设计文档](../../../../aide-marketplace/aide-plugin/docs/skill/aide.md)
