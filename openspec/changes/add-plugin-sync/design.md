# Design: add-plugin-sync

## Overview

本变更在 `aide init` 流程中新增插件同步能力，分为全局初始化（`--global`）和项目初始化两个阶段。

## Architecture

### 全局初始化流程（`aide init --global`）

```
┌─────────────────────────────────────────────────────────────┐
│                    aide init --global                        │
├─────────────────────────────────────────────────────────────┤
│  1. 确保全局配置存在（现有逻辑）                               │
│  2. 检测 Git 可用性                                           │
│     └─ 不可用 → 警告并跳过插件同步                             │
│  3. 读取 plugin.repo_url 配置（无则使用默认值）                 │
│  4. 检查 ~/.aide/agent-aide/ 是否存在                         │
│     ├─ 不存在 → git clone                                     │
│     └─ 已存在 → git pull（更新）                              │
│  5. PlantUML 检测与安装（现有逻辑）                            │
└─────────────────────────────────────────────────────────────┘
```

### 项目初始化流程（`aide init`）

```
┌─────────────────────────────────────────────────────────────┐
│                      aide init                               │
├─────────────────────────────────────────────────────────────┤
│  1. 全局配置检查与创建（现有逻辑）                             │
│  2. 项目目录结构创建（现有逻辑）                               │
│  3. 检查 ~/.aide/agent-aide/aide-plugin/ 是否存在             │
│     └─ 不存在 → 警告并跳过插件同步                             │
│  4. 创建项目 .claude/ 目录                                    │
│  5. 复制 commands/ 到 .claude/commands/                       │
│  6. 复制 skills/ 到 .claude/skills/                           │
│  7. 更新 .gitignore（现有逻辑）                               │
└─────────────────────────────────────────────────────────────┘
```

## Configuration Schema

新增 `[plugin]` 配置段：

```toml
[plugin]
repo_url = "git@github.com:sayurinana/agent-aide.git"
sync_on_init = true
```

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `repo_url` | String | `git@github.com:sayurinana/agent-aide.git` | agent-aide 仓库地址 |
| `sync_on_init` | Boolean | `true` | 项目初始化时是否同步插件 |

### 配置优先级

1. 优先读取配置文件中的 `plugin.repo_url`
2. 若配置文件中无此配置项，使用程序硬编码的默认值

## Directory Structure

### 全局目录（~/.aide/）

```
~/.aide/
├── config.toml
├── agent-aide/              # 克隆的仓库
│   ├── aide-plugin/
│   │   ├── commands/        # 复制源
│   │   └── skills/          # 复制源
│   └── ...
└── utils/
    └── plantuml/
```

### 项目目录

```
project/
├── .claude/
│   ├── commands/            # 从全局复制
│   └── skills/              # 从全局复制
├── aide-memory/
└── .gitignore
```

## Error Handling

| 场景 | 行为 |
|------|------|
| Git 未安装 | 输出警告，跳过插件同步，继续其他初始化 |
| 网络不可达 | 输出错误，提示检查网络，允许跳过 |
| 仓库已存在但更新失败 | 输出警告，使用现有版本继续 |
| 目标目录非 git 仓库 | 输出警告，删除后重新克隆 |
| commands/skills 目录不存在 | 输出警告，跳过对应同步步骤 |

## Trade-offs

### 选择 git clone 而非下载压缩包

**优点**：
- 支持增量更新（git pull）
- 版本可追溯
- 便于调试和贡献

**缺点**：
- 依赖 git 命令
- 首次克隆较慢

**决策**：采用 git clone，因为 agent-aide 仓库体积不大，且版本控制能力对开发者更有价值。

### 复制而非符号链接

**优点**：
- 跨平台兼容性好
- 项目可独立修改

**缺点**：
- 占用额外磁盘空间
- 更新需重新执行 init

**决策**：采用复制方式，避免符号链接在 Windows 等系统的兼容性问题。

## Future Extensions

1. **版本锁定**：支持指定克隆的 tag 或 commit
2. **多源支持**：支持配置多个插件源
3. **选择性同步**：允许用户选择要同步的 commands 和 skills
4. **更新检测**：检测远程仓库是否有更新并提示用户