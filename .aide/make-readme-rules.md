# README 编写规范

## 基础模板

模板：monorepo

## 启用模块

- quickstart（快速开始）
- contributing（贡献指南）
- license（许可证）
- architecture（架构概述）

## 许可证

类型：Apache 2.0

## 自定义要求

### 目标定位

- 面向 GitHub 开源社区
- 打造优秀开源项目形象
- 吸引潜在用户和贡献者

### README 结构规范

README 整体结构应遵循以下顺序，确保用户能快速上手并了解命令使用：

#### 第一部分：快速入门（简洁）

1. **项目介绍** - 一句话说明 + 核心价值（2-3 行）
2. **安装** - 克隆、创建虚拟环境、安装依赖（简洁步骤）
3. **初始化** - `aide init` 一步完成

#### 第二部分：命令使用指南（重点）

这是 README 的核心部分，需要详细说明：

1. **命令体系概览**
   - 展示 8 个斜杠命令的关系和使用场景
   - 使用流程图 `docs/graph-guide/aide-marketplace/commands.png` 辅助说明

2. **命令使用场景对照表**
   | 场景 | 推荐命令 | 说明 |
   |------|----------|------|
   | 首次使用/环境问题 | `/aide:setup` | 环境配置和检测 |
   | 了解项目结构 | `/aide:load` | 载入项目认知 |
   | 生成项目文档 | `/aide:docs` | 创建 LLM 友好的项目文档 |
   | 执行开发任务 | `/aide:run` | 核心工作流命令 |
   | 全自动执行 | `/aide:auto-run` | 减少交互的自动模式 |
   | 生成 README | `/aide:readme` | 本命令 |
   | 生成用户文档 | `/aide:user-docs` | 面向最终用户的文档 |
   | 生成流程图 | `/aide:user-graph` | 可视化项目架构 |

3. **典型工作流示例**
   - 新项目入门流程：`/aide:setup` → `/aide:docs` → `/aide:load`
   - 任务执行流程：`/aide:run`（含环节说明）
   - 文档维护流程：`/aide:readme` + `/aide:user-docs` + `/aide:user-graph`

#### 第三部分：深入了解（引导）

1. **流程图索引**
   - 列出 `docs/graph-guide/` 下的所有流程图
   - 分类展示：aide-program（6 个）、aide-marketplace（3 个）、project-config-docs（1 个）
   - 每个流程图附简要说明和图片预览

2. **详细文档链接**
   - `docs/reference/` 下的参考文档
   - 各模块的详细说明

3. **架构概述**
   - 使用 `docs/graph-guide/aide-program/guide.png` 展示整体架构
   - 简要说明核心组件

#### 第四部分：贡献与许可（保持简洁）

- 贡献指南要点
- 许可证说明

### 流程图引用规范

在 README 中引用流程图时：

```markdown
### 命令执行流程

![Commands Flow](docs/graph-guide/aide-marketplace/commands.png)

> 查看完整流程图：[docs/graph-guide/](docs/graph-guide/)
```

### 关键流程图清单

| 流程图 | 路径 | 用途 |
|--------|------|------|
| 整体架构 | `aide-program/guide.png` | 展示系统架构 |
| 命令流程 | `aide-marketplace/commands.png` | 说明命令使用 |
| 技能体系 | `aide-marketplace/skills.png` | 说明技能关系 |
| 环境检测 | `aide-program/env.png` | 环境管理流程 |
| 流程追踪 | `aide-program/flow.png` | 任务追踪流程 |
| 待定项确认 | `aide-program/decide.png` | 交互决策流程 |

### 子项目说明

| 子项目 | 路径 | 定位 |
|--------|------|------|
| aide-program | `aide-program/` | 核心 CLI 工具实现（Python） |
| aide-marketplace | `aide-marketplace/` | Claude Code 插件市场 |

### 徽章建议

- Python 版本
- 许可证
- GitHub Stars
- 最新版本

## 生成时间

2025-12-19（规范更新）
