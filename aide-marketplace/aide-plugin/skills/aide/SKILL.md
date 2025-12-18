---
name: aide
description: Aide 工作流工具集。提供环境管理、进度追踪、待定项确认等功能。当需要执行 aide 命令、管理任务进度、处理待定项确认时使用。
---

# Aide 工具使用指南

Aide 是一套命令行工具，用于支持 Aide 工作流体系。所有 aide 命令的输出遵循精简原则。

## 输出格式

| 前缀 | 含义 |
|------|------|
| `✓` | 成功 |
| `⚠` | 警告（可继续） |
| `✗` | 错误（需处理） |
| `→` | 进行中/信息 |

**静默原则**：无输出 = 正常完成

---

## aide env - 环境管理

### aide env ensure

检测并修复开发环境。

```bash
# 仅检查 aide 运行时环境（init 流程使用）
aide env ensure --runtime

# 检查项目开发环境（按配置启用的模块）
aide env ensure
```

**参数**：

| 参数 | 说明 |
|------|------|
| `--runtime` | 仅检查 aide 运行时环境（python + uv） |
| `--modules M1,M2` | 指定要检测的模块（逗号分隔） |
| `--all` | 检测所有已启用模块，仅检查不修复 |
| `-v, --verbose` | 显示详细配置信息 |

**输出示例**：

```
# 成功
✓ python: 3.14.2 (>=3.11)
✓ uv: uv 0.9.16
✓ 环境就绪 (python:3.14.2, uv:uv 0.9.16)
```

```
# 失败
✗ venv: 已启用但缺少配置项: path
```

**失败处理**：当 `aide env ensure` 输出 `✗` 时，触发 `env-config` skill 获取详细配置指导。

### aide env list

列出所有可用的环境检测模块。

```bash
aide env list
```

### aide env set

设置环境配置（详细用法见 `env-config` skill）。

```bash
aide env set modules <模块列表>
aide env set <模块名>.<配置项> <值>
```

---

## aide flow - 进度追踪

进度追踪工具，集成 git 自动提交和流程校验。

### aide flow start

开始新任务。

```bash
aide flow start <环节名> "<总结>"
```

**参数**：
- `<环节名>`：task-optimize / flow-design / impl / verify / docs / finish
- `<总结>`：本次操作的简要说明

**示例**：
```bash
aide flow start flow-design "开始任务: 实现用户登录功能"
```

### aide flow next-step

记录小步骤前进。

```bash
aide flow next-step "<总结>"
```

**示例**：
```bash
aide flow next-step "完成数据库模型设计"
```

### aide flow back-step

记录小步骤回退。

```bash
aide flow back-step "<原因>"
```

**示例**：
```bash
aide flow back-step "发现字段命名不符合规范，需要调整"
```

### aide flow next-part

进入下一个大环节。

```bash
aide flow next-part <环节名> "<总结>"
```

**示例**：
```bash
aide flow next-part impl "流程设计完成，开始实现"
```

### aide flow back-part

回退到之前的大环节。

```bash
aide flow back-part <环节名> "<原因>"
```

**示例**：
```bash
aide flow back-part flow-design "实现中发现设计遗漏，需要补充"
```

### aide flow issue

记录一般问题（不阻塞继续）。

```bash
aide flow issue "<描述>"
```

**示例**：
```bash
aide flow issue "测试覆盖率较低，后续需要补充"
```

### aide flow error

记录严重错误（需要解决）。

```bash
aide flow error "<描述>"
```

**示例**：
```bash
aide flow error "数据库连接失败，无法继续"
```

### aide flow status

查看当前任务状态。

```bash
aide flow status
```

**输出示例**：
```
→ 任务 ID: 2025-12-15T17-28-53
→ 环节: impl
→ 步骤: 11
→ 开始时间: 2025-12-15T17:28:53+08:00
→ 最新操作: 完成数据库模型设计
→ 操作时间: 2025-12-15T18:09:36+08:00
→ Git 提交: 79facec
```

### aide flow list

列出所有任务（当前 + 归档）。

```bash
aide flow list
```

**输出示例**：
```
→ 任务列表:
  *[1] 2025-12-15T17-28-53 (impl) 开始任务准备: 阅读任务...
   [2] 2025-12-14T09-00-00 (finish) 实现用户登录功能
→ 提示: 使用 aide flow show <task_id> 查看详细状态
```

> 注：`*` 标记表示当前活跃任务

### aide flow show

查看指定任务的详细状态历史。

```bash
aide flow show <task_id>
```

**示例**：
```bash
aide flow show 2025-12-15T17-28-53
```

**输出示例**：
```
→ 任务 ID: 2025-12-15T17-28-53
→ 当前环节: impl
→ 当前步骤: 3
→ 开始时间: 2025-12-15T17:28:53+08:00
→
→ 历史记录:
  [task-optimize] 开始任务准备 [7defeac]
         2025-12-15T17:28:53+08:00 (start)
  [flow-design] 进入流程设计环节 [7e1f026]
         2025-12-15T17:51:56+08:00 (next-part)
  [impl] 流程设计完成，进入实现环节 [5f27bb7]
         2025-12-15T17:58:58+08:00 (next-part)
```

### 环节名称列表

| 环节名 | 说明 | 使用场景 |
|-------|------|---------|
| `task-optimize` | 任务优化 | prep 阶段使用 |
| `flow-design` | 流程设计 | exec 阶段使用 |
| `impl` | 迭代实现 | exec 阶段使用 |
| `verify` | 验证交付 | exec 阶段使用 |
| `docs` | 文档更新 | exec 阶段使用 |
| `confirm` | 用户确认 | 审阅与返工 |
| `finish` | 收尾 | exec 阶段使用 |

### 流程校验

aide flow 会自动校验环节跳转是否合理：
- `flow-design` → `impl` ✓
- `impl` → `verify` ✓
- `impl` → `flow-design` ✓（回退）
- `flow-design` → `finish` ✗（跳过环节）

### Git 集成

每次调用 aide flow 命令都会自动执行：
1. `git add .`
2. `git commit -m "<自动生成的提交信息>"`

提交信息格式：`[aide] <环节>: <总结>`

---

## aide decide - 待定项确认

通过 Web 界面处理待定项确认。服务在后台运行，用户完成决策后自动关闭。

```
aide decide {submit,result} ...

子命令:
  submit <file>  从文件读取待定项数据，启动后台 Web 服务
  result         获取用户决策结果
```

### aide decide submit

从 JSON 文件读取待定项数据，启动后台 Web 服务，立即返回。

```bash
aide decide submit <json文件路径>
```

**使用流程**：
1. 将待定项数据写入 JSON 文件
2. 执行 `aide decide submit <文件路径>` 启动服务
3. 告知用户访问 Web 界面进行决策
4. 用户完成后执行 `aide decide result` 获取结果

**JSON 文件格式**：
```json
{
  "task": "任务简述",
  "source": "now-task.md",
  "items": [
    {
      "id": 1,
      "title": "问题标题",
      "location": {
        "file": "now-task.md",
        "start": 5,
        "end": 7
      },
      "context": "问题背景说明",
      "options": [
        {
          "value": "option_a",
          "label": "选项A描述",
          "score": 85,
          "pros": ["优点1", "优点2"],
          "cons": ["缺点1"]
        },
        {
          "value": "option_b",
          "label": "选项B描述",
          "score": 70,
          "pros": ["优点1"],
          "cons": ["缺点1", "缺点2"]
        }
      ],
      "recommend": "option_a"
    }
  ]
}
```

**配置项**（在 `.aide/config.toml` 的 `[decide]` 节）：

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| `port` | 3721 | 起始端口（自动探测可用端口） |
| `bind` | `"127.0.0.1"` | 监听地址，设为 `"0.0.0.0"` 可允许外部访问 |
| `url` | `""` | 自定义访问地址，为空时自动生成 |
| `timeout` | 0 | 超时时间（秒），0 表示不超时 |

**输出**：
```
→ Web 服务已启动
→ 请访问: http://localhost:3721
→ 用户完成决策后执行 aide decide result 获取结果
```

> 注：服务在后台运行，命令立即返回。用户提交决策后服务自动关闭。

### aide decide result

获取用户决策结果。

```bash
aide decide result
```

**输出格式**：
```json
{
  "decisions": [
    {"id": 1, "chosen": "option_a"},
    {"id": 2, "chosen": "option_b", "note": "用户的补充说明"}
  ]
}
```

**错误情况**：
- 尚无决策结果（服务运行中）：提示等待用户完成操作
- 尚无决策结果（服务已关闭）：提示重新执行 submit

> 注：`note` 字段仅在用户添加备注时出现
> 注：如果数据中有 `recommend` 字段，对应选项会默认选中

---

## aide config - 配置管理

### aide config get

获取配置值。

```bash
aide config get <key>
```

**示例**：
```bash
aide config get flow.phases
```

**输出**：
```
["flow-design", "impl", "verify", "docs", "finish"]
```

### aide config set

设置配置值。

```bash
aide config set <key> <value>
```

**示例**：
```bash
aide config set env.python.version ">=3.11"
```

---

## aide init

初始化 .aide 目录和默认配置。

```bash
aide init
```

**操作**：
1. 创建 `.aide/` 目录
2. 生成默认 `config.toml`
3. 检查并更新 `.gitignore`

**输出**：
```
✓ 已创建 .aide/ 目录
✓ 已生成默认配置
✓ 已添加 .aide/ 到 .gitignore
```

---

## 数据存储

所有 aide 数据文件存放在项目根目录的 `.aide/` 下：

```
.aide/
├── config.toml          # 项目配置（自文档化，含完整注释）
├── flow-status.json     # 当前任务进度
├── decisions/           # 待定项决策记录
│   └── {timestamp}.json
├── logs/                # 历史任务归档
│   └── flow-status.{task_id}.json
├── diagrams/            # 流程图目录
│   ├── *.puml           # PlantUML 源文件
│   └── *.png            # 生成的图片
├── task-plans/          # 复杂任务计划文档（可配置）
│   ├── guide.md         # 任务计划总导览
│   └── spec-NN.md       # 子计划细则
└── project-docs/        # 项目文档（面向 LLM）
    ├── README.md        # 总导览
    ├── block-plan.md    # 区块计划
    └── blocks/          # 子区块文档
```

---

## 常见用法示例

### /aide:run 完整流程示例

```bash
# 1. 检查当前 flow 状态
aide flow status

# 2. 如果无活跃任务，开始新任务
aide flow start task-optimize "开始任务准备: 实现用户认证模块"

# 3. 任务分析和优化
aide flow next-step "任务分析完成"
aide flow next-step "任务优化完成，生成待定项"

# 4. 处理待定项（如有）
aide decide submit .aide/pending-items.json
# 用户完成后
aide decide result
aide flow next-step "用户完成待定项确认"

# 5. 进入流程设计
aide flow next-part flow-design "进入流程设计环节"
aide flow next-step "流程图设计完成"

# 6. 进入实现（自动校验并生成 PNG）
aide flow next-part impl "流程设计完成，进入实现环节"
aide flow next-step "完成 User 模型定义"
aide flow next-step "完成登录接口"

# 7. 进入验证
aide flow next-part verify "实现完成，进入验证环节"
aide flow next-step "验证完成: 所有测试通过"

# 8. 进入文档更新
aide flow next-part docs "验证通过，进入文档环节"
aide flow next-step "文档更新完成"

# 9. 进入用户确认
aide flow next-part confirm "文档更新完成，进入用户确认环节"
# 等待用户确认...
# 如用户确认通过：
aide flow next-step "用户确认通过"

# 10. 收尾
aide flow next-part finish "用户确认通过，进入收尾"
aide flow next-step "任务完成"
```

### 查看任务状态

```bash
# 查看当前任务
aide flow status

# 列出所有任务
aide flow list

# 查看指定任务详情
aide flow show 2025-12-15T17-28-53
```

### 续接未完成的任务

```bash
# 1. 查看当前进度
aide flow status
aide flow show <task_id>

# 2. 从中断处继续
aide flow next-step "继续实现: 完成密码加密工具"
```

### 处理待定项

```bash
# 提交待定项（建议保存到文件）
aide decide submit .aide/pending-items.json

# 获取结果
aide decide result
```

### 记录问题

```bash
aide flow issue "部分边界情况未覆盖测试"
aide flow error "CI 构建失败"
```

### 回退操作

```bash
# 小步骤回退
aide flow back-step "发现字段命名不符合规范"

# 大环节回退
aide flow back-part flow-design "实现中发现设计遗漏"
```

---

## 新命令体系

| 命令 | 说明 | 独立运行 |
|------|------|----------|
| `/aide:setup` | 环境配置（分析、检测、修复） | 是 |
| `/aide:load` | 项目认知载入 | 否（由 run 调用） |
| `/aide:docs` | 项目文档创建和维护 | 是 |
| `/aide:run` | 任务执行（核心命令） | 否 |

> 注：原 `/aide:init`、`/aide:prep`、`/aide:exec` 已合并重组为上述命令
