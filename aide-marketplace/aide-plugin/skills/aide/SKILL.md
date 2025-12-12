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
# 检查项目开发环境
aide env ensure

# 仅检查 aide 运行时环境（不依赖配置文件）
aide env ensure --runtime
```

**参数**：
- `--runtime`：仅检查 aide 程序自身运行所需的环境（Python 等），不读取项目配置文件

**输出示例**：
```
✓ 环境就绪 (python:3.12, uv:0.4.0)
```

```
⚠ 已修复: 创建虚拟环境 .venv
✓ 环境就绪 (python:3.12)
```

```
✗ Python 版本不满足要求 (需要 >=3.10, 当前 3.8)
  建议: 安装 Python 3.10+ 或使用 pyenv 管理版本
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

### 环节名称列表

| 环节名 | 说明 | 使用场景 |
|-------|------|---------|
| `task-optimize` | 任务优化 | prep 阶段使用 |
| `flow-design` | 流程设计 | exec 阶段使用 |
| `impl` | 迭代实现 | exec 阶段使用 |
| `verify` | 验证交付 | exec 阶段使用 |
| `docs` | 文档更新 | exec 阶段使用 |
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

通过 Web 界面处理待定项确认。

### aide decide

提交待定项数据并启动 Web 服务。

```bash
aide decide '<json数据>'
```

**JSON 格式**：
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

**输出**：
```
→ Web 服务已启动
→ 请访问: http://localhost:3721
→ 等待用户完成决策...
```

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

> 注：`note` 字段仅在用户添加备注时出现

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
├── config.toml          # 项目配置
├── flow-status.json     # 当前任务进度
├── decisions/           # 待定项决策记录
│   └── {timestamp}.json
└── logs/                # 操作日志
```

---

## 常见用法示例

### prep 阶段示例

```bash
# 开始任务准备
aide flow start task-optimize "开始任务准备: 实现用户认证模块"

# 记录分析进度
aide flow next-step "任务分析完成"
aide flow next-step "任务优化完成，生成待定项"
aide flow next-step "用户完成待定项确认"
aide flow next-step "任务准备完成"
```

### exec 阶段示例

```bash
# 开始流程设计
aide flow start flow-design "开始任务: 实现用户认证模块"

# 记录实现进度
aide flow next-step "完成 User 模型定义"
aide flow next-step "完成密码加密工具"
aide flow next-step "完成登录接口"

# 进入下一环节
aide flow next-part verify "实现完成，开始验证"
```

### 处理待定项

```bash
# 提交待定项（JSON 数据较长时建议保存到文件后通过 cat 传入）
aide decide '{"task":"...", "items":[...]}'

# 获取结果
aide decide result
```

### 记录问题

```bash
aide flow issue "部分边界情况未覆盖测试"
aide flow error "CI 构建失败"
```
