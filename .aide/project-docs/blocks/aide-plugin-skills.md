# aide-plugin-skills

> 路径：aide-marketplace/aide-plugin/skills/
> 最后更新：2025-12-17

## 概述

Aide 插件的技能定义。每个技能是一个独立目录，包含 SKILL.md 文件作为技能知识库。技能通过 Skill 工具触发，为 LLM 提供专业领域的知识和指导。

## 目录结构

```
aide-marketplace/aide-plugin/skills/
├── aide/
│   └── SKILL.md             Aide 命令使用指南
├── env-config/
│   └── SKILL.md             环境配置详细指南
└── task-parser/
    └── SKILL.md             口语化内容解析指南
```

## 文件清单

| 文件 | 类型 | 说明 |
|------|------|------|
| aide/SKILL.md | 技能 | Aide CLI 工具完整使用指南 |
| env-config/SKILL.md | 技能 | 环境配置和故障排除指南 |
| task-parser/SKILL.md | 技能 | 口语化任务内容解析方法 |

## 技能详解

### aide - Aide 工具使用指南

- **触发描述**：Aide 工作流工具集。提供环境管理、进度追踪、待定项确认等功能。
- **内容覆盖**：
  - `aide env` - 环境管理命令
  - `aide flow` - 进度追踪命令
  - `aide decide` - 待定项确认命令
  - `aide config` - 配置管理命令
  - `aide init` - 初始化命令
  - 数据存储结构
  - 常见用法示例

### env-config - 环境配置指南

- **触发描述**：环境配置详细指南。由 `/aide:setup` 命令强制触发。
- **触发条件**：当 `aide env ensure` 检测失败时
- **内容覆盖**：
  - 问题诊断（常见失败原因）
  - 模块分类（类型A/类型B）
  - 配置命令使用
  - 项目类型配置示例
  - 多项目场景处理（模块实例化命名）
  - node_deps 模块详解
  - 故障排除

### task-parser - 口语化内容解析器

- **触发描述**：口语化任务内容解析器。当发现用户对话或任务文档具有明显口头语气时使用。
- **触发条件**：内容具有口语化特征（模糊表述、松散结构、思维跳跃等）
- **解析流程**：
  1. 语义解析（表层理解、深层提取、结构重组）
  2. 批判性分析（逻辑漏洞、盲点识别、过度设计识别）
  3. 建设性优化（优化建议、方案对比、风险权衡）
  4. 上下文关联分析（项目关联、隐含需求、复杂度预判）

## 接口说明

### 技能文件格式

每个 SKILL.md 文件包含 YAML 前置元数据：

```yaml
---
name: 技能名称
description: 技能描述（用于触发匹配）
---

# 技能内容
...
```

### 技能触发方式

技能通过 Skill 工具自动触发：
- LLM 根据任务场景匹配合适的技能
- 触发后技能内容作为上下文提供给 LLM

## 依赖关系

- 被依赖：aide-plugin commands（命令中引用技能）

## 注意事项

- 技能文件使用 Markdown 格式，内容作为 LLM 知识库
- YAML 前置元数据中的 description 用于技能匹配
- 技能应专注于单一领域，提供完整但聚焦的指导
- 技能之间可以相互引用（如 aide skill 引用 env-config skill）
