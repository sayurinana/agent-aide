# 任务细则

> 生成时间：2025-12-16
> 任务来源：task-now.md
> 复杂度等级：复杂（需拆分为子计划）

## 任务概述

对 Aide 工作流体系进行多模块调整，包括：创建新 skill、修改 run 命令、强化流程图规范、创建示例程序、调整 flow 提交顺序、更新文档。

## 子计划列表

### 子计划 1：创建口语化内容解析 skill

**目标**：创建一个 skill，用于识别和解析口语化的任务描述

**输入**：`statements/optimize.md` 作为基础

**输出**：`aide-marketplace/aide-plugin/skills/task-parser.md`

**功能要求**：
1. 语义解析：分析口语化表达，提取核心意图和真实需求
2. 批判性分析：识别逻辑漏洞、盲点、过度设计
3. 建设性建议：提出优化方向和替代方案
4. 规范化转换：将口语化内容转化为结构化任务描述

**扩展内容**（基于原材料发挥）：
- 上下文关联：识别任务与项目现有内容的关联
- 隐含需求挖掘：发现未明确说明但实际需要的内容
- 复杂度预判：初步评估任务复杂度

---

### 子计划 2：修改 run 命令集成 skill 触发

**目标**：在 run 命令中添加口语化内容检测和 skill 触发逻辑

**修改文件**：`aide-marketplace/aide-plugin/commands/run.md`

**修改位置**：在 1.3 任务分析之前

**新增逻辑**：
```
如果任务文档或用户对话具有以下特征之一：
- 使用非正式的口头表达方式
- 包含大量"我觉得"、"好像"、"大概"等模糊表述
- 句子结构松散，缺乏条理性
- 包含冗余或重复的表达

则：
1. 触发 task-parser skill 学习解析方法
2. 对内容进行深度理解和规范化转换
3. 将转换后的结构化内容作为后续分析的基础
```

---

### 子计划 3：强化流程图规范

**目标**：明确定义两种流程图类型及其规范

**修改文件**：`aide-marketplace/aide-plugin/commands/run.md`

**修改位置**：2.2 创建流程图部分

**新增内容**：

#### 流程图类型定义

| 类型 | 适用场景 | 必需性 |
|------|----------|--------|
| 任务执行流程图 | 所有任务 | 必需 |
| 程序逻辑流图 | 含程序设计与代码编写的任务 | 必需 |

#### 任务执行流程图规范
- 展示任务执行的步骤顺序
- 体现任务分解和依赖关系
- 包含关键决策点和分支

#### 程序逻辑流图规范
- **入口点**：从程序入口函数（如 main）开始
- **结构体现**：展示顺序、分支、循环等控制结构
- **语义化抽象**：将代码逻辑抽象为人类可理解的业务描述
- **模块化表示**：
  - 函数/模块表示为"盒子"
  - 标注输入和输出
  - 复杂模块可用子流程图详细展开
- **层次化组织**：
  - 主流程图展示整体逻辑
  - 子系统/模块可单独绘制详图

---

### 子计划 4：创建 Python 示例程序并绘制流程图

**目标**：验证流程图规范的有效性

**位置**：`test-cache/demo-program/`

**程序要求**：
- 语言：Python
- 复杂度：中低
- 特点：模块化设计，包含多个文件

**产出**：
1. Python 程序代码
2. 程序逻辑流图（PlantUML）
   - 主流程图
   - 关键模块详图（如需要）

---

### 子计划 5：调整 aide flow 的 git 提交顺序

**目标**：确保 flow-status.json 的更新包含在 git commit 中

**修改文件**：`aide-program/aide/flow/tracker.py`

**当前顺序**（`_apply_action` 方法）：
1. run_pre_commit_hooks
2. git.add_all()
3. git.commit(message)
4. 创建新的 FlowStatus（内存）
5. 返回 FlowStatus
6. （在 `_run` 中）storage.save_status(updated)

**目标顺序**：
1. run_pre_commit_hooks
2. 创建新的 FlowStatus（内存）
3. storage.save_status(updated)
4. git.add_all()
5. git.commit(message)

**技术方案**：
- 重构 `_apply_action` 方法，将 git 操作分离
- 或在 `_run` 中调整调用顺序

---

### 子计划 6：同步更新所有相关文档

**更新列表**：
1. `.aide/project-docs/blocks/aide-plugin-skills.md` - 添加新 skill 说明
2. `.aide/project-docs/blocks/aide-plugin-commands.md` - 更新 run 命令说明
3. `.aide/project-docs/blocks/aide-program-flow.md` - 更新 flow 模块说明
4. `aide-marketplace/aide-plugin/skills/aide.md` - 如有必要

## 执行顺序

```
子计划1 → 子计划2 → 子计划3 → 子计划4 → 子计划5 → 子计划6
```

## 成功标准

1. ✓ 新 skill 文件存在且功能完整
2. ✓ run 命令能识别口语化内容并触发 skill
3. ✓ 流程图规范清晰明确，区分两种类型
4. ✓ 示例程序正常运行，流程图正确展示程序逻辑
5. ✓ flow 命令的 git 提交包含最新的 flow-status.json
6. ✓ 所有相关文档已同步更新
