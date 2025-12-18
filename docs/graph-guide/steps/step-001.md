# 步骤 001：aide-program - guide.puml

## 元信息

| 属性 | 值 |
|------|-----|
| 状态 | pending |
| 所属区块 | aide-program |
| 流程图类型 | guide |
| 预估工作量 | 中 |
| 依赖步骤 | 无 |

## 任务描述

绘制 aide-program 的整体架构图，展示核心模块关系和数据流向。

## 模块结构（已分析）

以下是执行本步骤所需的全部模块信息：

### 涉及文件

| 文件路径 | 职责 | 关键内容 |
|----------|------|----------|
| `aide-program/aide/main.py` | CLI 主入口 | 命令行解析、处理器分发（约 440 行） |
| `aide-program/aide/core/config.py` | 配置管理 | ConfigManager 类（约 390 行） |
| `aide-program/aide/core/output.py` | 输出格式 | 输出格式工具（25 行） |
| `aide-program/aide/env/manager.py` | 环境管理 | EnvManager 类（约 370 行） |
| `aide-program/aide/flow/tracker.py` | 流程追踪 | FlowTracker 类（233 行） |
| `aide-program/aide/decide/cli.py` | 待定项 CLI | submit/result 处理（134 行） |

### 模块关系

```
main.py (CLI 入口)
  ├→ core/
  │    ├→ config.py (ConfigManager)
  │    └→ output.py (输出格式)
  ├→ env/
  │    ├→ manager.py (EnvManager)
  │    ├→ registry.py (模块注册)
  │    └→ modules/* (检测模块)
  ├→ flow/
  │    ├→ tracker.py (FlowTracker)
  │    ├→ storage.py (状态存储)
  │    ├→ git.py (Git 集成)
  │    ├→ hooks.py (环节钩子)
  │    ├→ branch.py (分支管理)
  │    └→ validator.py (流程校验)
  └→ decide/
       ├→ cli.py (命令处理)
       ├→ server.py (HTTP 服务)
       ├→ handlers.py (请求处理)
       └→ storage.py (数据存储)
```

### 数据流

```
用户命令 → main.py 解析 → 对应模块处理 → 输出结果

具体流向：
- aide init → core/config.py
- aide env * → env/manager.py
- aide flow * → flow/tracker.py
- aide decide * → decide/cli.py
- aide config * → core/config.py
```

### 关键函数/类

| 名称 | 位置 | 说明 |
|------|------|------|
| `main()` | main.py | 程序入口，命令行解析 |
| `ConfigManager` | core/config.py:240 | 配置管理器 |
| `EnvManager` | env/manager.py:53 | 环境管理器 |
| `FlowTracker` | flow/tracker.py:20 | 流程追踪器 |
| `handle_decide_command()` | decide/cli.py | 待定项命令处理 |

## 输出要求

- 文件：`aide-program/guide.puml`
- 类型：组件图 (component diagram)
- 内容要求：
  - [ ] 展示 main.py 作为入口
  - [ ] 展示 4 个核心模块（core、env、flow、decide）
  - [ ] 展示模块间的依赖关系
  - [ ] 标注各模块的主要职责

## PlantUML 模板

```plantuml
@startuml guide
skinparam defaultFontName "PingFang SC"
skinparam dpi 300
scale 0.3

title aide-program 架构概览

' TODO: 基于上述模块结构绘制组件图

@enduml
```

## 执行记录

| 时间 | 操作 | 备注 |
|------|------|------|
| | | |
