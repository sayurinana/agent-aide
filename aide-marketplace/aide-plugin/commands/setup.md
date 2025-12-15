# Aide 环境配置

你正在执行 Aide 环境配置流程。这是一个**独立运行**的命令，专注于环境依赖分析、配置、检测和修复。

## 前置准备

**首先触发 `env-config` skill 学习详细的环境配置方法。**

这是必要步骤，确保你了解各种环境模块的配置和故障排除方法。

---

## 开始

### 1. 检查 aide 运行时环境

```bash
aide env ensure --runtime
```

如果失败，根据 env-config skill 的指导进行修复。

### 2. 初始化 aide 目录

```bash
aide init
```

### 3. 分析项目依赖

探索项目结构，识别项目类型和依赖：

- **Python 项目**：检查 `requirements.txt`、`pyproject.toml`、`setup.py`
- **Node.js 项目**：检查 `package.json`
- **Rust 项目**：检查 `Cargo.toml`
- **Flutter 项目**：检查 `pubspec.yaml`
- **多语言项目**：识别所有涉及的语言和框架

### 4. 配置环境模块

根据项目类型配置相应的环境模块：

```bash
# 示例：Python 项目
aide env set modules python,uv,venv,requirements
aide env set venv.path .venv
aide env set requirements.path requirements.txt

# 示例：Node.js 项目
aide env set modules node,node_deps
aide env set node_deps.path .
aide env set node_deps.manager npm

# 示例：Rust 项目
aide env set modules rust
```

### 5. 执行环境检测

```bash
aide env ensure
```

### 6. 处理问题

如果检测失败：

1. 阅读错误信息
2. 参考 env-config skill 的故障排除指南
3. 尝试修复（最多 3 次）
4. 如果无法自动修复，向用户说明问题并请求帮助

---

## 完成

环境配置完成后，向用户汇报：

```
环境配置完成：
- 项目类型：[识别的项目类型]
- 启用模块：[模块列表]
- 检测结果：全部通过 / 部分问题（详情）
```

---

## 注意事项

- 此命令是**独立运行**的，通常在新项目开始时执行一次
- 后续任务执行时不会再专门处理环境问题，除非遇到意外错误
- 如果用户报告环境问题，可以重新执行此命令进行诊断和修复
