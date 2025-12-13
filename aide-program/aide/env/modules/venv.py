"""Python 虚拟环境模块。"""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Any

from aide.env.modules.base import BaseModule, CheckResult, ModuleInfo


class VenvModule(BaseModule):
    """Python 虚拟环境模块（类型B：需要配置）。"""

    @property
    def info(self) -> ModuleInfo:
        return ModuleInfo(
            name="venv",
            description="Python 虚拟环境",
            capabilities=["check", "ensure"],
            requires_config=True,
            config_keys=["path"],
        )

    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测虚拟环境是否存在。"""
        venv_path = root / config["path"]

        if not venv_path.exists():
            return CheckResult(
                success=False,
                message=f"虚拟环境不存在: {config['path']}",
                can_ensure=True,
            )

        # 检查是否是有效的虚拟环境
        python_path = venv_path / "bin" / "python"
        if not python_path.exists():
            python_path = venv_path / "Scripts" / "python.exe"  # Windows

        if not python_path.exists():
            return CheckResult(
                success=False,
                message=f"无效的虚拟环境: {config['path']}",
                can_ensure=True,
            )

        return CheckResult(
            success=True,
            version=config["path"],
        )

    def ensure(self, config: dict[str, Any], root: Path) -> CheckResult:
        """创建虚拟环境。"""
        venv_path = root / config["path"]

        try:
            subprocess.run(
                ["uv", "venv", str(venv_path)],
                check=True,
                capture_output=True,
            )
            return CheckResult(
                success=True,
                version=config["path"],
                message="已创建",
            )
        except FileNotFoundError:
            return CheckResult(
                success=False,
                message="创建失败: uv 未安装",
            )
        except subprocess.CalledProcessError as exc:
            return CheckResult(
                success=False,
                message=f"创建失败: {exc.stderr.decode() if exc.stderr else exc}",
            )


module = VenvModule()
