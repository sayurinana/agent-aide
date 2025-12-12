# Aide 项目概览

本仓库包含三部分产物：
- **aide-marketplace/**：Claude Code 插件目录（commands + skills）
- **aide-program/**：aide 程序实现（本阶段提供初始化、配置与环境管理）
- **ai-agent-memory/** & **docs/**：原始流程文档与参考资料

当前完成情况：插件与文档已就绪，aide 程序已实现基础 CLI，后续将补充 `aide flow` 与 `aide decide` 细节。

## 快速开始（aide 程序）

### 环境准备
1. 确认已安装 `uv`（0.9+）。
2. 在仓库根目录创建虚拟环境并安装依赖：
   ```bash
   uv venv .venv
   uv pip install -r requirements.txt --python .venv
   ```

### 可用命令
- 初始化配置与 .aide 目录（会写入 `.gitignore`）  
  ```bash
  ./aide-program/aide.sh init
  ```
- 检测运行时环境（不读取配置）  
  ```bash
  ./aide-program/aide.sh env ensure --runtime
  ```
- 检测项目环境，确保 `.venv`、`requirements`、任务文档路径等  
  ```bash
  ./aide-program/aide.sh env ensure
  ```
- 读取/设置配置（示例）  
  ```bash
  ./aide-program/aide.sh config get task.source
  ./aide-program/aide.sh config set task.spec task-spec.md
  ```

> Windows 可使用 `aide-program\\aide.bat`，命令参数一致。

### 配置文件
`aide init` 会生成 `.aide/config.toml`，默认字段：
- `runtime.python_min`：最小 Python 版本（默认 3.11）
- `task.source` / `task.spec`：任务原文档与细则文档默认路径
- `env.venv` / `env.requirements`：虚拟环境与依赖文件位置
- `flow.phases`：流程环节名称（flow/decide 功能尚未实现）

## 未完成功能
- `aide flow` 进度追踪、`aide decide` 待定项 Web 界面尚未实现，后续阶段补充。
- 配置写入时暂不保留注释，必要时可重新运行 `aide init` 重置为模板后再调整。

## 参考
- 需求规格：`aide-requirements.md`
- 设计讨论：`discuss/` 目录（Phase 1/2 已完成，当前进入程序实现阶段）
