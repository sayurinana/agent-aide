# 规范：Commands 重写

## ADDED: command: make-memory

指导 Agent 创建子代理，由子代理学习 make-memory skill 后为项目生成 memory 文档集。

## ADDED: command: load-memory

指导 Agent 学习 load-memory skill，根据该 skill 的方法载入项目 memory。

## ADDED: command: hi

- 若尚未理解 aide-process-overview.md，要求先完整读取并学习
- 指导执行 `aide hi` 子命令，说明其输出内容及如何理解
- 根据输出信息判断是否需要载入 memory
- 结合 aide hi 输出和项目 memory，向用户提出建议的行动

## ADDED: command: go

- 若尚未理解 aide-process-overview.md，要求先完整读取并学习
- 若尚未载入 memory，要求学习 load-memory skill 并载入
- 指导执行 `aide hi` 获取当前项目状态
- 指导执行 `aide go` 接续当前任务状态，按计划流程继续实施

## ADDED: command: bye

- 若尚未理解 aide-process-overview.md，要求先完整读取并学习
- 指导执行 `aide hi` 获取当前状态
- 根据状态判断应执行的收尾操作

## ADDED: aide-process-overview.md

概述 aide 体系的所有情境和阶段，列出每个阶段对应的详细 skill。

## ADDED: AGENT.md

定义 Agent 身份为统筹全局的总工程师，明确职责分工。
