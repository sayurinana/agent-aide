"""配置管理：生成默认配置、读取/写入配置、维护 .aide 目录与 .gitignore。"""

from __future__ import annotations

from pathlib import Path
from typing import Any
import tomllib

from tomli_w import dumps as toml_dumps

from aide.core import output

DEFAULT_CONFIG = """# Aide 默认配置（由 aide init 生成）
# runtime: aide 自身运行要求
# task: 任务文档路径
# env: 环境模块配置
# flow: 环节名称列表，供流程校验使用

[runtime]
python_min = "3.11"
use_uv = true

[task]
source = "task-now.md"
spec = "task-spec.md"

[env]
# 启用的模块列表
modules = ["python", "uv", "venv", "requirements"]

# Python 版本要求（可选，默认使用 runtime.python_min）
# [env.python]
# min_version = "3.11"

# 虚拟环境配置（类型B模块，必须配置）
[env.venv]
path = ".venv"

# 依赖文件配置（类型B模块，必须配置）
[env.requirements]
path = "requirements.txt"

[flow]
phases = ["task-optimize", "flow-design", "impl", "verify", "docs", "finish"]
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
        data = self.ensure_config()
        self._walk_set(data, key, value)
        self._write_config(data)
        output.ok(f"已更新 {key} = {value!r}")

    def _write_config(self, data: dict[str, Any]) -> None:
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
