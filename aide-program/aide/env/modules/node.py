"""Node.js 环境检测模块。"""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Any

from aide.env.modules.base import BaseModule, CheckResult, ModuleInfo


class NodeModule(BaseModule):
    """Node.js 检测模块（类型A：无需配置）。"""

    @property
    def info(self) -> ModuleInfo:
        return ModuleInfo(
            name="node",
            description="Node.js 运行时",
            capabilities=["check"],
            requires_config=False,
        )

    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测 Node.js 版本。"""
        node_version = self._get_version("node")
        npm_version = self._get_version("npm")

        if not node_version:
            return CheckResult(
                success=False,
                message="node 未安装",
                can_ensure=False,
            )

        # 检查最低版本要求（如果配置了）
        min_version = config.get("min_version")
        if min_version:
            if not self._version_satisfies(node_version, min_version):
                return CheckResult(
                    success=False,
                    version=node_version,
                    message=f"版本不足，要求>={min_version}，当前 {node_version}",
                    can_ensure=False,
                )

        # 构建版本信息
        extra = f"npm {npm_version}" if npm_version else "npm 未安装"
        return CheckResult(
            success=True,
            version=node_version,
            message=extra,
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
                # node: v20.10.0 -> 20.10.0
                # npm: 10.2.3 -> 10.2.3
                output = result.stdout.strip()
                if output.startswith("v"):
                    output = output[1:]
                return output
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


module = NodeModule()
