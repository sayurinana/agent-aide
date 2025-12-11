# Aide 系统需求规格

## 一、项目背景

### 1.1 现状问题

原有 `ai-agent-memory/` 体系存在以下问题：

| 问题类型 | 具体表现 |
|---------|---------|
| 信息过载 | CLAUDE.md 包含完整流程规范，每次对话都需加载 |
| 操作繁琐 | CSV 状态需手动编辑、PlantUML 命令冗长、Git 多步操作 |
| 输出冗余 | 命令执行输出大量日志，无论成功失败 |
| 流程耦合 | AB 部分虽拆分但仍需手动切换和阅读大量文档 |

### 1.2 转型目标

将原有内容体系转化为 Command + Skill 体系：

- **CLAUDE.md 精简化**：仅保留项目文件结构说明，不再指导规则和流程
- **流程按需触发**：通过 Command 主动触发流程指导和规则启示
- **操作确定性封装**：通过 Skill + 定制脚本简化操作，减少不确定性

---

## 二、核心设计原则

### 2.1 渐进式披露

- 按需调用、触发
- 不在 CLAUDE.md 中堆积所有规则
- 用户/LLM 主动调用时才加载相关指引

### 2.2 确定性封装

- 将可变过程转化为固定接口
- 只暴露脚本程序和参数信息
- 内部处理流程固定化，减少多余输出

### 2.3 信息隔离

- LLM 只传核心语义数据
- 程序负责格式化、渲染、美化
- 返回结果极简，只含决策所需信息

### 2.4 核心与形式分离

| 类型 | 定义 | 处理方式 |
|------|------|----------|
| 核心信息 | 分析思考、优化思考、业务决策 | LLM 自由发挥，不受限制 |
| 形式问题 | 待定项呈现、状态记录、环境配置 | 程序封装，减少 token 污染 |

---

## 三、组件职责定义

### 3.1 Command（命令）

**本质**：对 LLM 的要求和规则

**内容焦点**：
- 思考方法论：怎么分析、怎么优化、怎么执行
- 流程指导：阶段划分、核心必做、质量要求
- 决策边界：哪些由 LLM 自主完成，哪些需要用户确认

**设计要求**：
- 聚焦核心思考，不涉及工具调用细节
- 指导方向而非限制形式
- 结果输出不受格式约束，让 LLM 竭尽全力发挥

### 3.2 Skill（技能）

**本质**：告诉 LLM 有什么工具可用

**内容焦点**：
- 工具使用说明：命令、参数、输出格式
- 调用示例：典型场景的命令示例
- 输出解读：各种输出前缀的含义

**设计要求**：
- 纯工具说明，不涉及流程指导
- 精简明确，便于快速查阅
- 封装形式问题，减少 LLM 认知负担

---

## 四、命令清单

### 4.1 /aide:init - 认知初始化

**触发时机**：进入项目开始工作时

**职责**：
1. 介绍 aide 流程体系
2. 列出可用能力（Skills）
3. 说明环境和版本控制约定

**特点**：
- 不执行实际操作
- 提供认知框架

### 4.2 /aide:prep - 任务准备

**触发时机**：准备开始新任务时

**职责**：
1. 任务分析（理解目标、识别复杂度、分析环境）
2. 任务优化（准确性、简洁性、可执行性）
3. 待定项处理（通过 aide 程序化呈现）
4. 结果生成（LLM 自由发挥，产出 task-spec.md）

**核心原则**：
- 分析和优化阶段：指导 LLM 思考方向，让其竭尽全力发挥
- 待定项处理：程序化呈现，减少 token 污染
- 结果生成：不受格式限制，由用户直接审阅

**运行特点**：
- 轻量化：不创建工作目录、不记录状态、不 git 提交

### 4.3 /aide:exec - 任务执行

**触发时机**：任务准备完成，开始执行时

**职责**：
1. 流程规划（理解细则、制定计划、环境准备）
2. 迭代实现（按计划执行、状态同步、阻塞处理）
3. 验证交付（对照标准、功能验证）
4. 文档收尾（变更记录、版本发布）

**核心原则**：
- 业务代码编写：LLM 自由发挥，不加程序约束
- 状态管理、版本控制：通过 aide 程序处理，避免信息污染

---

## 五、技能清单

### 5.1 aide-env - 环境管理

**用途**：检测和修复项目开发环境

**核心命令**：
- `aide env check` - 检测环境（只读）
- `aide env ensure` - 检测并修复

**设计要点**：
- 成功时输出极简：`✓ 环境就绪 (python:3.12)`
- 自动修复小问题时简短提示
- 仅无法修复时才需要 LLM 关注

### 5.2 aide-undetermined - 待定项处理

**用途**：程序化呈现待定项，获取用户确认

**核心命令**：
- `aide undetermined add '<json>'` - 添加待定项
- `aide undetermined confirm` - 生成确认报告（给用户）
- `aide undetermined result` - 获取结果（给 LLM）

**设计要点**：
- LLM 传入精简 JSON 数据
- 程序渲染美化界面给用户
- 返回精简决策结果给 LLM

### 5.3 aide-workspace - 工作目录管理

**用途**：管理任务工作目录

**核心命令**：
- `aide workspace init <name>` - 创建工作目录
- `aide workspace clean --keep=N` - 清理旧目录

### 5.4 aide-progress - 进度管理

**用途**：记录任务执行状态

**核心命令**：
- `aide progress init <name>` - 初始化状态
- `aide progress update <phase> <status>` - 更新状态

**设计要点**：
- 替代手动编辑 CSV
- 提供动态反馈和步骤引导
- 避免 LLM "跑飞"

### 5.5 aide-version - 版本控制

**用途**：管理 CHANGELOG 和 Git 操作

**核心命令**：
- `aide version add <type> "<desc>"` - 添加变更
- `aide version commit "<msg>"` - Git 提交
- `aide version release [level]` - 发布版本

**设计要点**：
- 封装 Git 操作，减少情况分析复杂度
- 通过有限参数控制不同需求

### 5.6 aide-build - 构建工具

**用途**：编译 PlantUML 等

**核心命令**：
- `aide build plantuml <src>` - 编译
- `aide build plantuml <src> -c` - 语法检查

**设计要点**：
- 集成语法检查、编译、简要输出
- 替代冗长的 java -jar 命令

---

## 六、信息流设计

### 6.1 待定项处理流程

```
LLM                    程序                    用户
 │                      │                      │
 │  JSON (精简数据)     │                      │
 │─────────────────────→│                      │
 │                      │  渲染美化界面         │
 │                      │─────────────────────→│
 │                      │                      │
 │                      │  用户选择             │
 │                      │←─────────────────────│
 │  JSON (决策结果)     │                      │
 │←─────────────────────│                      │
```

### 6.2 输出精简原则

| 场景 | 输出策略 |
|------|----------|
| 成功 | 极简确认：`✓ 操作完成` |
| 自动修复 | 简短提示：`✓ 已修复: xxx` |
| 警告 | 告知但可继续：`⚠ 警告内容` |
| 失败 | 详细原因：`✗ 失败原因及建议` |

---

## 七、LLM 自由发挥边界

### 7.1 需要程序约束的场景

- 环境检测与修复
- 待定项呈现与确认
- 状态记录与更新
- 版本控制操作
- 构建工具调用

### 7.2 不需要程序约束的场景

- 任务分析思考
- 任务优化思考
- 业务决策判断
- 任务细则编写（task-spec.md）
- 业务代码编写
- 结果文档产出

---

## 八、数据格式规范

### 8.1 待定项数据格式

**输入格式（LLM → 程序）**：

```json
{
  "items": [
    {
      "id": "唯一标识符",
      "question": "问题标题",
      "description": "详细说明（可选）",
      "options": [
        {"value": "程序值", "label": "显示文本", "score": 0-100}
      ],
      "recommend": "推荐选项的 value",
      "reason": "推荐理由"
    }
  ]
}
```

**输出格式（程序 → LLM）**：

```json
{
  "decisions": [
    {"id": "标识符", "chosen": "选择的 value", "custom": "自定义内容或 null"}
  ]
}
```

### 8.2 输出前缀规范

| 前缀 | 函数 | 用途 |
|------|------|------|
| ✓ | `ok` | 成功 |
| ⚠ | `warn` | 警告（可继续） |
| ✗ | `err` | 失败 |
| → | `info` | 进行中 |
| [n/m] | `step` | 步骤进度 |

### 8.3 配置文件 aide.toml

```toml
[env]
modules = ["python", "uv"]

[env.python]
version = ">=3.10"
venv = ".venv"
requirements = "requirements.txt"

[workspace]
output = "ai-agent-output"
format = "{timestamp}_{name}"

[progress]
phases = ["规划", "实现", "验证", "文档", "收尾"]

[version]
changelog = "CHANGELOG.md"
```

---

## 九、实施结构

### 9.1 插件目录结构

```
aide-plugin/
├── .claude-plugin/
│   └── plugin.json
├── commands/
│   ├── init.md
│   ├── prep.md
│   └── exec.md
└── skills/
    ├── aide-env/SKILL.md
    ├── aide-undetermined/SKILL.md
    ├── aide-workspace/SKILL.md
    ├── aide-progress/SKILL.md
    ├── aide-version/SKILL.md
    └── aide-build/SKILL.md
```

### 9.2 运行时脚本结构

```
aide/
├── aide.sh                 # 统一入口
├── lib/
│   ├── output.sh           # 输出函数（ok/warn/err/info/step）
│   └── config.py           # 配置读取
├── env/
│   └── check.py            # 环境检测
├── undetermined/
│   └── handler.py          # 待定项处理
├── workspace/
│   └── manager.py          # 工作目录管理
├── progress/
│   └── tracker.py          # 进度管理
├── version/
│   ├── changelog.py        # CHANGELOG 管理
│   └── git.sh              # Git 操作
└── build/
    └── plantuml.sh         # PlantUML 编译
```

### 9.3 安装方式

```bash
export AIDE_ROOT="/path/to/aide"
ln -s "$AIDE_ROOT/aide.sh" ~/.local/bin/aide
```

---

## 十、实施检查清单

### 10.1 插件部分

- [ ] plugin.json 元数据
- [ ] commands/init.md - 认知初始化
- [ ] commands/prep.md - 任务准备方法论
- [ ] commands/exec.md - 任务执行方法论
- [ ] skills/aide-env/SKILL.md
- [ ] skills/aide-undetermined/SKILL.md
- [ ] skills/aide-workspace/SKILL.md
- [ ] skills/aide-progress/SKILL.md
- [ ] skills/aide-version/SKILL.md
- [ ] skills/aide-build/SKILL.md

### 10.2 运行时部分

- [ ] aide.sh 入口脚本
- [ ] lib/output.sh 输出函数
- [ ] lib/config.py 配置读取
- [ ] env/check.py 环境检测
- [ ] undetermined/handler.py 待定项处理
- [ ] workspace/manager.py 工作目录
- [ ] progress/tracker.py 进度管理
- [ ] version/changelog.py CHANGELOG
- [ ] version/git.sh Git 操作
- [ ] build/plantuml.sh PlantUML

### 10.3 验收标准

1. 三个 Command 能正确触发对应流程
2. 六个 Skill 的 aide 命令能正常执行
3. 待定项能正确渲染和返回结果
4. 输出符合精简原则
