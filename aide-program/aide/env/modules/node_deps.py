"""Node.js 项目依赖检测模块。"""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Any

from aide.env.modules.base import BaseModule, CheckResult, ModuleInfo


class NodeDepsModule(BaseModule):
    """Node.js 项目依赖检测模块（类型B：需要配置）。

    支持多种包管理器：npm, pnpm, yarn, bun
    自动根据锁文件检测包管理器类型
    """

    # 锁文件到包管理器的映射
    LOCK_FILES = {
        "pnpm-lock.yaml": "pnpm",
        "yarn.lock": "yarn",
        "bun.lockb": "bun",
        "package-lock.json": "npm",
    }

    # 包管理器安装命令
    INSTALL_COMMANDS = {
        "npm": ["npm", "install"],
        "pnpm": ["pnpm", "install"],
        "yarn": ["yarn", "install"],
        "bun": ["bun", "install"],
    }

    @property
    def info(self) -> ModuleInfo:
        return ModuleInfo(
            name="node_deps",
            description="Node.js 项目依赖",
            capabilities=["check", "ensure"],
            requires_config=True,
            config_keys=["path"],
        )

    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测 Node.js 项目依赖。"""
        project_path = root / config["path"]

        # 检测 package.json 是否存在
        package_json = project_path / "package.json"
        if not package_json.exists():
            return CheckResult(
                success=False,
                message=f"package.json 不存在: {config['path']}",
                can_ensure=False,
            )

        # 检测 node_modules 是否存在
        node_modules = project_path / "node_modules"
        if not node_modules.exists():
            manager = self._detect_manager(project_path, config)
            return CheckResult(
                success=False,
                message=f"node_modules 不存在",
                can_ensure=True,
            )

        # 检测包管理器
        manager = self._detect_manager(project_path, config)

        return CheckResult(
            success=True,
            version=config["path"],
            message=manager,
        )

    def ensure(self, config: dict[str, Any], root: Path) -> CheckResult:
        """安装 Node.js 项目依赖。"""
        project_path = root / config["path"]
        manager = self._detect_manager(project_path, config)

        # 检测包管理器是否已安装
        if not self._is_manager_installed(manager):
            return CheckResult(
                success=False,
                message=f"{manager} 未安装",
            )

        # 运行安装命令
        install_cmd = self.INSTALL_COMMANDS.get(manager, ["npm", "install"])

        try:
            subprocess.run(
                install_cmd,
                cwd=project_path,
                check=True,
                capture_output=True,
            )
            return CheckResult(
                success=True,
                version=config["path"],
                message=f"已安装 ({manager})",
            )
        except subprocess.CalledProcessError as exc:
            error_msg = exc.stderr.decode() if exc.stderr else str(exc)
            return CheckResult(
                success=False,
                message=f"安装失败: {error_msg[:100]}",
            )

    def _detect_manager(self, project_path: Path, config: dict[str, Any]) -> str:
        """检测包管理器类型。

        优先使用配置指定的 manager，否则根据锁文件自动检测。
        """
        # 优先使用配置指定的 manager
        if "manager" in config:
            return config["manager"]

        # 根据锁文件检测
        for lock_file, manager in self.LOCK_FILES.items():
            if (project_path / lock_file).exists():
                return manager

        # 默认使用 npm
        return "npm"

    def _is_manager_installed(self, manager: str) -> bool:
        """检测包管理器是否已安装。"""
        try:
            subprocess.run(
                [manager, "--version"],
                capture_output=True,
                timeout=10,
            )
            return True
        except (FileNotFoundError, subprocess.TimeoutExpired):
            return False


module = NodeDepsModule()
