# 验证清单（实现完成后的自检）

本清单用于在实现 `aide decide` 后进行验证，确保行为符合设计文档与 plugin 契约。

## 一、准备条件

- 已执行 `aide init`，确保 `.aide/` 目录存在
- 准备测试用的 JSON 数据（见附录）
- 确保端口 3721 未被占用（或准备测试端口探测）

## 二、CLI 用例

### 2.1 正常提交待定项

**步骤**：

1. 执行 `aide decide '<valid_json>'`

**期望**：

- 输出包含 `→ Web 服务已启动`
- 输出包含 `→ 请访问: http://localhost:3721`（或其他端口）
- 输出包含 `→ 等待用户完成决策...`
- `.aide/decisions/pending.json` 被创建
- 服务阻塞等待

### 2.2 JSON 解析失败

**步骤**：

1. 执行 `aide decide 'invalid json'`

**期望**：

- 输出 `✗ JSON 解析失败: ...`
- 退出码为 1
- 不创建 pending.json
- 不启动服务

### 2.3 数据验证失败

**步骤**：

1. 执行 `aide decide '{"task":"test"}'`（缺少必填字段）

**期望**：

- 输出 `✗ 数据验证失败: ...`
- 错误信息明确指出缺少的字段
- 退出码为 1

### 2.4 获取决策结果（无数据）

**步骤**：

1. 确保 `.aide/decisions/` 为空
2. 执行 `aide decide result`

**期望**：

- 输出 `✗ 未找到待定项数据`
- 退出码为 1

### 2.5 获取决策结果（有 pending 无 result）

**步骤**：

1. 执行 `aide decide '<json>'` 并立即中断（Ctrl+C）
2. 执行 `aide decide result`

**期望**：

- 输出 `✗ 尚无决策结果`
- 退出码为 1

### 2.6 获取决策结果（正常）

**步骤**：

1. 完成一次完整的决策流程
2. 执行 `aide decide result`

**期望**：

- 输出 JSON 格式的决策结果
- JSON 可被正确解析
- 包含所有待定项的决策
- 退出码为 0

## 三、HTTP 服务用例

### 3.1 端口探测

**步骤**：

1. 占用端口 3721（如 `nc -l 3721`）
2. 执行 `aide decide '<json>'`

**期望**：

- 服务在 3722 端口启动
- 输出显示正确的端口号

### 3.2 端口全部被占用

**步骤**：

1. 占用端口 3721-3730
2. 执行 `aide decide '<json>'`

**期望**：

- 输出 `✗ 无法启动服务: 端口 3721-3730 均被占用`
- 退出码为 1

### 3.3 GET /api/items

**步骤**：

1. 启动服务
2. 使用 curl 访问 `http://localhost:3721/api/items`

**期望**：

- 返回 200 状态码
- Content-Type 为 `application/json`
- 返回的 JSON 与 pending.json 内容一致

### 3.4 POST /api/submit（正常）

**步骤**：

1. 启动服务
2. 使用 curl 提交决策：
   ```bash
   curl -X POST http://localhost:3721/api/submit \
     -H "Content-Type: application/json" \
     -d '{"decisions":[{"id":1,"chosen":"jwt"}]}'
   ```

**期望**：

- 返回 200 状态码
- 返回 `{"success":true,"message":"决策已保存"}`
- 服务关闭
- 历史记录文件被创建

### 3.5 POST /api/submit（数据不完整）

**步骤**：

1. 启动服务（待定项有 2 个）
2. 提交只包含 1 个决策的数据

**期望**：

- 返回 400 状态码
- 返回错误信息指出缺少的决策
- 服务不关闭

### 3.6 静态资源服务

**步骤**：

1. 启动服务
2. 访问 `http://localhost:3721/`
3. 访问 `http://localhost:3721/style.css`
4. 访问 `http://localhost:3721/app.js`

**期望**：

- 所有资源返回 200 状态码
- Content-Type 正确
- 内容非空

### 3.7 404 处理

**步骤**：

1. 启动服务
2. 访问 `http://localhost:3721/nonexistent`

**期望**：

- 返回 404 状态码

### 3.8 服务中断（Ctrl+C）

**步骤**：

1. 启动服务
2. 按 Ctrl+C

**期望**：

- 服务正常关闭
- 无异常输出
- 退出码为 0 或 130（SIGINT）

## 四、Web 前端用例

### 4.1 页面加载

**步骤**：

1. 启动服务
2. 在浏览器中打开页面

**期望**：

- 页面正常渲染
- 显示任务名称和来源
- 显示所有待定项
- 提交按钮为禁用状态

### 4.2 选项选择

**步骤**：

1. 点击某个选项

**期望**：

- 选项被选中（视觉反馈）
- 进度更新（如 "已完成 1/2 项"）
- 若所有项已选择，提交按钮启用

### 4.3 推荐选项显示

**步骤**：

1. 提交包含 recommend 字段的数据
2. 查看页面

**期望**：

- 推荐选项有特殊标记
- 卡片头部显示推荐信息

### 4.4 备注输入

**步骤**：

1. 在备注框中输入文字
2. 提交决策
3. 查看决策结果

**期望**：

- 备注内容被保存
- 决策结果中包含 note 字段

### 4.5 提交决策

**步骤**：

1. 选择所有待定项的选项
2. 点击提交按钮

**期望**：

- 显示提交中状态
- 提交成功后显示成功提示
- 页面提示可以关闭

### 4.6 提交失败处理

**步骤**：

1. 模拟服务端错误（如断开网络）
2. 点击提交按钮

**期望**：

- 显示错误提示
- 提交按钮恢复可用
- 可以重试

### 4.7 浏览器兼容性

**步骤**：

1. 在 Chrome、Firefox、Safari、Edge 中分别测试

**期望**：

- 页面正常渲染
- 交互功能正常
- 无 JavaScript 错误

## 五、数据存储用例

### 5.1 pending.json 创建

**步骤**：

1. 执行 `aide decide '<json>'`
2. 检查 `.aide/decisions/pending.json`

**期望**：

- 文件存在
- JSON 格式正确
- 包含 `_meta` 字段
- `_meta.session_id` 格式正确

### 5.2 历史记录创建

**步骤**：

1. 完成一次决策流程
2. 检查 `.aide/decisions/` 目录

**期望**：

- 存在 `{session_id}.json` 文件
- 文件包含 input、output、completed_at 字段
- session_id 与 pending.json 中的一致

### 5.3 pending.json 覆盖

**步骤**：

1. 执行 `aide decide '<json1>'`
2. 中断服务
3. 执行 `aide decide '<json2>'`
4. 检查 pending.json

**期望**：

- pending.json 内容为 json2
- session_id 已更新

### 5.4 原子写入验证

**步骤**：

1. 在写入过程中模拟中断
2. 检查文件状态

**期望**：

- 文件要么是完整的旧内容，要么是完整的新内容
- 不会出现部分写入的损坏状态

## 六、端到端用例

### 6.1 完整流程

**步骤**：

1. 执行 `aide decide '<json>'`
2. 在浏览器中打开链接
3. 选择所有选项
4. 添加备注
5. 点击提交
6. 执行 `aide decide result`

**期望**：

- 整个流程无错误
- 决策结果正确反映用户选择
- 备注内容正确保存

### 6.2 与 /aide:prep 集成

**步骤**：

1. 模拟 LLM 调用流程
2. 构造待定项 JSON
3. 调用 aide decide
4. 完成决策
5. 获取结果

**期望**：

- 流程符合 plugin 契约
- 输出格式便于 LLM 解析

## 七、异常用例

### 7.1 .aide 目录不存在

**步骤**：

1. 删除 .aide 目录
2. 执行 `aide decide '<json>'`

**期望**：

- 输出 `✗ .aide 目录不存在，请先执行 aide init`
- 退出码为 1

### 7.2 pending.json 损坏

**步骤**：

1. 手动写入无效 JSON 到 pending.json
2. 执行 `aide decide result`

**期望**：

- 输出解析错误信息
- 建议重新执行 aide decide

### 7.3 超大 JSON 数据

**步骤**：

1. 构造包含 100 个待定项的 JSON
2. 执行 `aide decide '<json>'`

**期望**：

- 正常处理或给出合理的限制提示
- 不会导致内存溢出或崩溃

### 7.4 特殊字符处理

**步骤**：

1. 在 task、title、note 中包含特殊字符（中文、emoji、引号、换行）
2. 完成决策流程

**期望**：

- 特殊字符正确保存和显示
- 不会导致 JSON 解析错误

## 八、附录：测试数据

### 8.1 最小有效数据

```json
{
  "task": "测试任务",
  "source": "test.md",
  "items": [
    {
      "id": 1,
      "title": "测试问题",
      "options": [
        {"value": "a", "label": "选项A"},
        {"value": "b", "label": "选项B"}
      ]
    }
  ]
}
```

### 8.2 完整数据

```json
{
  "task": "实现用户认证模块",
  "source": "task-now.md",
  "items": [
    {
      "id": 1,
      "title": "认证方式选择",
      "location": {
        "file": "task-now.md",
        "start": 5,
        "end": 7
      },
      "context": "任务描述中未明确指定认证方式，需要确认",
      "options": [
        {
          "value": "jwt",
          "label": "JWT Token 认证",
          "score": 85,
          "pros": ["无状态", "易于扩展", "跨域友好"],
          "cons": ["Token 无法主动失效", "需要处理刷新"]
        },
        {
          "value": "session",
          "label": "Session 认证",
          "score": 70,
          "pros": ["实现简单", "可主动失效"],
          "cons": ["需要存储", "扩展性差"]
        }
      ],
      "recommend": "jwt"
    },
    {
      "id": 2,
      "title": "密码加密算法",
      "context": "选择密码存储的加密算法",
      "options": [
        {
          "value": "bcrypt",
          "label": "bcrypt",
          "score": 90,
          "pros": ["安全性高", "自带盐值"],
          "cons": ["计算较慢"]
        },
        {
          "value": "argon2",
          "label": "Argon2",
          "score": 95,
          "pros": ["最新标准", "抗GPU攻击"],
          "cons": ["库支持较少"]
        }
      ],
      "recommend": "bcrypt"
    }
  ]
}
```

### 8.3 期望的决策结果

```json
{
  "decisions": [
    {"id": 1, "chosen": "jwt"},
    {"id": 2, "chosen": "bcrypt", "note": "团队更熟悉 bcrypt"}
  ]
}
```

## 九、相关文档

- [decide 详细设计入口](README.md)
- [CLI 规格](cli.md)
- [HTTP 服务设计](server.md)
- [Web 前端设计](web.md)
- [数据存储设计](storage.md)
