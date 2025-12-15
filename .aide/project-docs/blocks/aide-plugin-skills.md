# aide-plugin-skills

> 路径：aide-marketplace/aide-plugin/skills/
> 最后更新：2025-12-15

## 概述

Aide 插件的技能定义，提供专门知识供 LLM 按需学习。

## 文件清单

| 目录 | 说明 |
|------|------|
| `aide/SKILL.md` | aide 基础命令指南（始终加载） |
| `env-config/SKILL.md` | 环境配置详细指南（按需触发） |

## 技能说明

### aide skill

- **触发方式**：始终加载
- **内容**：
  - aide 命令行工具的完整使用指南
  - 所有子命令的参数说明和示例
  - 输出格式说明（✓/⚠/✗/→）
  - 数据存储结构
  - 常见用法示例

### env-config skill

- **触发方式**：当 `aide env ensure` 失败时按需触发
- **内容**：
  - 环境模块详细配置方法
  - 各模块的配置项说明
  - 多项目场景处理
  - 故障排除指南

## 设计理念

**Skill 拆分原则**：
- `aide` skill：基础知识，始终需要
- `env-config` skill：专门知识，仅在配置环境时需要

这样设计的好处：
1. 减少 LLM 上下文占用
2. 按需加载专门知识
3. 职责分离，便于维护

## 依赖关系

- 被依赖：commands（setup, load, docs, run）

## 注意事项

- Skill 文件是 Markdown 格式
- 内容应聚焦于 LLM 执行任务所需的知识
- 避免冗余，保持简洁
