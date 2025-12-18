"""配置管理：生成默认配置、读取/写入配置、维护 .aide 目录与 .gitignore。"""

from __future__ import annotations

from pathlib import Path
from typing import Any
import tomllib

from tomli_w import dumps as toml_dumps

from aide.core import output

DEFAULT_CONFIG = """################################################################################
#                           Aide 配置文件 (config.toml)
################################################################################
#
# 本配置文件为 Aide 工作流体系的核心配置，由 `aide init` 命令生成。
# 所有配置项都有详细说明，用户可仅通过本文件了解所有支持的功能。
#
# 配置操作说明：
#   - 读取配置：aide config get <key>        例：aide config get flow.phases
#   - 设置配置：aide config set <key> <value> 例：aide config set task.source "my-task.md"
#   - 支持点号分隔的嵌套键，如：env.venv.path
#
# 注意：LLM 不应直接编辑此文件，必须通过 aide 命令操作。
#
################################################################################

################################################################################
# [general] - 通用配置
################################################################################
# 控制 Aide 的全局行为设置。

[general]
# 是否在 .gitignore 中忽略 .aide 目录
# - true（默认）：自动添加 .aide/ 到 .gitignore，不跟踪 aide 状态
# - false：不修改 .gitignore，允许 git 跟踪 .aide 目录
#   适用于需要在多设备同步 aide 状态的场景
gitignore_aide = true

################################################################################
# [runtime] - Aide 运行时要求
################################################################################
# 定义 aide 程序本身的运行环境要求。
# 这些配置用于 `aide env ensure --runtime` 检测。

[runtime]
# Python 最低版本要求
# 格式："主版本.次版本"，如 "3.11"、"3.12"
python_min = "3.11"

# 是否要求使用 uv 包管理器
# - true：检测 uv 是否安装
# - false：不检测 uv
use_uv = true

################################################################################
# [task] - 任务文档配置
################################################################################
# 定义任务相关文档的默认路径。

[task]
# 任务原文档路径（用户提供的原始任务描述）
# /aide:prep 命令在未指定参数时读取此文件
source = "task-now.md"

# 任务细则文档路径（aide 生成的可执行任务细则）
# /aide:exec 命令在未指定参数时读取此文件
spec = "task-spec.md"

# 复杂任务计划文档目录
# 当任务被拆分为多个子计划时，存放：
#   - guide.md: 任务计划总导览
#   - spec-01.md, spec-02.md, ...: 各子计划细则
plans_path = ".aide/task-plans/"

################################################################################
# [env] - 环境检测配置
################################################################################
# 配置项目的开发环境检测模块。
#
# 模块分为两类：
#   类型A - 无需配置即可检测（全局工具）：
#     python  - Python 解释器版本检测
#     uv      - uv 包管理器检测
#     rust    - Rust 工具链（rustc + cargo）检测
#     node    - Node.js 运行时检测
#     flutter - Flutter SDK 检测
#     android - Android SDK 检测
#
#   类型B - 需要配置路径才能检测（项目级）：
#     venv         - Python 虚拟环境
#     requirements - Python 依赖文件（requirements.txt）
#     node_deps    - Node.js 项目依赖（package.json）
#
# 模块实例化（多项目场景）：
#   格式：模块类型:实例名
#   例如：node_deps:frontend、node_deps:backend
#   各实例独立配置：[env."node_deps:frontend"]

[env]
# 启用的模块列表
# 只有列在此处的模块才会被 `aide env ensure` 检测
# 示例：
#   - 纯 Python 项目：["python", "uv", "venv", "requirements"]
#   - Rust 项目：["rust"]
#   - Node.js 项目：["node", "node_deps"]
#   - 多语言项目：["python", "uv", "venv", "node", "node_deps:frontend"]
modules = ["python", "uv", "venv", "requirements"]

# -------------------------------
# 以下为各模块的详细配置示例
# -------------------------------

# Python 版本要求（可选，默认使用 runtime.python_min）
# [env.python]
# min_version = "3.11"

# Rust 版本要求（可选）
# [env.rust]
# min_version = "1.70"

# Node.js 版本要求（可选）
# [env.node]
# min_version = "18"

# Flutter 版本要求（可选）
# [env.flutter]
# min_version = "3.0"

# 虚拟环境配置（类型B模块，启用时必须配置）
# path: 虚拟环境目录路径，相对于项目根目录
[env.venv]
path = ".venv"

# Python 依赖文件配置（类型B模块，启用时必须配置）
# path: requirements.txt 文件路径，相对于项目根目录
[env.requirements]
path = "requirements.txt"

# Node.js 依赖配置示例（类型B模块）
# 注意：需先在 modules 中添加 "node_deps" 才会生效
# [env.node_deps]
# path = "."           # package.json 所在目录
# manager = "npm"      # 包管理器：npm、yarn、pnpm

# 多项目实例化示例（前端 + 后端分离项目）
# modules = ["node_deps:frontend", "node_deps:backend"]
#
# [env."node_deps:frontend"]
# path = "frontend"
# manager = "pnpm"
#
# [env."node_deps:backend"]
# path = "backend"
# manager = "npm"

################################################################################
# [docs] - 项目文档配置（面向 LLM）
################################################################################
# 配置面向 LLM 的项目文档系统。
# 这些文档用于帮助 LLM 理解项目结构，支持增量更新。

[docs]
# 项目文档目录路径
# 存放总导览和各区块文档
# 默认：.aide/project-docs
path = ".aide/project-docs"

# 区块计划文档路径
# 记录文档区块划分和生成进度，用于多对话续接
# 默认：.aide/project-docs/block-plan.md
block_plan_path = ".aide/project-docs/block-plan.md"

################################################################################
# [flow] - 流程追踪配置
################################################################################
# 配置任务执行流程的追踪和校验。

[flow]
# 环节名称列表（有序）
# 定义任务执行的标准流程，用于校验环节跳转合法性
# 标准环节：
#   task-optimize - 任务优化阶段（/aide:prep 使用）
#   flow-design   - 流程设计（创建流程图）
#   impl          - 迭代实现
#   verify        - 验证交付
#   docs          - 文档更新
#   confirm       - 用户确认（审阅与返工）
#   finish        - 收尾
phases = ["task-optimize", "flow-design", "impl", "verify", "docs", "confirm", "finish"]

# 流程图目录路径
# 存放 PlantUML 源文件（.puml）和生成的图片（.png）
# flow-design 阶段会校验此目录下的 .puml 文件
diagram_path = ".aide/diagrams"

################################################################################
# [plantuml] - PlantUML 配置
################################################################################
# 配置 PlantUML 流程图生成工具。

[plantuml]
# PlantUML jar 文件路径
# 支持绝对路径或相对于 aide-program 目录的相对路径
# 默认使用 aide-program/lib/plantuml.jar
jar_path = "lib/plantuml.jar"

# Java 命令路径（可选）
# 默认使用系统 PATH 中的 java
# java_path = "/usr/bin/java"

################################################################################
# [decide] - 待定项确认配置
################################################################################
# 配置待定项 Web 确认界面。

[decide]
# HTTP 服务起始端口
# 如果端口被占用，会自动探测下一个可用端口
port = 3721

# 监听地址
# - "127.0.0.1"（默认）：仅本机访问
# - "0.0.0.0"：允许外部访问（注意安全风险）
bind = "127.0.0.1"

# 自定义访问地址（可选）
# 为空时自动生成为 http://{bind}:{port}
# 适用于反向代理或自定义域名场景
# 示例：url = "https://decide.example.com"
url = ""

# 超时时间（秒）
# - 0（默认）：不超时，等待用户完成
# - >0：超时后自动关闭服务
timeout = 0

################################################################################
# 配置文件版本信息
################################################################################
# 本配置文件格式版本，用于未来兼容性检查
# _version = "1.1.0"
"""


class ConfigManager:
    def __init__(self, root: Path):
        self.root = root
        self.aide_dir = self.root / ".aide"
        self.config_path = self.aide_dir / "config.toml"
        self.decisions_dir = self.aide_dir / "decisions"
        self.logs_dir = self.aide_dir / "logs"

    def ensure_base_dirs(self) -> None:
        self.aide_dir.mkdir(parents=True, exist_ok=True)
        self.decisions_dir.mkdir(parents=True, exist_ok=True)
        self.logs_dir.mkdir(parents=True, exist_ok=True)

    def ensure_gitignore(self) -> None:
        """根据配置决定是否在 .gitignore 中添加 .aide/ 忽略项。"""
        # 读取配置，默认为 True（忽略 .aide 目录）
        config = self.load_config()
        gitignore_aide = self._walk_get(config, "general.gitignore_aide")
        if gitignore_aide is None:
            gitignore_aide = True  # 默认值

        if not gitignore_aide:
            # 配置为 False，不添加忽略项
            return

        gitignore_path = self.root / ".gitignore"
        marker = ".aide/"
        if gitignore_path.exists():
            content = gitignore_path.read_text(encoding="utf-8").splitlines()
            if any(line.strip() == marker for line in content):
                return
            content.append(marker)
            gitignore_path.write_text("\n".join(content) + "\n", encoding="utf-8")
        else:
            gitignore_path.write_text(f"{marker}\n", encoding="utf-8")

    def ensure_config(self) -> dict[str, Any]:
        self.ensure_base_dirs()
        if not self.config_path.exists():
            self.config_path.write_text(DEFAULT_CONFIG, encoding="utf-8")
            output.ok("已创建默认配置 .aide/config.toml")
        return self.load_config()

    def load_config(self) -> dict[str, Any]:
        if not self.config_path.exists():
            return {}
        try:
            with self.config_path.open("rb") as f:
                return tomllib.load(f)
        except Exception as exc:  # pragma: no cover - 兼容性输出
            output.err(f"读取配置失败: {exc}")
            return {}

    def get_value(self, key: str) -> Any:
        data = self.load_config()
        return self._walk_get(data, key)

    def set_value(self, key: str, value: Any) -> None:
        self.ensure_config()
        self._update_config_value(key, value)
        output.ok(f"已更新 {key} = {value!r}")

    def _update_config_value(self, key: str, value: Any) -> None:
        """保守更新配置值，保留注释和格式。"""
        import re

        content = self.config_path.read_text(encoding="utf-8")
        parts = key.split(".")

        # 格式化值为 TOML 格式
        if isinstance(value, bool):
            toml_value = "true" if value else "false"
        elif isinstance(value, str):
            toml_value = f'"{value}"'
        elif isinstance(value, (int, float)):
            toml_value = str(value)
        elif isinstance(value, list):
            toml_value = toml_dumps({"_": value}).split("=", 1)[1].strip()
        else:
            toml_value = toml_dumps({"_": value}).split("=", 1)[1].strip()

        new_content = None
        count = 0

        if len(parts) == 1:
            # 顶层键：key = value
            pattern = rf'^({re.escape(parts[0])}\s*=\s*)(.*)$'
            new_content, count = re.subn(pattern, rf'\g<1>{toml_value}', content, count=1, flags=re.MULTILINE)
        elif len(parts) >= 2:
            # 两级或三级键：找到对应 section，然后替换其中的 key
            if len(parts) == 2:
                section = parts[0]
                subkey = parts[1]
            else:
                section = ".".join(parts[:-1])
                subkey = parts[-1]

            # 找到 section 的起始位置
            section_pattern = rf'^\[{re.escape(section)}\]\s*$'
            section_match = re.search(section_pattern, content, flags=re.MULTILINE)

            if section_match:
                section_start = section_match.end()
                # 找到下一个 section 的位置（或文件末尾）
                next_section = re.search(r'^\[', content[section_start:], flags=re.MULTILINE)
                if next_section:
                    section_end = section_start + next_section.start()
                else:
                    section_end = len(content)

                # 在 section 范围内查找并替换 key
                section_content = content[section_start:section_end]
                key_pattern = rf'^({re.escape(subkey)}\s*=\s*)(.*)$'
                new_section, count = re.subn(key_pattern, rf'\g<1>{toml_value}', section_content, count=1, flags=re.MULTILINE)

                if count > 0:
                    new_content = content[:section_start] + new_section + content[section_end:]

        if count == 0 or new_content is None:
            # 键不存在，需要添加（回退到传统方式，会丢失注释）
            data = self.load_config()
            self._walk_set(data, key, value)
            self._write_config(data)
            output.warn("配置键不存在，已添加（注释可能丢失）")
        else:
            self.config_path.write_text(new_content, encoding="utf-8")

    def _write_config(self, data: dict[str, Any]) -> None:
        """完全重写配置文件（会丢失注释，仅在添加新键时使用）。"""
        self.config_path.write_text(toml_dumps(data), encoding="utf-8")

    @staticmethod
    def _walk_get(data: dict[str, Any], dotted_key: str) -> Any:
        current: Any = data
        for part in dotted_key.split("."):
            if not isinstance(current, dict):
                return None
            if part not in current:
                return None
            current = current[part]
        return current

    @staticmethod
    def _walk_set(data: dict[str, Any], dotted_key: str, value: Any) -> None:
        parts = dotted_key.split(".")
        current = data
        for part in parts[:-1]:
            if part not in current or not isinstance(current[part], dict):
                current[part] = {}
            current = current[part]
        current[parts[-1]] = value
