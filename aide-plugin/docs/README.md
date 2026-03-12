# Aide Plugin 设计文档

## 一、概述

aide-plugin 是 Claude Code 插件，提供 Aide 工作流体系的 Commands 和 Skill。

### 1.1 解决的问题

| 问题 | 解决方案 |
|------|----------|
| CLAUDE.md 信息过载 | 流程规则封装到 Commands，按需触发 |
| 操作指令分散 | 工具使用方法集中到 Skill |
| 流程遵循不一致 | Commands 定义明确的阶段和顺序 |

### 1.2 组件关系

```
┌─────────────────────────────────────────────────────────────────┐
│                        Commands                                  │
│  定义"做什么"和"按什么顺序做"                                    │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐            │
│  │  setup  │  │  load   │  │  docs   │  │   run   │            │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘            │
└───────┼────────────┼────────────┼────────────┼──────────────────┘
        │            │            │            │
        ▼            ▼            ▼            ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Skills                                   │
│  定义"怎么调用工具"                                              │
│  ┌─────────────────────┐  ┌─────────────────────┐              │
│  │     aide skill      │  │  env-config skill   │              │
│  │ (基础命令指南)       │  │ (环境配置详细指南)   │              │
│  └─────────────────────┘  └─────────────────────┘              │
└─────────────────────────────────────────────────────────────────┘
        │
        ▼ 调用
┌─────────────────────────────────────────────────────────────────┐
│                      aide-program                                │
│  实际执行操作，返回精简结果                                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## 二、Commands 索引

| Command | 设计文档 | 执行文件 | 职责 | 独立运行 |
|---------|----------|----------|------|----------|
| `/aide:setup` | [commands/setup.md](commands/setup.md) | [../commands/setup.md](../commands/setup.md) | 环境配置 | 是 |
| `/aide:load` | [commands/load.md](commands/load.md) | [../commands/load.md](../commands/load.md) | 项目认知载入 | 否 |
| `/aide:docs` | [commands/docs.md](commands/docs.md) | [../commands/docs.md](../commands/docs.md) | 项目文档管理 | 是 |
| `/aide:run` | [commands/run.md](commands/run.md) | [../commands/run.md](../commands/run.md) | 任务执行（核心） | 否 |

> 注：原 `/aide:init`、`/aide:prep`、`/aide:exec` 已重组为上述命令

### 2.1 Commands 设计原则

**聚焦思考方法论，不涉及工具细节**

Commands 只告诉 LLM：
- 怎么思考（分析方法、优化方向）
- 流程是什么（阶段划分、执行顺序）
- 决策边界（哪些自主完成，哪些需确认）

具体工具调用由 Skill 负责说明。

### 2.2 工作流程

```
/aide:setup         /aide:docs          /aide:run
(独立运行)          (独立运行)          (核心命令)
    │                   │                   │
    ▼                   ▼                   ▼
┌────────┐        ┌──────────┐        ┌──────────────────┐
│环境配置│        │ 项目文档 │        │ task-optimize    │
│依赖分析│        │ 区块划分 │        │ ├─ 任务分析      │
│模块检测│        │ 文档生成 │        │ ├─ 复杂度评估    │
│问题修复│        │ 增量更新 │        │ ├─ 待定项处理    │
└────────┘        └──────────┘        │ └─ 生成细则      │
                                      │ flow-design      │
                                      │ └─ 创建流程图    │
                                      │ impl (迭代实现)  │
                                      │ verify (验证)    │
                                      │ docs (文档更新)  │
                                      │ finish (收尾)    │
                                      └──────────────────┘
```

> `/aide:load` 由 `/aide:run` 自动调用，用于按需载入项目文档。

---

## 三、Skill 索引

| Skill | 执行文件 | 职责 | 触发时机 |
|-------|----------|------|----------|
| aide | [../skills/aide/SKILL.md](../skills/aide/SKILL.md) | aide 基础命令指南 | `/aide:run` 强制触发 |
| env-config | [../skills/env-config/SKILL.md](../skills/env-config/SKILL.md) | 环境配置详细指南 | `/aide:setup` 强制触发 |

### 3.1 Skill 设计原则

**按需触发，避免信息过载**

- `aide` skill：由 `/aide:run` 强制触发，提供基础命令用法
- `env-config` skill：由 `/aide:setup` 强制触发，提供详细环境配置指导

**纯工具说明，便于快速查阅**

Skill 只包含：
- 命令语法和参数
- 输入输出格式
- 典型使用示例

不包含流程指导和业务逻辑。

### 3.2 Skill 触发逻辑

```
/aide:setup                          /aide:run
    │                                    │
    ▼                                    ▼
触发 env-config skill              触发 aide skill
    │                                    │
    ▼                                    ▼
aide env ensure                    aide flow status
    │                                    │
    ├─ 全部 ✓ → 完成                     ├─ 无活跃任务 → 新任务流程
    │                                    │
    └─ 有 ✗ → 分析项目类型               └─ 有活跃任务 → 续接流程
              │
              ├─ aide env set 配置
              └─ 重试 aide env ensure
```

---

## 四、职责边界

### 4.1 需要程序约束的场景

| 场景 | 处理方式 |
|------|----------|
| 环境检测与修复 | `aide env ensure` |
| 待定项呈现与确认 | `aide decide` |
| 状态记录与 git 提交 | `aide flow` |
| 配置读写 | `aide config` |
| 流程图校验与构建 | `aide flow` + PlantUML |

### 4.2 不需要程序约束的场景

| 场景 | 说明 |
|------|------|
| 任务分析思考 | LLM 自由发挥 |
| 任务优化思考 | LLM 自由发挥 |
| 复杂度评估 | LLM 根据指导原则判断 |
| 业务决策判断 | LLM 自由发挥 |
| 任务细则编写 | LLM 自由发挥，产出 task-spec.md |
| 业务代码编写 | LLM 自由发挥 |
| 流程图设计 | LLM 自由发挥，产出 .puml 文件 |

---

## 五、修改指南

### 5.1 修改 Command

1. 阅读对应的设计文档（如 `commands/setup.md`）
2. 理解职责和流程
3. 修改执行文件（如 `../commands/setup.md`）
4. 更新设计文档（如有重大变更）
5. 更新本导览（如有新增/删除 Command）

### 5.2 修改 Skill

1. 确定要修改的 skill（aide 或 env-config）
2. 修改对应执行文件 `../skills/<skill>/SKILL.md`
3. 如涉及 aide-program 变更，同步更新 [aide-program 文档](../../../aide-program/docs/README.md)

**注意**：
- `aide` skill 保持精简，仅包含基础命令用法
- 详细配置指导放在 `env-config` skill

### 5.3 新增 Command

1. 在 `commands/` 下创建设计文档
2. 在 `../commands/` 下创建执行文件
3. 更新本导览的索引表
4. 更新 [总导览](../../../docs/aide-overview.md)

---

## 六、相关文档

- [总导览](../../../docs/aide-overview.md)
- [aide-program 导览](../../../aide-program/docs/README.md)
- [aide flow 子命令](../../../aide-program/docs/commands/flow.md)
- [aide flow 详细设计](../../../aide-program/docs/commands/flow/README.md)
- [Claude Code 插件指南](../../../docs/03-插件指南.md)

---

## 七、版本信息

- 当前版本：2.0.0
- 更新日期：2025-12-15
- 主要变更：Commands 体系重组（setup/load/docs/run 替代 init/prep/exec）
