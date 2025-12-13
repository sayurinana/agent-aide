"""uv 包管理器检测模块。"""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Any

from aide.env.modules.base import BaseModule, CheckResult, ModuleInfo


class UvModule(BaseModule):
    """uv 包管理器检测模块（类型A：无需配置）。"""

    @property
    def info(self) -> ModuleInfo:
        return ModuleInfo(
            name="uv",
            description="uv 包管理器",
            capabilities=["check"],
            requires_config=False,
        )

    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测 uv 是否可用。"""
        try:
            result = subprocess.run(
                ["uv", "--version"],
                check=True,
                capture_output=True,
                text=True,
            )
            version = result.stdout.strip()
            return CheckResult(
                success=True,
                version=version,
            )
        except FileNotFoundError:
            return CheckResult(
                success=False,
                message="未安装，请先安装 uv",
                can_ensure=False,
            )
        except subprocess.CalledProcessError as exc:
            return CheckResult(
                success=False,
                message=f"执行失败: {exc}",
                can_ensure=False,
            )


module = UvModule()
