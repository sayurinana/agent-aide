"""Flutter SDK 检测模块。"""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Any

from aide.env.modules.base import BaseModule, CheckResult, ModuleInfo


class FlutterModule(BaseModule):
    """Flutter SDK 检测模块（类型A：无需配置）。"""

    @property
    def info(self) -> ModuleInfo:
        return ModuleInfo(
            name="flutter",
            description="Flutter SDK",
            capabilities=["check"],
            requires_config=False,
        )

    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测 Flutter SDK。"""
        flutter_version = self._get_flutter_version()
        dart_version = self._get_dart_version()

        if not flutter_version:
            return CheckResult(
                success=False,
                message="flutter 未安装",
                can_ensure=False,
            )

        # 检查最低版本要求（如果配置了）
        min_version = config.get("min_version")
        if min_version:
            if not self._version_satisfies(flutter_version, min_version):
                return CheckResult(
                    success=False,
                    version=flutter_version,
                    message=f"版本不足，要求>={min_version}，当前 {flutter_version}",
                    can_ensure=False,
                )

        # 构建版本信息
        extra = f"dart {dart_version}" if dart_version else ""
        return CheckResult(
            success=True,
            version=flutter_version,
            message=extra,
        )

    def _get_flutter_version(self) -> str | None:
        """获取 Flutter 版本。"""
        try:
            result = subprocess.run(
                ["flutter", "--version", "--machine"],
                capture_output=True,
                text=True,
                timeout=30,
            )
            if result.returncode == 0:
                # 尝试解析 JSON 输出
                import json
                try:
                    data = json.loads(result.stdout)
                    return data.get("frameworkVersion")
                except json.JSONDecodeError:
                    pass

            # 回退到普通版本输出
            result = subprocess.run(
                ["flutter", "--version"],
                capture_output=True,
                text=True,
                timeout=30,
            )
            if result.returncode == 0:
                # Flutter 3.16.0 • channel stable • ...
                output = result.stdout.strip()
                lines = output.split("\n")
                if lines:
                    parts = lines[0].split()
                    if len(parts) >= 2 and parts[0] == "Flutter":
                        return parts[1]
            return None
        except (FileNotFoundError, subprocess.TimeoutExpired):
            return None

    def _get_dart_version(self) -> str | None:
        """获取 Dart 版本。"""
        try:
            result = subprocess.run(
                ["dart", "--version"],
                capture_output=True,
                text=True,
                timeout=10,
            )
            if result.returncode == 0:
                # Dart SDK version: 3.2.0 (stable) ...
                output = result.stdout.strip()
                if not output:
                    output = result.stderr.strip()  # dart 有时输出到 stderr
                parts = output.split()
                for i, part in enumerate(parts):
                    if part == "version:" and i + 1 < len(parts):
                        return parts[i + 1]
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


module = FlutterModule()
