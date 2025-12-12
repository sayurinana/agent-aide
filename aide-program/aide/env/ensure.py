"""环境检测与修复逻辑。"""

from __future__ import annotations

import platform
import subprocess
import sys
from pathlib import Path

from aide.core import output
from aide.core.config import ConfigManager


class EnvManager:
    def __init__(self, root: Path):
        self.root = root

    def ensure(self, runtime_only: bool, cfg: ConfigManager) -> bool:
        """运行环境检测入口。"""
        required_py = self._get_required_python(cfg, runtime_only)
        if not self._check_python_version(required_py):
            return False
        uv_version = self._check_uv()
        if uv_version is None:
            return False

        if runtime_only:
            output.ok(f"运行时环境就绪 (python:{platform.python_version()}, uv:{uv_version})")
            return True

        config = cfg.ensure_config()
        cfg.ensure_gitignore()

        env_config = config.get("env", {})
        venv_path = self.root / env_config.get("venv", ".venv")
        req_path = self.root / env_config.get("requirements", "requirements.txt")

        self._ensure_requirements_file(req_path)
        if not self._ensure_venv(venv_path):
            return False
        if not self._install_requirements(venv_path, req_path):
            return False

        task_config = config.get("task", {})
        output.info(f"任务原文档: {task_config.get('source', 'task-now.md')}")
        output.info(f"任务细则文档: {task_config.get('spec', 'task-spec.md')}")
        output.ok(f"环境就绪 (python:{platform.python_version()}, uv:{uv_version}, venv:{venv_path})")
        return True

    @staticmethod
    def _get_required_python(cfg: ConfigManager, runtime_only: bool) -> str:
        if runtime_only:
            return "3.11"
        data = cfg.load_config()
        runtime = data.get("runtime", {})
        return str(runtime.get("python_min", "3.11"))

    @staticmethod
    def _parse_version(version: str) -> tuple[int, ...]:
        parts = []
        for part in version.split("."):
            try:
                parts.append(int(part))
            except ValueError:
                break
        return tuple(parts)

    def _check_python_version(self, required: str) -> bool:
        current = self._parse_version(platform.python_version())
        target = self._parse_version(required)
        if current >= target:
            return True
        output.err(f"Python 版本不足，要求>={required}，当前 {platform.python_version()}")
        return False

    def _check_uv(self) -> str | None:
        try:
            result = subprocess.run(
                ["uv", "--version"],
                check=True,
                capture_output=True,
                text=True,
            )
            return result.stdout.strip()
        except (subprocess.CalledProcessError, FileNotFoundError) as exc:
            output.err(f"未检测到 uv，请先安装（{exc}）")
            return None

    def _ensure_venv(self, venv_path: Path) -> bool:
        if venv_path.exists():
            return True
        output.info(f"创建虚拟环境: {venv_path}")
        try:
            subprocess.run(["uv", "venv", str(venv_path)], check=True)
            output.ok("已创建虚拟环境")
            return True
        except subprocess.CalledProcessError as exc:
            output.err(f"创建虚拟环境失败: {exc}")
            return False

    @staticmethod
    def _ensure_requirements_file(req_path: Path) -> None:
        if req_path.exists():
            return
        req_path.write_text("# 在此添加依赖\n", encoding="utf-8")
        output.warn(f"未找到 {req_path.name}，已创建空文件")

    def _install_requirements(self, venv_path: Path, req_path: Path) -> bool:
        if not req_path.exists():
            output.err(f"缺少 {req_path}")
            return False
        cmd = ["uv", "pip", "install", "-r", str(req_path), "--python", str(venv_path)]
        output.info("安装依赖（uv pip install -r requirements.txt）")
        try:
            subprocess.run(cmd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.STDOUT)
            return True
        except subprocess.CalledProcessError as exc:
            output.err(f"安装依赖失败: {exc}")
            return False
