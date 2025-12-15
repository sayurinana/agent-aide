---
name: env-config
description: 环境配置详细指南。当 aide env ensure 检测失败需要配置环境时使用。提供模块选择、配置设置、多项目场景处理等详细指导。由 /aide:setup 命令强制触发。
---

# 环境配置指南

当 `aide env ensure` 检测失败（输出 `✗`）时，使用本指南分析项目所需环境并完成配置。

> **触发场景**：本 skill 由 `/aide:setup` 命令强制触发，用于独立的环境配置流程。

---

## 一、问题诊断

### 1.1 常见失败原因

| 错误信息 | 原因 | 解决方案 |
|----------|------|----------|
| `已启用但缺少配置项: path` | 类型B模块未配置路径 | 使用 `aide env set` 配置 |
| `未知模块: xxx` | 启用了不存在的模块 | 检查模块名拼写 |
| `xxx 未安装` | 工具未安装 | 安装对应工具或移除该模块 |
| `node_modules 不存在` | Node.js 依赖未安装 | 配置 node_deps 模块 |

### 1.2 查看可用模块

```bash
aide env list
```

---

## 二、模块分类

### 2.1 类型A：全局工具检测（无需配置）

| 模块 | 检测内容 |
|------|----------|
| `python` | Python 解释器版本 |
| `uv` | uv 包管理器 |
| `rust` | Rust 工具链（rustc + cargo） |
| `node` | Node.js 运行时 |
| `flutter` | Flutter SDK |
| `android` | Android SDK（ANDROID_HOME） |

### 2.2 类型B：项目级检测（需要配置）

| 模块 | 配置项 | 说明 |
|------|--------|------|
| `venv` | `path` | Python 虚拟环境目录 |
| `requirements` | `path` | Python 依赖文件路径 |
| `node_deps` | `path`, `manager`(可选) | Node.js 项目依赖 |

---

## 三、配置命令

### 3.1 aide env set

```bash
# 设置启用的模块列表
aide env set modules <模块列表>

# 设置模块配置
aide env set <模块名>.<配置项> <值>
```

### 3.2 验证规则

- 设置 `modules` 时，验证每个模块类型是否存在
- 无效模块名会报错并显示可用模块列表

```bash
# 验证失败示例
$ aide env set modules python,fortran
✗ 未知模块: fortran
→ 可用模块: python, uv, venv, requirements, rust, node, flutter, node_deps, android
```

---

## 四、项目类型配置

### 4.1 项目特征与模块映射

| 项目特征 | 推荐模块 |
|----------|----------|
| 存在 `Cargo.toml` | `rust` |
| 存在 `package.json` | `node`, `node_deps` |
| 存在 `pubspec.yaml` | `flutter` |
| 存在 `build.gradle` 或 `android/` | `android` |
| 存在 `requirements.txt` 或 `.venv` | `python`, `uv`, `venv`, `requirements` |

### 4.2 配置示例

**Rust 项目**：
```bash
aide env set modules rust
```

**Node.js 项目**：
```bash
aide env set modules node,node_deps
aide env set node_deps.path .
```

**Flutter 项目**：
```bash
aide env set modules flutter
# 如果需要构建 Android APK
aide env set modules flutter,android
```

**Python 项目**：
```bash
aide env set modules python,uv,venv,requirements
aide env set venv.path .venv
aide env set requirements.path requirements.txt
```

**混合项目**（多种技术栈）：
```bash
aide env set modules rust,node,flutter,android
```

---

## 五、多项目场景

### 5.1 模块实例化命名

当工作目录下有多个同类型子项目时，使用 `模块类型:实例名` 格式：

```bash
# 多个 Node.js 项目
aide env set modules node,node_deps:frontend,node_deps:admin
aide env set node_deps:frontend.path frontend
aide env set node_deps:admin.path admin
```

### 5.2 配置文件格式

```toml
[env]
modules = ["node", "node_deps:frontend", "node_deps:admin"]

[env."node_deps:frontend"]
path = "frontend"

[env."node_deps:admin"]
path = "admin"
manager = "pnpm"
```

### 5.3 输出示例

```
✓ node: 24.11.1 (npm 11.6.2)
✓ node_deps:frontend: frontend (npm)
✓ node_deps:admin: admin (pnpm)
✓ 环境就绪 (...)
```

---

## 六、node_deps 模块详解

### 6.1 配置项

| 配置项 | 必需 | 说明 |
|--------|------|------|
| `path` | 是 | package.json 所在目录 |
| `manager` | 否 | 包管理器，默认自动检测 |

### 6.2 包管理器自动检测

根据锁文件自动判断：

| 锁文件 | 包管理器 |
|--------|----------|
| `pnpm-lock.yaml` | pnpm |
| `yarn.lock` | yarn |
| `bun.lockb` | bun |
| `package-lock.json` 或无 | npm |

### 6.3 ensure 行为

检测失败时自动运行对应的安装命令：
- npm: `npm install`
- pnpm: `pnpm install`
- yarn: `yarn install`
- bun: `bun install`

---

## 七、配置流程

### 7.1 标准流程

```bash
# 1. 查看可用模块
aide env list

# 2. 分析项目类型，设置模块
aide env set modules <根据项目选择>

# 3. 配置类型B模块（如有）
aide env set <模块>.<配置项> <值>

# 4. 验证配置
aide env ensure
```

### 7.2 快速配置模板

**前端项目**：
```bash
aide env set modules node,node_deps
aide env set node_deps.path .
aide env ensure
```

**全栈项目**：
```bash
aide env set modules node,node_deps:frontend,node_deps:backend
aide env set node_deps:frontend.path frontend
aide env set node_deps:backend.path backend
aide env ensure
```

**移动端项目**：
```bash
aide env set modules flutter,android
aide env ensure
```

---

## 八、故障排除

### 8.1 模块检测失败

```bash
# 查看详细信息
aide env ensure --verbose
```

### 8.2 重置配置

直接修改 `.aide/config.toml` 或重新运行：

```bash
aide env set modules <新的模块列表>
```

### 8.3 跳过某个模块

从 modules 列表中移除该模块即可。

---

## 九、与命令体系的关系

### 9.1 触发方式

本 skill 由 `/aide:setup` 命令**强制触发**，用于独立的环境配置流程。

### 9.2 命令体系

| 命令 | 说明 | 是否触发本 skill |
|------|------|------------------|
| `/aide:setup` | 环境配置（独立运行） | **是** |
| `/aide:load` | 项目认知载入 | 否 |
| `/aide:docs` | 项目文档管理 | 否 |
| `/aide:run` | 任务执行 | 否（仅触发 aide skill） |

### 9.3 典型使用场景

1. **新项目开始**：执行 `/aide:setup` 进行环境配置
2. **环境问题排查**：当任务执行中遇到环境错误时，重新执行 `/aide:setup`
3. **添加新依赖**：项目引入新技术栈后，更新环境模块配置

### 9.4 配置文件位置

所有环境配置存储在 `.aide/config.toml` 的 `[env]` 节：

```toml
[env]
modules = ["python", "uv", "venv", "requirements"]

[env.venv]
path = ".venv"

[env.requirements]
path = "requirements.txt"
```

> 注：配置文件已自文档化，包含所有配置项的详细注释说明。
