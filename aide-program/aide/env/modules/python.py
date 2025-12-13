"""Python 环境检测模块。"""

from __future__ import annotations

import platform
from pathlib import Path
from typing import Any

from aide.env.modules.base import BaseModule, CheckResult, ModuleInfo


class PythonModule(BaseModule):
    """Python 解释器检测模块（类型A：无需配置）。"""

    @property
    def info(self) -> ModuleInfo:
        return ModuleInfo(
            name="python",
            description="Python 解释器版本",
            capabilities=["check"],
            requires_config=False,
        )

    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测 Python 版本。"""
        current_version = platform.python_version()
        min_version = config.get("min_version", "3.11")

        current_parts = self._parse_version(current_version)
        min_parts = self._parse_version(min_version)

        if current_parts >= min_parts:
            return CheckResult(
                success=True,
                version=current_version,
                message=f">={min_version}",
            )
        else:
            return CheckResult(
                success=False,
                version=current_version,
                message=f"版本不足，要求>={min_version}，当前 {current_version}",
                can_ensure=False,
            )

    @staticmethod
    def _parse_version(version: str) -> tuple[int, ...]:
        """解析版本号字符串。"""
        parts = []
        for part in version.split("."):
            try:
                parts.append(int(part))
            except ValueError:
                break
        return tuple(parts)


module = PythonModule()
