# 命令参考

## aide -V / aide --version

显示 aide 版本信息及 PlantUML 可用性状态。

```bash
aide -V
```

**输出示例（PlantUML 已安装）：**
```
aide 0.1.0

PlantUML:
  版本: 1.2025.4
  路径: ~/.aide/utils/plantuml/bin/plantuml
  状态: 可用
```

**输出示例（PlantUML 未安装）：**
```
aide 0.1.0

PlantUML:
  状态: 未安装
  提示: 运行 aide init --global 安装 PlantUML
```

---

## aide init

初始化 `aide-memory` 目录和默认配置文件。

```bash
aide init
aide init --global
```

**行为（无 --global）：**
- 确保全局配置 `~/.aide/config.toml` 存在
- 在当前目录创建 `aide-memory/` 目录
- 从全局配置复制到项目 `aide-memory/config.toml`
- 生成详细的 `config.md` 配置说明文档
- 创建 `tasks/`、`archived-tasks/`、`templates/`、`memory/` 等子目录
- 同步 commands 和 skills 到项目 `.claude/` 目录（需要先执行 `aide init --global`）
- 更新 `.gitignore` 添加 `aide-memory/` 条目
- 幂等操作，重复执行不会覆盖已有配置

**行为（--global）：**
- 在 `~/.aide/` 下创建全局配置（如不存在）
- 检测 Git 可用性，若可用则克隆/更新 agent-aide 仓库到 `~/.aide/agent-aide/`
  - 仓库地址可通过 `plugin.repo_url` 配置项自定义
  - 默认地址：`git@github.com:sayurinana/agent-aide.git`
- 检测 PlantUML 可执行程序是否已安装
  - 已安装：显示版本信息
  - 未安装：提示用户是否自动下载并安装

**插件同步说明：**
- 全局初始化时，agent-aide 仓库会被克隆到 `~/.aide/agent-aide/`
- 项目初始化时，`aide-plugin/commands/` 和 `aide-plugin/skills/` 会被复制到项目 `.claude/` 目录
- 若 Git 未安装或全局仓库不存在，项目初始化时会跳过插件同步并给出提示

---

## aide config

配置管理命令，支持读取和写入 `.aide/config.toml` 中的配置项。

### aide config get

```bash
aide config get <key>
```

读取指定配置项的值。键名使用点号分隔表示嵌套路径。

**示例：**
```bash
aide config get task.source        # → task-now.md
aide config get flow.phases        # → ["task-optimize", "flow-design", ...]
aide config get decide.port        # → 3721
```

**退出码：**
- 0: 成功
- 1: 键不存在

### aide config set

```bash
aide config set <key> <value>
```

设置指定配置项的值。支持自动类型推断。

**类型推断规则：**
- `true` / `false`（不区分大小写）→ 布尔值
- 纯整数（如 `42`、`-5`）→ 整数
- 含小数点的数字（如 `3.14`）→ 浮点数
- 其他 → 字符串

**示例：**
```bash
aide config set task.source "my-task.md"
aide config set general.gitignore_aide true
aide config set decide.port 8080
aide config set plantuml.scale 0.8
```

**特性：**
- 使用 `toml_edit` 保留配置文件中其他位置的注释
- 自动创建不存在的嵌套节

### aide config reset

```bash
aide config reset [--force]
```

重置配置文件到默认值，并自动备份现有配置。

**行为：**
- 备份现有配置到 `.aide/backups/config.toml.{timestamp}`
- 生成新的默认 `config.toml`
- 重新生成 `config.md` 配置说明文档
- 默认显示确认提示，使用 `--force` 跳过

**示例：**
```bash
aide config reset              # 显示确认提示
aide config reset --force      # 跳过确认直接重置
```

### aide config update

```bash
aide config update
```

更新配置文件到最新版本，处理版本升级时的配置迁移。

**行为：**
- 检测配置中的 `meta.schema_version`
- 如版本低于当前版本，执行迁移：
  - 添加新引入的配置项（使用默认值）
  - 注释掉废弃的配置项
  - 保留用户自定义的配置值
- 更新 `meta` 节的版本信息
- 重新生成 `config.md` 配置说明文档

**示例：**
```bash
aide config update
```

---

## aide flow

进度追踪与 git 集成命令组。管理任务的环节流转、步骤追踪和分支操作。

### aide flow start

```bash
aide flow start <phase> <summary>
```

开始一个新任务，初始化流程状态。

**参数：**
- `phase`: 起始环节名（必须是 `flow.phases` 配置中的有效环节）
- `summary`: 任务简要说明

**行为：**
- 归档已有任务状态
- 生成唯一任务 ID（基于时间戳）
- 创建任务分支（从当前 git 分支派生）
- 初始化 `flow-status.json`
- 自动执行 git commit

**示例：**
```bash
aide flow start task-optimize "重构用户认证模块"
```

### aide flow next-step

```bash
aide flow next-step <summary>
```

在当前环节内记录步骤前进。

**参数：**
- `summary`: 本次操作的简要说明

**行为：**
- 步骤编号 +1
- 自动 git commit
- 记录历史

### aide flow back-step

```bash
aide flow back-step <reason>
```

在当前环节内回退一步。

**参数：**
- `reason`: 回退原因

**行为：**
- 步骤编号 -1（最小为 1）
- 自动 git commit
- 记录历史

### aide flow next-part

```bash
aide flow next-part <phase> <summary>
```

进入下一个环节。

**参数：**
- `phase`: 目标环节名（必须是当前环节的相邻下一环节）
- `summary`: 本次操作的简要说明

**校验规则：**
- 目标环节必须是 `flow.phases` 中当前环节的下一个
- 不允许跳过中间环节

**行为：**
- 执行离开当前环节的 post-hooks
- 执行进入目标环节的 pre-hooks
- 步骤重置为 1
- 自动 git commit
- 当目标环节为 `finish` 时，执行分支合并和清理

**示例：**
```bash
aide flow next-part flow-design "任务优化完成，开始流程设计"
```

### aide flow back-part

```bash
aide flow back-part <phase> <reason>
```

回退到之前的环节（两阶段确认机制）。

**参数：**
- `phase`: 目标环节名（必须是当前环节之前的任意环节）
- `reason`: 回退原因

**行为：**
- 生成确认 key 并保存待确认状态
- 输出确认命令提示
- 需要使用 `back-confirm` 完成确认

### aide flow back-confirm

```bash
aide flow back-confirm --key <key>
```

确认环节回退操作。

**参数：**
- `--key`: `back-part` 命令生成的确认 key

**行为：**
- 验证 key 匹配
- 执行回退
- 步骤重置为 1
- 自动 git commit
- 清除待确认状态

### aide flow issue

```bash
aide flow issue <description>
```

记录一般问题（不阻塞继续执行）。

**参数：**
- `description`: 问题描述

### aide flow error

```bash
aide flow error <description>
```

记录严重错误（需要用户关注）。

**参数：**
- `description`: 错误描述

### aide flow status

```bash
aide flow status
```

查看当前任务的状态信息，包括任务 ID、当前环节、步骤编号、开始时间和最近操作。

### aide flow list

```bash
aide flow list
```

列出所有任务记录（包括当前任务和归档任务），按任务 ID 倒序排列。当前活跃任务以 `*` 标记。

### aide flow show

```bash
aide flow show <task_id>
```

查看指定任务的详细状态和完整操作历史。

**参数：**
- `task_id`: 任务 ID

### aide flow clean

```bash
aide flow clean
```

强制清理当前任务状态，删除 `flow-status.json` 和 `back-confirm-state.json`。

---

## aide decide

待定项确认与决策记录命令组。通过 Web 界面让用户对待定项做出选择。

### aide decide submit

```bash
aide decide submit <file> [--web-dir <path>]
```

提交待定项数据并启动 Web 服务。

**参数：**
- `file`: 待定项 JSON 数据文件路径
- `--web-dir`: Web 前端文件目录路径（可选，默认为可执行文件同级 `web/` 目录）

**行为：**
1. 读取并验证 JSON 数据
2. 保存为 `pending.json`
3. 启动后台 HTTP 服务器
4. 输出访问 URL

**示例：**
```bash
aide decide submit pending-items.json
aide decide submit items.json --web-dir /path/to/custom/web
```

### aide decide result

```bash
aide decide result
```

获取用户决策结果。

**行为：**
- 检查是否有待定项数据
- 查找对应的决策结果文件
- 以 JSON 格式输出结果到标准输出
- 清理服务器信息

**输出格式（JSON）：**
```json
{
  "decisions": [
    {
      "id": 1,
      "chosen": "option_value",
      "note": "用户备注"
    }
  ]
}
```

**退出码：**
- 0: 成功获取结果
- 1: 无待定项数据、用户未完成决策、或数据异常

---

## 通用说明

### 项目根目录发现

所有命令（除 `init` 外）使用三阶段向上搜索算法查找项目根目录：

1. 当前目录有 `.aide/` 目录 → 直接使用
2. 向上搜索包含 `.aide/flow-status.json` 的目录（活跃任务优先）
3. 向上搜索包含 `.aide/config.toml` 的目录
4. 兜底：使用当前目录

### 退出码

- 0: 命令执行成功
- 1: 命令执行失败（错误信息输出到 stderr）
