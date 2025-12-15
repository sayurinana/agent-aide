# CHANGELOG

本文件记录 Aide 项目对使用者可见的重要变更。

## 2025-12-15 (v2.0.0)

### 新增功能

**Commands 重组**
- 新增 `/aide:setup` - 独立的环境配置命令
- 新增 `/aide:load` - 项目文档按需载入命令
- 新增 `/aide:docs` - 项目文档创建和维护命令
- 新增 `/aide:run` - 整合任务准备和执行的核心命令（替代原 prep + exec）

**aide flow 增强**
- 新增 `aide flow status` - 查看当前任务状态
- 新增 `aide flow list` - 列出所有任务（当前 + 归档）
- 新增 `aide flow show <task_id>` - 查看指定任务的详细状态历史

**配置系统增强**
- 配置文件完全自文档化（所有配置项含详细注释说明）
- 新增 `[general]` 节：`gitignore_aide` 配置是否忽略 .aide 目录
- 新增 `[docs]` 节：项目文档路径配置
- 新增 `[flow]` 节：`diagram_path` 流程图目录配置
- 新增 `[plantuml]` 节：PlantUML jar 路径配置

**PlantUML 集成**
- 内置 plantuml.jar，支持本地流程图校验和构建
- flow-design 阶段自动校验 PlantUML 语法
- 进入 impl 阶段时自动生成 PNG 图片

**项目文档体系**
- 设计面向 LLM 的区块化文档结构
- 支持总导览 + 多子区块的文档组织
- 支持增量更新和多对话续接

**任务分析增强**
- 制定任务复杂度评估指导原则
- 支持复杂任务拆分为多个子计划
- 子计划循环执行机制

### 变更

- 原 `/aide:init` 重命名为 `/aide:setup`
- 原 `/aide:prep` 和 `/aide:exec` 合并为 `/aide:run`
- 旧命令文件保留为 `_deprecated_*.md` 供参考

## 2025-12-14

- 实现 `aide flow`（进度追踪 + Git 自动提交 + 流程校验 + Hooks）
- 补充 `aide flow` 详细设计文档与导航链路
