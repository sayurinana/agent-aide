# 步骤 003：aide-program - env.puml

## 元信息

| 属性 | 值 |
|------|-----|
| 状态 | pending |
| 所属区块 | aide-program |
| 流程图类型 | 模块图 |
| 预估工作量 | 中 |
| 依赖步骤 | step-001 |

## 任务描述

绘制 aide-program 的环境检测流程图，展示 EnvManager 协调各检测模块的工作流程。

## 模块结构（已分析）

以下是执行本步骤所需的全部模块信息：

### 涉及文件

| 文件路径 | 职责 | 关键内容 |
|----------|------|----------|
| `aide-program/aide/env/manager.py` | 环境管理器 | EnvManager 类（约 370 行） |
| `aide-program/aide/env/registry.py` | 模块注册表 | 模块发现和注册（55 行） |
| `aide-program/aide/env/modules/base.py` | 模块基类 | BaseModule 接口定义（90 行） |
| `aide-program/aide/env/modules/python.py` | Python 检测 | PythonModule（59 行） |
| `aide-program/aide/env/modules/uv.py` | uv 检测 | UvModule（53 行） |
| `aide-program/aide/env/modules/venv.py` | 虚拟环境 | VenvModule（81 行） |
| `aide-program/aide/env/modules/requirements.py` | 依赖管理 | RequirementsModule（89 行） |
| `aide-program/aide/env/modules/rust.py` | Rust 检测 | RustModule（99 行） |
| `aide-program/aide/env/modules/node.py` | Node.js 检测 | NodeModule（94 行） |
| `aide-program/aide/env/modules/flutter.py` | Flutter 检测 | FlutterModule（133 行） |
| `aide-program/aide/env/modules/android.py` | Android 检测 | AndroidModule（147 行） |
| `aide-program/aide/env/modules/node_deps.py` | Node 依赖 | NodeDepsModule（142 行） |

### 模块关系

```
EnvManager
  ├→ Registry (模块注册表)
  │    └→ 自动发现 modules/ 下的模块
  │
  └→ 遍历启用的模块执行：
       ├→ validate_config() 验证配置
       ├→ check() 检测环境
       └→ ensure() 修复环境（如需要）

模块类型：
- 类型 A（全局）：python, uv, rust, node, flutter, android
- 类型 B（项目级）：venv, requirements, node_deps
```

### 数据流

```
aide env ensure
  → EnvManager.ensure()
     → 1. 加载配置中的 modules 列表
     → 2. 遍历每个模块：
          → validate_config(config) 验证配置
          → check(config, root) 检测状态
          → 如果检测失败且模块支持 ensure：
               → ensure(config, root) 修复
     → 3. 输出结果（✓/⚠/✗）
```

### 关键函数/类

| 名称 | 位置 | 说明 |
|------|------|------|
| `EnvManager` | manager.py:53 | 环境管理器，协调检测和修复 |
| `ensure()` | manager.py | 检测并修复环境 |
| `list_modules()` | manager.py | 列出可用模块 |
| `BaseModule` | base.py:37 | 模块基类 |
| `check()` | base.py | 检测方法（抽象） |
| `ensure()` | base.py | 修复方法（可选） |
| `ModuleRegistry` | registry.py | 模块注册表 |

### 检测模块能力矩阵

| 模块 | 类型 | check | ensure | 说明 |
|------|------|-------|--------|------|
| python | A | ✓ | - | Python 版本检测 |
| uv | A | ✓ | - | uv 包管理器检测 |
| venv | B | ✓ | ✓ | 虚拟环境管理 |
| requirements | B | ✓ | ✓ | 依赖管理 |
| rust | A | ✓ | - | Rust 工具链 |
| node | A | ✓ | - | Node.js 运行时 |
| flutter | A | ✓ | - | Flutter SDK |
| android | A | ✓ | - | Android SDK |
| node_deps | B | ✓ | ✓ | Node 项目依赖 |

## 输出要求

- 文件：`aide-program/env.puml`
- 类型：活动图 (activity diagram)
- 内容要求：
  - [ ] 展示 ensure 主流程
  - [ ] 展示模块遍历和检测逻辑
  - [ ] 展示 check → ensure 条件分支
  - [ ] 区分类型 A 和类型 B 模块的处理

## PlantUML 模板

```plantuml
@startuml env
skinparam defaultFontName "PingFang SC"
skinparam dpi 300
scale 0.3

title aide env ensure 流程

' TODO: 基于上述模块结构绘制活动图

@enduml
```

## 执行记录

| 时间 | 操作 | 备注 |
|------|------|------|
| | | |
