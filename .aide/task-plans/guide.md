# 任务计划总导览

## 任务简述

对 Aide 工作流体系进行多项调整：基础配置修改、面向用户的文档系统（3个命令）、返工流程优化。

## 用户决策记录

| # | 待定项 | 选择 | 备注 |
|---|--------|------|------|
| 1 | 命令拆分方案 | 3个命令 | `/aide:readme` + `/aide:user-docs` + `/aide:user-graph` |
| 2 | 返工确认机制 | 完整 key 确认 | `aide flow back-confirm --key xxx` |
| 3 | 配置项命名 | user_docs 命名空间 | `user_docs.*` |
| 4 | README 已存在处理 | 直接覆盖 | 不需备份，git 已提供版本控制 |

## 子计划列表

| # | 名称 | 状态 | 细则文档 | 说明 |
|---|------|------|----------|------|
| 1 | 基础配置修改 | pending | spec-01.md | gitignore_aide 默认值、aide init 任意目录 |
| 2 | README skill 模板集 | pending | spec-02.md | 创建 readme-templates skill |
| 3 | README 命令实现 | pending | spec-03.md | /aide:readme 命令 |
| 4 | 用户文档命令实现 | pending | spec-04.md | /aide:user-docs 命令 |
| 5 | 用户流程图命令实现 | pending | spec-05.md | /aide:user-graph 命令 |
| 6 | 返工流程优化 | pending | spec-06.md | rework skill + 程序修改 |

## 执行顺序

```
1 (基础配置) ─────────────────────────────────────┐
                                                  │
2 (README skill) ──→ 3 (readme 命令) ─────────────┼──→ 完成
                          │                       │
                          ↓                       │
                     4 (user-docs 命令)           │
                          │                       │
                          ↓                       │
                     5 (user-graph 命令) ─────────┤
                                                  │
6 (返工优化) ─────────────────────────────────────┘
```

**依赖说明**：
- 子计划 1、2、6 可并行执行（无相互依赖）
- 子计划 3 依赖子计划 2（需要 skill 模板）
- 子计划 4 依赖子计划 3（基于 readme 命令扩展）
- 子计划 5 依赖子计划 4（流程图与用户文档关联）

## 涉及模块

| 模块 | 路径 | 涉及子计划 |
|------|------|-----------|
| aide-program | `aide-program/aide/` | 1, 3, 4, 5, 6 |
| commands | `aide-marketplace/aide-plugin/commands/` | 3, 4, 5, 6 |
| skills | `aide-marketplace/aide-plugin/skills/` | 2, 6 |

## 配置项规划

```toml
[user_docs]
# README 文件路径
readme_path = "README.md"

# 用户文档目录
docs_path = "docs"

# README 编写规范文件
rules_path = "make-readme-rules.md"

# 用户流程图目录
graph_path = "docs/graph-guide"
```

## 备注

- 这是一个超大型任务，预计需要多个对话周期完成
- 子计划 5（用户流程图）复杂度最高，可能需要进一步细分
- 建议每完成一个子计划后进行 confirm，确保方向正确
