"""Android 开发环境检测模块。"""

from __future__ import annotations

import os
import subprocess
from pathlib import Path
from typing import Any

from aide.env.modules.base import BaseModule, CheckResult, ModuleInfo


class AndroidModule(BaseModule):
    """Android 开发环境检测模块（类型A：无需配置）。

    检测 Android SDK 和相关工具：
    - ANDROID_HOME / ANDROID_SDK_ROOT 环境变量
    - Android SDK 目录结构
    - 关键工具：adb, aapt, sdkmanager
    """

    @property
    def info(self) -> ModuleInfo:
        return ModuleInfo(
            name="android",
            description="Android SDK",
            capabilities=["check"],
            requires_config=False,
        )

    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测 Android 开发环境。"""
        # 检测 ANDROID_HOME 或 ANDROID_SDK_ROOT
        sdk_root = self._get_sdk_root()
        if not sdk_root:
            return CheckResult(
                success=False,
                message="ANDROID_HOME 或 ANDROID_SDK_ROOT 未设置",
                can_ensure=False,
            )

        sdk_path = Path(sdk_root)
        if not sdk_path.exists():
            return CheckResult(
                success=False,
                message=f"Android SDK 目录不存在: {sdk_root}",
                can_ensure=False,
            )

        # 检测关键目录
        platform_tools = sdk_path / "platform-tools"
        build_tools = sdk_path / "build-tools"
        platforms = sdk_path / "platforms"

        missing = []
        if not platform_tools.exists():
            missing.append("platform-tools")
        if not build_tools.exists():
            missing.append("build-tools")
        if not platforms.exists():
            missing.append("platforms")

        if missing:
            return CheckResult(
                success=False,
                message=f"缺少 SDK 组件: {', '.join(missing)}",
                can_ensure=False,
            )

        # 获取版本信息
        build_tools_versions = self._get_build_tools_versions(build_tools)
        platform_versions = self._get_platform_versions(platforms)

        # 检测 adb
        adb_version = self._get_adb_version(platform_tools)

        # 构建版本信息
        version_info = []
        if adb_version:
            version_info.append(f"adb {adb_version}")
        if build_tools_versions:
            version_info.append(f"build-tools {build_tools_versions[0]}")
        if platform_versions:
            version_info.append(f"API {platform_versions[0]}")

        return CheckResult(
            success=True,
            version=sdk_root,
            message=", ".join(version_info) if version_info else None,
        )

    def _get_sdk_root(self) -> str | None:
        """获取 Android SDK 根目录。"""
        return os.environ.get("ANDROID_HOME") or os.environ.get("ANDROID_SDK_ROOT")

    def _get_adb_version(self, platform_tools: Path) -> str | None:
        """获取 adb 版本。"""
        adb_path = platform_tools / "adb"
        if not adb_path.exists():
            return None

        try:
            result = subprocess.run(
                [str(adb_path), "version"],
                capture_output=True,
                text=True,
                timeout=10,
            )
            if result.returncode == 0:
                # Android Debug Bridge version 1.0.41
                lines = result.stdout.strip().split("\n")
                if lines:
                    parts = lines[0].split()
                    if len(parts) >= 5:
                        return parts[4]
            return None
        except (subprocess.TimeoutExpired, Exception):
            return None

    def _get_build_tools_versions(self, build_tools: Path) -> list[str]:
        """获取已安装的 build-tools 版本列表（降序）。"""
        if not build_tools.exists():
            return []

        versions = []
        for item in build_tools.iterdir():
            if item.is_dir() and item.name[0].isdigit():
                versions.append(item.name)

        return sorted(versions, reverse=True)

    def _get_platform_versions(self, platforms: Path) -> list[str]:
        """获取已安装的 platform 版本列表（降序）。"""
        if not platforms.exists():
            return []

        versions = []
        for item in platforms.iterdir():
            if item.is_dir() and item.name.startswith("android-"):
                api_level = item.name.replace("android-", "")
                versions.append(api_level)

        return sorted(versions, key=lambda x: int(x) if x.isdigit() else 0, reverse=True)


module = AndroidModule()
