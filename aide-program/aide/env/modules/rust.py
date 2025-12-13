"""Rust 工具链检测模块。"""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Any

from aide.env.modules.base import BaseModule, CheckResult, ModuleInfo


class RustModule(BaseModule):
    """Rust 工具链检测模块（类型A：无需配置）。"""

    @property
    def info(self) -> ModuleInfo:
        return ModuleInfo(
            name="rust",
            description="Rust 工具链",
            capabilities=["check"],
            requires_config=False,
        )

    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测 Rust 工具链（rustc 和 cargo）。"""
        rustc_version = self._get_version("rustc")
        cargo_version = self._get_version("cargo")

        if not rustc_version:
            return CheckResult(
                success=False,
                message="rustc 未安装",
                can_ensure=False,
            )

        if not cargo_version:
            return CheckResult(
                success=False,
                message="cargo 未安装",
                can_ensure=False,
            )

        # 检查最低版本要求（如果配置了）
        min_version = config.get("min_version")
        if min_version:
            if not self._version_satisfies(rustc_version, min_version):
                return CheckResult(
                    success=False,
                    version=rustc_version,
                    message=f"版本不足，要求>={min_version}，当前 {rustc_version}",
                    can_ensure=False,
                )

        return CheckResult(
            success=True,
            version=rustc_version,
            message=f"cargo {cargo_version}",
        )

    def _get_version(self, cmd: str) -> str | None:
        """获取命令版本。"""
        try:
            result = subprocess.run(
                [cmd, "--version"],
                capture_output=True,
                text=True,
                timeout=10,
            )
            if result.returncode == 0:
                # rustc 1.75.0 (xxx) -> 1.75.0
                # cargo 1.75.0 (xxx) -> 1.75.0
                output = result.stdout.strip()
                parts = output.split()
                if len(parts) >= 2:
                    return parts[1]
            return None
        except (FileNotFoundError, subprocess.TimeoutExpired):
            return None

    def _version_satisfies(self, current: str, minimum: str) -> bool:
        """检查版本是否满足最低要求。"""
        current_parts = self._parse_version(current)
        min_parts = self._parse_version(minimum)
        return current_parts >= min_parts

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


module = RustModule()
