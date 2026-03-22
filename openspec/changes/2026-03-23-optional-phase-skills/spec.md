# 规范：可选子过程 Skills

## ADDED: skill: make-graphics

- 为任务编写 PlantUML 图解
- 保存至 flow-graphics/ 目录
- 根据 design.md 中的标记判断是否需要绘制图解

## ADDED: skill: integration

- 所有任务点完成 impl-verify 循环后执行
- 整体集成测试，确保各部分协同工作
- 检查遗漏的边界情况和兼容性问题

## ADDED: skill: review

- 代码质量评估（可读性、可维护性、性能、安全性）
- 文档审校（准确性、一致性）
- 方案评审（可行性、合理性）

## ADDED: skill: docs-update

- 更新项目文档（README、API 文档等）
- 同步 memory 中的全局信息
- 更新 concepts/ 和 structure/ 中的相关内容

## MODIFIED: skill: rework

调整现有 rework skill，适配新的阶段体系。

- 从任意阶段返回之前的阶段
- 记录返工原因和影响分析
- 确保返工后至少经过后续必要阶段才能再次进入 confirm
