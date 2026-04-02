# 规范：aide-memory 目录结构

## ADDED: aide-memory 目录结构

### 场景：初始化 aide-memory

**前置条件**：
- 项目目录存在
- 用户运行 `aide init`

**操作**：
```bash
aide init
```

**预期结果**：
- 创建 aide-memory/ 目录及所有子目录
- 生成 config.toml 配置文件
- 生成 config.md 文档
- 生成默认模板文件
- 输出成功消息

**目录结构**：
```
aide-memory/
├── memory/
│   ├── structure/
│   ├── concepts/
│   ├── diagram/
│   └── overview.md (空文件，带注释)
├── tasks/
├── archived-tasks/
├── config.toml
├── config.md
├── branches.json (空 JSON 对象)
├── branches.md (自动生成的空报告)
├── templates/
│   ├── 任务口述模板.md
│   └── 任务解析指导.md
├── aide-process-overview.md (占位符)
└── AGENT.md (占位符)
```

### 场景：config.toml 格式

**内容示例**：
```toml
# Aide 配置文件

[task]
description_file = "task-now.md"
template = "任务口述模板.md"
parse_guide = "任务解析指导.md"

[branch]
prefix = ""
format = "task-{n}"
resident = "dev"

[git]
auto_commit_on_switch = true
auto_commit_message = "暂存：清理仓库状态以切换分支"
bye_commit_message = "暂存：清理仓库状态"
```

### 场景：重复初始化

**前置条件**：
- aide-memory/ 目录已存在

**操作**：
```bash
aide init
```

**预期结果**：
- 输出警告：aide-memory 目录已存在
- 询问用户是否覆盖（默认：否）
- 若用户确认，备份现有目录后重新初始化

## ADDED: config.md 文档

### 内容结构

```markdown
# Aide 配置说明

## 任务配置 [task]

### description_file
- 类型：String
- 默认值：task-now.md
- 说明：任务描述文档路径（相对于项目根目录）

### template
- 类型：String
- 默认值：任务口述模板.md
- 说明：任务模板路径（相对于 templates/）

### parse_guide
- 类型：String
- 默认值：任务解析指导.md
- 说明：任务解析指导文档路径（相对于 templates/）

## 分支配置 [branch]

### prefix
- 类型：String
- 默认值：空
- 说明：任务分支名前缀

### format
- 类型：String
- 默认值：task-{n}
- 说明：分支名格式，{n} 为任务编号

### resident
- 类型：String
- 默认值：dev
- 说明：常驻工作分支名（不建议使用 master/main）

## Git 配置 [git]

### auto_commit_on_switch
- 类型：Boolean
- 默认值：true
- 说明：切换分支时自动暂存并提交

### auto_commit_message
- 类型：String
- 默认值：暂存：清理仓库状态以切换分支
- 说明：自动提交的默认消息

### bye_commit_message
- 类型：String
- 默认值：暂存：清理仓库状态
- 说明：bye 操作的默认提交消息
```
