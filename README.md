# Aide

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Python](https://img.shields.io/badge/Python-3.11+-green.svg)](https://www.python.org/)

**面向 LLM 驱动开发的工作流工具**

Aide 是一套为 Claude Code 设计的工作流辅助体系，通过结构化的任务管理、进度追踪和交互式决策，让 AI 辅助开发更加高效可控。

## 核心特性

- **任务进度追踪** - 自动记录开发进度，与 Git 深度集成
- **环境智能检测** - 支持 Python/Node/Rust/Flutter 等多种环境
- **交互式决策** - Web 界面处理待定项确认，让 AI 遵循你的选择
- **渐进式披露** - 按需加载信息，避免上下文过载
- **Claude Code 插件** - 8 个斜杠命令 + 5 个技能定义

## 项目结构

```
agent-aide/
├── aide-program/          # 核心 CLI 工具（Python）
│   ├── aide/              # 源码
│   │   ├── core/          # 配置管理
│   │   ├── env/           # 环境检测
│   │   ├── flow/          # 流程追踪
│   │   └── decide/        # 待定项确认
│   └── bin/               # 可执行脚本
├── aide-marketplace/      # Claude Code 插件市场
│   └── aide-plugin/       # Aide 插件
│       ├── commands/      # 斜杠命令（8 个）
│       └── skills/        # 技能定义（5 个）
└── docs/                  # 项目文档
```

## 子项目

| 项目 | 路径 | 说明 | 技术栈 |
|------|------|------|--------|
| aide-program | `aide-program/` | 核心命令行工具，提供环境检测、流程追踪、待定项确认 | Python 3.11+ |
| aide-plugin | `aide-marketplace/aide-plugin/` | Claude Code 插件，提供工作流命令和技能 | Markdown |

## 快速开始

只需 3 步，即可体验 Aide 的核心功能：

### 1. 安装

```bash
# 克隆仓库
git clone https://github.com/your-username/agent-aide.git
cd agent-aide/aide-program

# 创建虚拟环境并安装依赖
uv venv .venv
source .venv/bin/activate
uv pip install -r requirements.txt
```

### 2. 初始化

```bash
# 在你的项目目录中
aide init
```

### 3. 运行

```bash
# 检测环境
aide env ensure --runtime

# 开始任务追踪
aide flow start task-optimize "开始任务: 实现新功能"
```

**预期输出**：

```
✓ python: 3.11.x (>=3.11)
✓ uv: 0.x.x
✓ 环境就绪

✓ 任务已开始: 2025-12-19T10-00-00
✓ Git 提交: abc1234
```

### 下一步

- [完整使用指南](#使用说明)
- [Claude Code 插件安装](#安装-aide-插件)
- [命令参考](#命令参考)

## 使用说明

### aide 命令行工具

#### 环境管理

```bash
aide env ensure              # 检测并修复环境
aide env ensure --runtime    # 仅检测 aide 运行时
aide env list                # 列出可用模块
aide env set modules python,venv  # 设置启用模块
```

#### 流程追踪

```bash
aide flow start <phase> "<summary>"   # 开始新任务
aide flow next-step "<summary>"       # 步骤前进
aide flow next-part <phase> "<summary>"  # 环节前进
aide flow status                      # 查看当前状态
aide flow list                        # 列出所有任务
```

#### 待定项确认

```bash
aide decide submit <file.json>   # 提交待定项，启动 Web 服务
aide decide result               # 获取用户决策结果
```

#### 配置管理

```bash
aide config get <key>        # 获取配置值
aide config set <key> <value>  # 设置配置值
```

### 安装 Aide 插件

在 Claude Code 中添加 Aide 市场：

1. 编辑 `~/.claude/settings.json`：

```json
{
  "marketplaces": [
    "/path/to/agent-aide/aide-marketplace"
  ]
}
```

2. 使用 Aide 命令：

```
/aide:run          # 任务执行（核心命令）
/aide:auto-run     # 全自动任务执行
/aide:setup        # 环境配置
/aide:docs         # 项目文档管理
/aide:readme       # README 生成
```

## 架构

### 系统概览

Aide 采用分层架构设计，将关注点分离：

```
┌─────────────────────────────────────────────────────────┐
│                    Claude Code                          │
├─────────────────────────────────────────────────────────┤
│  aide-plugin                                            │
│  ├── Commands (定义"做什么")                            │
│  │   └── /aide:run, /aide:setup, /aide:docs...         │
│  └── Skills (定义"怎么做")                              │
│      └── aide, env-config, readme-templates...          │
├─────────────────────────────────────────────────────────┤
│  aide-program (CLI)                                     │
│  ├── env/    环境检测                                   │
│  ├── flow/   流程追踪 + Git 集成                        │
│  └── decide/ 待定项确认 (Web UI)                        │
└─────────────────────────────────────────────────────────┘
```

### 核心组件

| 组件 | 职责 | 位置 |
|------|------|------|
| ConfigManager | 配置管理，维护 .aide/ 目录 | `aide/core/config.py` |
| EnvManager | 环境检测与修复 | `aide/env/manager.py` |
| FlowTracker | 流程追踪，协调 Git 提交 | `aide/flow/tracker.py` |
| DecideServer | 待定项 Web 服务 | `aide/decide/server.py` |

### 设计原则

1. **渐进式披露** - 按需加载信息，避免上下文过载
2. **确定性封装** - 工具调用产生确定性输出
3. **信息隔离** - Commands 定义流程，Skills 定义工具用法

### 工作流程

```
task-optimize → flow-design → impl → verify → docs → confirm → finish
      │              │
      ├─ 任务分析     ├─ 流程图设计
      ├─ 复杂度评估   └─ PlantUML 校验
      └─ 待定项处理
```

## 命令参考

### aide-program CLI

| 命令 | 说明 |
|------|------|
| `aide init` | 初始化 .aide 目录 |
| `aide env ensure` | 检测并修复环境 |
| `aide env list` | 列出可用模块 |
| `aide flow start` | 开始新任务 |
| `aide flow status` | 查看任务状态 |
| `aide decide submit` | 提交待定项 |
| `aide config get/set` | 配置管理 |

### aide-plugin Commands

| 命令 | 说明 |
|------|------|
| `/aide:run` | 任务执行（核心命令） |
| `/aide:auto-run` | 全自动任务执行 |
| `/aide:setup` | 环境配置 |
| `/aide:docs` | 项目文档管理 |
| `/aide:load` | 项目认知载入 |
| `/aide:readme` | README 生成 |
| `/aide:user-docs` | 用户文档生成 |
| `/aide:user-graph` | 用户流程图生成 |

## 贡献指南

感谢你考虑为 Aide 做贡献！

### 贡献方式

- 报告 Bug
- 提交功能建议
- 改进文档
- 提交代码 PR

### 开发环境

#### 环境要求

- Python >= 3.11
- uv 包管理器
- Git

#### 环境搭建

```bash
# 克隆仓库
git clone https://github.com/your-username/agent-aide.git
cd agent-aide

# 安装 aide-program 依赖
cd aide-program
uv venv .venv
source .venv/bin/activate
uv pip install -r requirements.txt

# 运行测试
python -m pytest
```

### 代码规范

- 使用 Python 3.11+ 语法特性
- 遵循 PEP 8 编码规范
- 保持函数简洁，单一职责

#### 提交信息格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

**type 类型**：
- `feat`: 新功能
- `fix`: 修复 Bug
- `docs`: 文档更新
- `style`: 代码格式
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具

### 提交 PR

1. Fork 本仓库
2. 创建特性分支：`git checkout -b feature/your-feature`
3. 提交更改：`git commit -m 'feat: add some feature'`
4. 推送分支：`git push origin feature/your-feature`
5. 创建 Pull Request

### PR 检查清单

- [ ] 代码通过所有测试
- [ ] 新功能有对应测试
- [ ] 更新了相关文档
- [ ] 提交信息格式正确

## 许可证

本项目采用 Apache License 2.0 许可证。

### 简要说明

Apache 2.0 是一个宽松的开源许可证，允许商业使用、修改和分发。

### 你可以

- 商业使用
- 修改代码
- 分发副本
- 私有使用

### 你必须

- 保留版权声明
- 声明重大修改
- 包含许可证副本

查看 [LICENSE](LICENSE) 文件获取完整许可证文本。

---

**Made with ❤️ for LLM-driven development**
