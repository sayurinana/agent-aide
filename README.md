# Aide - Claude Code 工作流辅助体系

Aide 是一套面向 Claude Code 的工作流辅助体系，通过模块化的 Commands、Skills 和命令行工具，帮助你更高效地进行 AI 辅助开发。

## 快速上手

### 1. 安装 Aide 插件

在 Claude Code 中添加本市场的插件：

```bash
# 进入 Claude Code 配置目录
cd ~/.claude

# 编辑 settings.json，在 marketplaces 数组中添加：
```

```json
{
  "marketplaces": [
    "https://path/to/aide-marketplace"
  ]
}
```

或者直接在 Claude Code 中运行：

```
/install-plugin aide-marketplace/aide-plugin
```

### 2. 安装 aide 命令行工具

aide 命令行工具提供环境管理、进度追踪等功能。

**前置要求**：
- Python >= 3.11
- uv 包管理器

**安装步骤**：

```bash
# 克隆仓库
git clone <repo-url> ccoptimize
cd ccoptimize/aide-program

# 创建虚拟环境并安装依赖
uv venv .venv
uv pip install -r requirements.txt

# 将 bin 目录添加到 PATH（可选）
export PATH="$PWD/bin:$PATH"

# 或者直接使用完整路径
./bin/aide --help
```

### 3. 开始使用

在你的项目中初始化 Aide：

```bash
# 初始化 .aide 目录
aide init

# 检查环境
aide env ensure --runtime
```

然后在 Claude Code 中使用 Aide 命令：

```
/aide:run          # 执行任务（核心命令）
/aide:setup        # 环境配置
/aide:docs/reference         # 项目文档管理
```

## 核心功能

| 命令 | 说明 |
|------|------|
| `/aide:run` | 任务执行（核心命令），整合任务准备和执行流程 |
| `/aide:setup` | 环境配置，检测和修复开发环境 |
| `/aide:docs/reference` | 项目文档创建和维护 |
| `/aide:load` | 项目认知载入（由 run 内部调用） |

## aide 命令行工具

```bash
aide init              # 初始化 .aide 目录
aide env ensure        # 环境检测与修复
aide env list          # 列出可用模块
aide config get <key>  # 获取配置值
aide config set <key> <value>  # 设置配置值
aide flow status       # 查看任务状态
aide flow list         # 列出所有任务
aide decide submit     # 提交待定项确认
aide decide result     # 获取决策结果
```

## 文档

- [项目详细说明](docs/reference/project-details.md) - 项目架构和实现状态
- [Aide 系统概述](docs/reference/aide-overview.md) - 系统设计理念
- [aide-plugin 文档](aide-marketplace/aide-plugin/docs/reference/README.md) - 插件设计
- [aide-program 文档](aide-program/docs/reference/README.md) - 命令行工具设计

## 许可证

MIT
