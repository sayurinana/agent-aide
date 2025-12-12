# 背景与范围

- 已审阅 `aide-requirements.md`、`ai-agent-memory/` 全部文件、`docs/` 全部文件、`aide-marketplace/` 现有 commands 与 SKILL、`discuss/01-03`、`reply/re-03.md`、`CLAUDE.md`/`AGENTS.md`。当前 Phase1/2 已完成，re-03 调整已落实。
- 任务：在 `aide-program/` 下实现 aide 程序系统的入口封装与 Python 核心功能，**暂不实现/细化 aide flow 与 aide decide**。

# 设计聚焦

- 入口封装：提供 `aide.sh`、`aide.bat` 调用 `.venv` 下的 Python，执行 `python -m aide`。
- CLI 范围（本阶段）：`aide init`、`aide env ensure [--runtime]`、`aide config get/set`。flow/decide 仅预留配置，不实现逻辑。
- 输出规范：统一使用 `✓/⚠/✗/→` 前缀，遵循“静默即成功”原则，中文提示。
- 配置管理：`.aide/config.toml`，默认包含 `runtime`（python_min）、`task`（source/spec 默认 `task-now.md`/`task-spec.md`）、`env`（venv/requirements）、`flow.phases`。支持 `config get/set`，缺失时自动创建默认配置与 `.aide/` 目录。
- 环境检测：
  - `--runtime`：检测 Python 版本、`uv` 可用性，不读配置。
  - 完整 `ensure`：读取配置，确保 `.aide/` 结构、`.gitignore` 条目、`.venv`（用 `uv venv` 创建）、`requirements.txt` 存在并通过 `uv pip install -r` 安装依赖，输出任务文档路径提示。
- 依赖：尽量标准库，TOML 写入使用轻量依赖 `tomli-w`，列入 `requirements.txt` 并通过 uv 安装。

# 目录与模块规划（aide-program/）

```
aide-program/
├── aide.sh / aide.bat          # 入口脚本，调用 .venv 下 Python
└── aide/
    ├── __init__.py
    ├── __main__.py             # 支持 python -m aide
    ├── main.py                 # CLI 解析与命令分发
    ├── core/
    │   ├── __init__.py
    │   ├── config.py           # 配置读写、默认生成、gitignore 维护
    │   └── output.py           # 统一输出前缀
    └── env/
        ├── __init__.py
        └── ensure.py           # 环境检测与依赖安装
```

# 开发计划

1) 初始化 `aide-program/` 目录与入口脚本骨架；补充 `requirements.txt`、`.venv`（已建）。  
2) 实现核心模块：输出工具、配置管理、环境检测、CLI 路由。  
3) 补充根级 `README.md`，描述用法/约束/未实现部分；必要自检（使用 `.venv/bin/python -m aide ...`）。  
4) 暂不实现 flow/decide 细节，后续再扩展。

# 实施进展（同步）

- 已创建 `aide-program/` 目录与入口脚本（bash/bat），通过 `.venv` 调用 `python -m aide`。
- 实现模块：`core/output.py`（统一输出）、`core/config.py`（默认配置生成、读取/写入、.gitignore 维护）、`env/ensure.py`（runtime 检查、venv/依赖处理）、`main.py`（init/env/config CLI）。
- 生成默认配置 `.aide/config.toml`，补充 `.gitignore`，`requirements.txt` 添加 `tomli-w` 并通过 uv 安装。
- 自检：`aide init`、`aide env ensure --runtime`、`aide env ensure`、`aide config get task.source` 均通过，输出符合 `✓/⚠/✗/→` 规范。
