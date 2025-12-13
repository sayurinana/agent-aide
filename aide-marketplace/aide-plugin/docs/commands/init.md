# /aide:init 命令设计文档

## 一、背景

### 1.1 解决的问题

在开始项目工作前，需要解决以下问题：

| 问题 | 影响 |
|------|------|
| 环境不就绪 | 后续命令执行失败，打断业务流程 |
| 项目认知缺失 | LLM 不了解项目结构，决策质量下降 |
| 配置未初始化 | aide 程序无法正常工作 |

### 1.2 设计目标

将环境问题**前置解决**，在业务流程开始前确保一切就绪：
- 沉没成本小：发现严重问题可重开对话
- 环境问题不会打断后续业务逻辑

---

## 二、职责

### 2.1 做什么

1. 检测 aide 运行时环境（Python、uv 等）
2. 初始化 `.aide/` 目录和配置文件
3. 触发 LLM 对项目内容的主动认知
4. 检测项目开发环境并自动修复问题
5. 介绍 aide 流程体系和可用能力

### 2.2 不做什么

- 不修改任何业务代码
- 不执行任务分析或优化
- 不启动流程追踪（flow）

---

## 三、执行流程

```
@startuml
skinparam defaultFontName "PingFang SC"

start

:aide env ensure --runtime;
note right: 检查 aide 自身运行环境\n不依赖配置文件

if (运行时环境就绪?) then (是)
else (否)
  :输出错误信息;
  :告知用户修复方法;
  stop
endif

:aide init;
note right: 创建 .aide/ 目录\n生成默认配置\n更新 .gitignore

:项目认知;
note right: 阅读 README.md\n阅读 CLAUDE.md\n浏览目录结构\n识别技术栈

:aide env ensure;
note right: 读取配置文件\n检查项目环境\n输出配置信息

if (项目环境就绪?) then (是)
else (否)
  if (可自动修复?) then (是)
    :自动修复;
  else (否)
    :告知用户;
    if (3次尝试失败?) then (是)
      stop
    endif
  endif
endif

:汇报就绪状态;
note right: 项目概况\n环境状态\n可用命令

stop
@enduml
```

### 3.1 步骤详解

| 步骤 | 操作 | aide 命令 | 说明 |
|------|------|-----------|------|
| 1 | 运行时环境检测 | `aide env ensure --runtime` | 不依赖配置文件 |
| 2 | 初始化配置 | `aide init` | 创建 .aide/ 和配置 |
| 3 | 项目认知 | 无 | LLM 自主阅读项目文件 |
| 4 | 项目环境检测 | `aide env ensure` | 读取配置，检查项目环境 |
| 5 | 汇报状态 | 无 | LLM 向用户汇报 |

---

## 四、与 aide 程序的交互

### 4.1 aide env ensure --runtime

**调用时机**：步骤 1

**输入**：无

**输出示例**：
```
✓ 运行时环境就绪 (python:3.12, uv:0.4.0)
```

**错误处理**：
- `✗ Python 版本不足` → 告知用户安装要求
- `✗ 未检测到 uv` → 告知用户安装 uv

### 4.2 aide init

**调用时机**：步骤 2

**输入**：无

**输出示例**：
```
✓ 已创建默认配置 .aide/config.toml
✓ 初始化完成，.aide/ 与默认配置已准备就绪
```

**行为**：
- 创建 `.aide/` 目录
- 生成 `config.toml` 默认配置
- 检查并更新 `.gitignore`

### 4.3 aide env ensure

**调用时机**：步骤 4

**输入**：无（读取 `.aide/config.toml`）

**输出示例**：
```
→ 任务原文档: task-now.md
→ 任务细则文档: task-spec.md
✓ 环境就绪 (python:3.12, uv:0.4.0, venv:.venv)
```

**行为**：
- 读取配置文件
- 检查/创建虚拟环境
- 安装依赖
- 输出任务文档路径配置

---

## 五、项目认知阶段

### 5.1 认知内容

| 内容 | 来源 | 目的 |
|------|------|------|
| 项目概述 | README.md | 理解项目目标和功能 |
| 开发约定 | CLAUDE.md | 了解项目特定规则 |
| 目录结构 | 文件系统 | 理解模块划分 |
| 技术栈 | package.json/requirements.txt 等 | 了解使用的技术 |

### 5.2 认知输出

向用户汇报项目概况（3-5句话），包括：
- 项目是什么
- 主要功能/模块
- 使用的技术栈

---

## 六、汇报格式

```
项目概况：[来自项目认知的概要]

环境状态：✓ 环境就绪 (python:3.12, ...)

项目配置：
- 任务原文档：task-now.md
- 任务细则：task-spec.md

Aide 已就绪，可用命令：
- /aide:prep [文档路径] - 任务准备
- /aide:exec [文档路径] - 任务执行
```

---

## 七、依赖

| 依赖项 | 类型 | 说明 |
|--------|------|------|
| aide env | aide 子命令 | 环境检测 |
| aide init | aide 子命令 | 配置初始化 |

---

## 八、被依赖

| 依赖方 | 说明 |
|--------|------|
| /aide:prep | 依赖 init 完成环境准备 |
| /aide:exec | 依赖 init 完成环境准备 |

---

## 九、修改指南

### 9.1 修改执行流程

1. 更新本文档的流程图和步骤详解
2. 修改执行文件 `../../commands/init.md`
3. 如涉及新的 aide 子命令，同步更新 [aide skill 设计文档](../skill/aide.md)

### 9.2 修改项目认知内容

1. 更新本文档的"项目认知阶段"章节
2. 修改执行文件中的认知步骤

### 9.3 修改汇报格式

1. 更新本文档的"汇报格式"章节
2. 修改执行文件中的汇报模板

---

## 十、相关文档

- [执行文件](../../commands/init.md)
- [aide env 设计](../../../../aide-program/docs/commands/env.md)
- [aide init 设计](../../../../aide-program/docs/commands/init.md)
- [plugin 导览](../README.md)
