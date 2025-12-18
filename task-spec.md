# 任务细则：调整 commands/skills/aide-program 体系

## 任务概述

对现有的 commands、skills 和 aide-program 体系做两项调整：
1. finish 环节添加任务计划文件清理功能
2. 流程图编写添加字体、DPI、缩放配置支持

## 任务 1：finish 环节清理任务计划文件

### 目标

在 finish 环节更新状态文件之后，添加对任务计划文件的清理。

### 具体步骤

1. 修改 `aide-marketplace/aide-plugin/commands/run.md`
   - 在「阶段 7：收尾 (finish)」部分添加清理任务计划文件的指导
   - 清理范围：`task.plans_path` 配置的目录下的所有文件（guide.md, spec-*.md 等）
   - 需要先通过 `aide config get task.plans_path` 获取路径

### 验证标准

- run.md 的 finish 环节包含清理任务计划文件的明确指导
- 指导中包含如何获取配置路径

## 任务 2：流程图配置优化

### 目标

保证编写的 PlantUML 带有字体、DPI、缩放配置信息，这些值从 aide 环境配置中获取。

### 具体步骤

1. 修改 `aide-program/aide/core/config.py` 中的 DEFAULT_CONFIG
   - 在 `[plantuml]` 节添加三个新配置项：
     - `font_name = "Arial"` - 默认字体
     - `dpi = 300` - DPI 值
     - `scale = 0.5` - 缩放系数
   - 添加相应的注释说明

2. 修改 `aide-marketplace/aide-plugin/commands/run.md`
   - 在「2.2 创建流程图」→「流程图示例结构」部分添加配置获取和使用说明
   - 示例结构需包含：
     ```plantuml
     skinparam defaultFontName "Arial"
     skinparam dpi 300
     scale 0.5
     ```
   - 说明这些值需要通过 `aide config get plantuml.font_name` 等命令获取

### 验证标准

- config.py 的 DEFAULT_CONFIG 包含三个新配置项
- run.md 的流程图示例包含配置获取说明和示例代码
- 配置默认值正确：Arial、300、0.5

## 文件变更清单

| 文件 | 变更类型 | 说明 |
|------|----------|------|
| `aide-program/aide/core/config.py` | 修改 | 添加 plantuml 配置项 |
| `aide-marketplace/aide-plugin/commands/run.md` | 修改 | 更新 finish 和流程图部分 |

## 执行顺序

1. 先完成任务 2 的 config.py 修改（添加配置项）
2. 再完成任务 1 和任务 2 的 run.md 修改（可合并在一次编辑中）
3. 验证修改内容

## 风险评估

- **低风险**：仅修改文档和默认配置，不影响现有功能
- **向后兼容**：新配置项有默认值，旧项目无需修改配置
