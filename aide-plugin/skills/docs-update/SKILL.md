---
name: docs-update
description: 文档更新指南。在 review 完成后、进入 confirm 前执行（当 build-task 阶段选定本阶段时），用于更新项目文档（README、API文档等）、同步 memory 全局信息、更新 concepts/ 和 structure/ 目录内容。
---

# docs-update 指南

## 目标

在任务实施完成后，更新项目文档以反映最新变化，同步 memory 全局信息，确保项目的可读性和可维护性。

## 进入前先读

- `information.md` — 任务目标与边界
- `design.md` — 方案细节
- `todo.md` — 已完成的任务点
- `task-summary.md` — 实施摘要
- memory 目录下的相关文档

## 更新范围

### 1. 项目文档

根据任务影响更新：

| 文档类型 | 更新时机 |
|----------|----------|
| README.md | 功能变更、使用方式变化 |
| CHANGELOG.md | 版本发布、重要变更 |
| API 文档 | 接口新增/修改/废弃 |
| 配置文档 | 配置项变更 |
| 架构文档 | 结构调整、依赖变化 |

### 2. memory 全局信息

同步更新 aide-memory 目录：

| 目录 | 更新内容 |
|------|----------|
| `memory/structure/` | 新增/修改的文件结构概述 |
| `memory/concepts/` | 新增/修改的概念、术语 |
| `memory/diagram/` | 新增/修改的架构图解 |
| `memory/overview.md` | 导览更新 |

## 更新流程

### 1. 确定影响范围

分析本次任务的改动，确定需要更新的文档：

- 新增了哪些功能/模块
- 修改了哪些接口/配置
- 影响了哪些现有文档

### 2. 更新项目文档

逐一更新受影响的文档：

```markdown
## 文档更新记录

- [x] README.md 添加新功能说明
- [x] CHANGELOG.md 记录本次变更
- [x] API 文档更新接口参数
```

### 3. 更新 memory 全局信息

同步 memory 目录：

#### structure/ 目录

更新或新增文件结构概述：

```markdown
# 新增模块

## src/auth/

新增用户认证模块，包含：
- login.py：登录逻辑
- permission.py：权限检查
```

#### concepts/ 目录

更新概念和术语：

```markdown
# 术语更新

## JWT Token

新增 JWT 认证机制，用于用户会话管理。
```

#### overview.md

更新导览文档，反映最新项目状态。

### 4. 完成确认

文档更新完成后：

- 更新 task-summary.md 记录文档更新结论
- 确认所有受影响文档已更新
- 进入 confirm 阶段

## 与其他阶段的关系

```
impl-verify → integration → review → docs-update → confirm
```

docs-update 是可选阶段，仅在 build-task 阶段选定时执行。

通常在 review 之后执行，确保审查通过后再更新文档。

## 更新原则

### 及时性

改动完成后及时更新文档，避免遗忘。

### 准确性

文档内容与实际实现保持一致。

### 简洁性

只更新必要内容，避免冗余。

### 可追溯性

重要变更记录时间、版本、原因。

## memory 更新细节

### structure/index.md

维护完整的目录结构索引：

- 新增目录时添加条目
- 修改结构时更新描述
- 废弃模块时标注状态

### concepts/term.md

术语表更新：

- 新增术语添加定义
- 修改含义更新描述
- 废弃术语标注废弃

### concepts/arch.md

架构描述更新：

- 模块关系变化时更新
- 依赖变化时更新
- 设计决策变化时补充说明

## 完成条件

docs-update 阶段完成时，应满足：

- 受影响的项目文档已更新
- memory 目录已同步
- task-summary.md 已记录更新结论
- 文档内容与实现一致
- 明确进入 confirm 阶段