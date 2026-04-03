# 任务解析结果（已确认）

## 原始内容概述

用户要求为 agent-aide 工具添加两个功能：
1. 从远程 Git 仓库拉取最新仓库数据的功能
2. 在 aide init 时从远程仓库的 templates 目录复制模板文件到项目中

## 核心意图

增强 aide 工具的远程数据同步能力，使项目初始化能够获取最新的插件和模板资源。

## 结构化任务描述

### 目标

完善 aide 工具的远程仓库同步机制，确保项目能够获取最新的模板文件和插件资源。

### 具体要求

#### 要求 1：新增 `aide sync` 命令

**用户确认**：采用独立命令 `aide sync` 方案。

**实现要点**：
- 命令名：`aide sync`
- 功能：克隆或更新 `~/.aide/agent-aide/` 仓库
- 输出：成功/失败信息，遵循静默成功原则
- 失败处理：显示错误信息，返回非零退出码
- 仓库地址：使用 HTTPS 协议 `https://github.com/sayurinana/agent-aide.git`

#### 要求 2：在 aide init 时同步模板文件

**用户确认**：
- 来源：全局仓库 `~/.aide/agent-aide/templates/`
- 目标：项目 `aide-memory/templates/`
- 全局仓库已包含 templates 目录（已验证）

**文件已存在时的处理策略**（配置项控制）：

| 策略 | 配置值 | 行为 |
|------|--------|------|
| 仅下载备份（默认） | `backup` | 下载为 `.bak` 文件，用户自行查阅并编辑 |
| 跳过 | `skip` | 保持现有文件不变 |
| 覆盖 | `overwrite` | 直接覆盖现有文件 |
| 备份后新建 | `backup-and-replace` | 现有文件备份后用新文件替换 |

**配置项设计**：
- 配置键：`template.sync_strategy`
- 默认值：`backup`
- 可选值：`backup`、`skip`、`overwrite`、`backup-and-replace`

#### 要求 3：更新默认仓库地址

**用户确认**：默认仓库地址改为 HTTPS 协议。

- 原地址：`git@github.com:sayurinana/agent-aide.git`（SSH）
- 新地址：`https://github.com/sayurinana/agent-aide.git`（HTTPS）

### 约束条件

- 不破坏现有功能
- 遵循静默成功原则（操作成功时默认不产生输出）
- 使用统一输出符号（✓、⚠、✗、→）
- 遵循现有代码风格和命名规范

### 期望产出

- `aide sync` 命令实现（Rust 代码）
- 模板同步逻辑集成到 `aide init`
- 配置项新增：`template.sync_strategy`
- 默认仓库地址更新为 HTTPS
- 完整的测试覆盖

## 实现范围

### 涉及文件

| 文件 | 修改内容 |
|------|----------|
| `aide/src/main.rs` | 新增 `Sync` 命令定义 |
| `aide/src/cli/init.rs` | 新增模板同步逻辑、调整 `sync_plugins_to_project` |
| `aide/src/core/config.rs` | 更新 `DEFAULT_PLUGIN_REPO_URL`、新增配置说明 |
| `aide/src/cli/sync.rs` | 新增文件，实现 sync 命令处理 |

### 数据结构

**模板同步策略配置**：
```toml
[template]
sync_strategy = "backup"  # backup | skip | overwrite | backup-and-replace
```

## 复杂度评估

**中等复杂度**

- 结构复杂度：中等（新增 1 个命令 + 1 个配置项）
- 逻辑复杂度：中等（四种同步策略逻辑）
- 集成复杂度：中等（与现有 init 流程集成）
- 风险等级：低（可逆操作，不影响核心功能）

**建议处理方式**：在单个提案中实现，分步骤完成。

## 状态

✅ 已确认，可开始创建提案