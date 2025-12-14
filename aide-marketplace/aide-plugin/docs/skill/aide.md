# Aide Skill 设计文档

## 一、背景

### 1.1 解决的问题

LLM 在执行任务时需要调用各种工具，但：

| 问题 | 影响 |
|------|------|
| 命令语法分散 | 每次都要回忆或查找 |
| 输出格式不明确 | 难以正确解析结果 |
| 使用场景模糊 | 不知道何时该用什么命令 |

### 1.2 设计目标

提供**统一的工具使用指南**：
- 所有 aide 子命令的语法和参数
- 输入输出格式说明
- 典型使用示例

---

## 二、职责

### 2.1 做什么

告诉 LLM：
- aide 有哪些子命令可用
- 每个子命令怎么调用
- 调用后会得到什么结果

### 2.2 不做什么

- 不涉及流程指导（那是 Commands 的职责）
- 不涉及业务逻辑
- 不涉及决策判断

---

## 三、子命令索引

| 子命令 | 用途 | 详细设计 |
|--------|------|----------|
| `aide env` | 环境检测与修复 | [env.md](../../../../aide-program/docs/commands/env.md) |
| `aide flow` | 进度追踪与 git 集成 | [flow.md](../../../../aide-program/docs/commands/flow.md) |
| `aide decide` | 待定项 Web 确认 | [decide.md](../../../../aide-program/docs/commands/decide.md) |
| `aide config` | 配置读写 | [config.md](../../../../aide-program/docs/formats/config.md) |
| `aide init` | 初始化 .aide 目录 | [init.md](../../../../aide-program/docs/commands/init.md) |

---

## 四、输出格式规范

### 4.1 前缀符号

| 前缀 | 含义 | 处理方式 |
|------|------|----------|
| `✓` | 成功 | 继续执行 |
| `⚠` | 警告（可继续） | 记录后继续 |
| `✗` | 错误（需处理） | 按提示处理或告知用户 |
| `→` | 进行中/信息 | 读取信息 |

### 4.2 静默原则

**无输出 = 正常完成**

只有在需要反馈信息时才会有输出。

---

## 五、子命令接口规格

### 5.1 aide env list

**用途**：列出所有可用的环境检测模块

**语法**：
```
aide env list
```

**输出**：
```
可用模块:
  模块          描述                    能力               需要配置
  ────────────────────────────────────────────────────────────
  python       Python 解释器版本         check            否
  uv           uv 包管理器              check            否
  venv         Python 虚拟环境          check, ensure    是 [path]
  requirements Python 依赖管理          check, ensure    是 [path]

当前启用: python, uv, venv, requirements
```

### 5.2 aide env ensure

**用途**：检测并修复开发环境

**语法**：
```
aide env ensure [--runtime] [--modules M1,M2] [--all] [-v]
```

**参数**：

| 参数 | 说明 |
|------|------|
| `--runtime` | 仅检查 aide 运行时环境（python + uv），不依赖配置文件 |
| `--modules M1,M2` | 指定要检测的模块（逗号分隔） |
| `--all` | 检测所有已启用模块，仅检查不修复 |
| `-v, --verbose` | 显示详细配置信息（工作目录、配置路径、模块配置等） |

**输出**：

```
# 成功
✓ python: 3.14.2 (>=3.11)
✓ uv: uv 0.9.16
✓ venv: .venv
✓ requirements: requirements.txt
✓ 环境就绪 (python:3.14.2, uv:uv 0.9.16, venv:.venv, requirements:requirements.txt)

# 自动修复
✓ python: 3.14.2 (>=3.11)
✓ uv: uv 0.9.16
→ venv: 虚拟环境不存在: .venv，尝试修复...
✓ venv: 已创建
✓ 环境就绪 (...)

# 失败（启用模块缺少配置）
✓ python: 3.14.2 (>=3.11)
✓ uv: uv 0.9.16
✗ venv: 已启用但缺少配置项: path

# --verbose 输出（供人工确认）
============================================================
环境检测详细信息
============================================================

  工作目录: /home/user/myproject
  配置文件: /home/user/myproject/.aide/config.toml
  配置存在: 是

  启用模块: python, uv, venv, requirements
  目标模块: python, uv, venv, requirements

  [venv] 配置:
    path: .venv
    path (绝对): /home/user/myproject/.venv
    path (存在): 是
...
```

### 5.3 aide init

**用途**：初始化 .aide 目录和默认配置

**语法**：
```
aide init
```

**行为**：
1. 创建 `.aide/` 目录
2. 生成默认 `config.toml`
3. 检查并更新 `.gitignore`

**输出**：
```
✓ 已创建默认配置 .aide/config.toml
✓ 初始化完成，.aide/ 与默认配置已准备就绪
```

### 5.4 aide flow

**用途**：进度追踪 + Git 自动提交 + 流程校验

#### aide flow start

**语法**：
```
aide flow start <环节名> "<总结>"
```

**参数**：

| 参数 | 说明 |
|------|------|
| `<环节名>` | task-optimize / flow-design / impl / verify / docs / finish |
| `<总结>` | 本次操作的简要说明 |

**示例**：
```bash
aide flow start task-optimize "开始任务准备: 实现用户登录功能"
aide flow start flow-design "开始任务: 实现用户认证模块"
```

#### aide flow next-step

**语法**：
```
aide flow next-step "<总结>"
```

**示例**：
```bash
aide flow next-step "完成数据库模型设计"
```

#### aide flow back-step

**语法**：
```
aide flow back-step "<原因>"
```

**示例**：
```bash
aide flow back-step "发现字段命名不符合规范，需要调整"
```

#### aide flow next-part

**语法**：
```
aide flow next-part <环节名> "<总结>"
```

**示例**：
```bash
aide flow next-part impl "流程设计完成，开始实现"
aide flow next-part verify "实现完成，开始验证"
```

#### aide flow back-part

**语法**：
```
aide flow back-part <环节名> "<原因>"
```

**示例**：
```bash
aide flow back-part flow-design "实现中发现设计遗漏，需要补充"
```

#### aide flow issue

**语法**：
```
aide flow issue "<描述>"
```

**示例**：
```bash
aide flow issue "测试覆盖率较低，后续需要补充"
```

#### aide flow error

**语法**：
```
aide flow error "<描述>"
```

**示例**：
```bash
aide flow error "数据库连接失败，无法继续"
```

#### 环节名称与使用场景

| 环节名 | 说明 | 使用场景 |
|--------|------|----------|
| `task-optimize` | 任务优化 | prep 阶段 |
| `flow-design` | 流程设计 | exec 阶段 |
| `impl` | 迭代实现 | exec 阶段 |
| `verify` | 验证交付 | exec 阶段 |
| `docs` | 文档更新 | exec 阶段 |
| `finish` | 收尾 | exec 阶段 |

#### Git 集成

每次调用 aide flow 命令都会自动执行：
1. `git add .`
2. `git commit -m "[aide] <环节>: <总结>"`

#### 流程校验

aide flow 会自动校验环节跳转是否合理：
- `flow-design` → `impl` ✓
- `impl` → `verify` ✓
- `impl` → `flow-design` ✓（回退）
- `flow-design` → `finish` ✗（跳过环节）

### 5.5 aide decide

**用途**：通过 Web 界面处理待定项确认

#### aide decide submit（提交数据）

**语法**：
```
aide decide submit '<json数据>'
```

**输入格式**：见 [数据格式文档](../../../../aide-program/docs/formats/data.md)

**输出**：
```
→ Web 服务已启动
→ 请访问: http://localhost:3721
→ 等待用户完成决策...
```

#### aide decide result

**语法**：
```
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

### 5.6 aide config

**用途**：配置读写

#### aide config get

**语法**：
```
aide config get <key>
```

**参数**：
- `<key>`：使用点号分隔的键名，如 `task.source`

**示例**：
```bash
aide config get task.source
# 输出: → task.source = 'task-now.md'

aide config get flow.phases
# 输出: → flow.phases = ['task-optimize', 'flow-design', 'impl', 'verify', 'docs', 'finish']
```

#### aide config set

**语法**：
```
aide config set <key> <value>
```

**示例**：
```bash
aide config set task.source "my-task.md"
# 输出: ✓ 已更新 task.source = 'my-task.md'
```

---

## 六、典型使用场景

### 6.1 init 阶段

```bash
# 检查运行时环境
aide env ensure --runtime

# 初始化配置
aide init

# 检查项目环境
aide env ensure
```

### 6.2 prep 阶段

```bash
# 开始任务准备
aide flow start task-optimize "开始任务准备: 实现用户认证模块"

# 记录进度
aide flow next-step "任务分析完成"
aide flow next-step "任务优化完成，生成待定项"

# 处理待定项
aide decide submit '{"task":"...", "items":[...]}'
aide decide result

aide flow next-step "用户完成待定项确认"
aide flow next-step "任务准备完成"
```

### 6.3 exec 阶段

```bash
# 进入流程设计
aide flow next-part flow-design "进入流程设计环节"

# 记录设计进度
aide flow next-step "流程图设计完成"

# 进入实现
aide flow next-part impl "流程设计完成，开始实现"

# 记录实现进度
aide flow next-step "完成 User 模型定义"
aide flow next-step "完成密码加密工具"

# 记录问题
aide flow issue "部分边界情况未覆盖测试"

# 进入验证
aide flow next-part verify "实现完成，开始验证"

# 进入文档
aide flow next-part docs "验证通过，更新文档"

# 收尾
aide flow next-part finish "文档更新完成，收尾"
aide flow next-step "任务完成"
```

---

## 七、修改指南

### 7.1 修改子命令接口

1. 更新本文档对应章节
2. 修改执行文件 `../../skills/aide/SKILL.md`
3. 同步更新 [aide-program 对应文档](../../../../aide-program/docs/README.md)

### 7.2 新增子命令

1. 在本文档添加接口规格
2. 在执行文件添加使用说明
3. 更新子命令索引表
4. 在 aide-program 添加对应设计文档

---

## 八、相关文档

- [执行文件](../../skills/aide/SKILL.md)
- [aide-program 导览](../../../../aide-program/docs/README.md)
- [数据格式规范](../../../../aide-program/docs/formats/data.md)
- [plugin 导览](../README.md)
