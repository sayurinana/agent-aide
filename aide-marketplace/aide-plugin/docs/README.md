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
┌─────────────────────────────────────────────────┐
│                  Commands                        │
│  定义"做什么"和"按什么顺序做"                    │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐         │
│  │  init   │  │  prep   │  │  exec   │         │
│  └────┬────┘  └────┬────┘  └────┬────┘         │
└───────┼────────────┼────────────┼───────────────┘
        │            │            │
        ▼            ▼            ▼
┌─────────────────────────────────────────────────┐
│                   Skill                          │
│  定义"怎么调用工具"                              │
│  ┌─────────────────────────────────────────┐    │
│  │              aide skill                  │    │
│  │  env | flow | decide | config | init    │    │
│  └─────────────────────────────────────────┘    │
└─────────────────────────────────────────────────┘
        │
        ▼ 调用
┌─────────────────────────────────────────────────┐
│               aide-program                       │
│  实际执行操作，返回精简结果                      │
└─────────────────────────────────────────────────┘
```

---

## 二、Commands 索引

| Command | 设计文档 | 执行文件 | 职责 |
|---------|----------|----------|------|
| `/aide:init` | [commands/init.md](commands/init.md) | [../../commands/init.md](../commands/init.md) | 项目认知与环境初始化 |
| `/aide:prep` | [commands/prep.md](commands/prep.md) | [../../commands/prep.md](../commands/prep.md) | 任务准备流程 |
| `/aide:exec` | [commands/exec.md](commands/exec.md) | [../../commands/exec.md](../commands/exec.md) | 任务执行流程 |

### 2.1 Commands 设计原则

**聚焦思考方法论，不涉及工具细节**

Commands 只告诉 LLM：
- 怎么思考（分析方法、优化方向）
- 流程是什么（阶段划分、执行顺序）
- 决策边界（哪些自主完成，哪些需确认）

具体工具调用由 Skill 负责说明。

### 2.2 工作流程

```
/aide:init          /aide:prep              /aide:exec
    │                   │                       │
    ▼                   ▼                       ▼
┌────────┐        ┌──────────┐           ┌──────────┐
│环境检测│        │ 任务分析 │           │ 流程设计 │
│项目认知│        │ 任务优化 │           │ 迭代实现 │
│介绍能力│        │ 待定项   │           │ 验证交付 │
└────────┘        │ 生成细则 │           │ 文档更新 │
                  └──────────┘           │ 收尾     │
                       │                 └──────────┘
                       ▼
                  task-spec.md ──────────────▶
```

---

## 三、Skill 索引

| Skill | 执行文件 | 职责 | 触发时机 |
|-------|----------|------|----------|
| aide | [../skills/aide/SKILL.md](../skills/aide/SKILL.md) | aide 基础命令指南 | 始终加载 |
| env-config | [../skills/env-config/SKILL.md](../skills/env-config/SKILL.md) | 环境配置详细指南 | `aide env ensure` 失败时 |

### 3.1 Skill 设计原则

**按需触发，避免信息过载**

- `aide` skill：始终加载，提供基础命令用法
- `env-config` skill：按需触发，仅在环境检测失败时使用

**纯工具说明，便于快速查阅**

Skill 只包含：
- 命令语法和参数
- 输入输出格式
- 典型使用示例

不包含流程指导和业务逻辑。

### 3.2 Skill 触发逻辑

```
aide env ensure
    │
    ├─ 全部 ✓ → 继续流程（无需额外 skill）
    │
    └─ 有 ✗ → 触发 env-config skill
              │
              ├─ 分析项目类型
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

### 4.2 不需要程序约束的场景

| 场景 | 说明 |
|------|------|
| 任务分析思考 | LLM 自由发挥 |
| 任务优化思考 | LLM 自由发挥 |
| 业务决策判断 | LLM 自由发挥 |
| 任务细则编写 | LLM 自由发挥，产出 task-spec.md |
| 业务代码编写 | LLM 自由发挥 |

---

## 五、修改指南

### 5.1 修改 Command

1. 阅读对应的设计文档（如 `commands/init.md`）
2. 理解职责和流程
3. 修改执行文件（如 `../commands/init.md`）
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
