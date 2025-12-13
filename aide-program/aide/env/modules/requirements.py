"""Python 依赖管理模块。"""

from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Any

from aide.env.modules.base import BaseModule, CheckResult, ModuleInfo


class RequirementsModule(BaseModule):
    """Python 依赖管理模块（类型B：需要配置）。"""

    @property
    def info(self) -> ModuleInfo:
        return ModuleInfo(
            name="requirements",
            description="Python 依赖管理",
            capabilities=["check", "ensure"],
            requires_config=True,
            config_keys=["path"],
        )

    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测 requirements.txt 是否存在。"""
        req_path = root / config["path"]

        if not req_path.exists():
            return CheckResult(
                success=False,
                message=f"文件不存在: {config['path']}",
                can_ensure=True,
            )

        return CheckResult(
            success=True,
            version=config["path"],
        )

    def ensure(self, config: dict[str, Any], root: Path) -> CheckResult:
        """创建空的 requirements.txt 并安装依赖。"""
        req_path = root / config["path"]

        # 如果文件不存在，创建空文件
        if not req_path.exists():
            req_path.write_text("# 在此添加依赖\n", encoding="utf-8")

        # 获取 venv 路径（从同级配置中获取）
        venv_config = config.get("_venv_path")
        if not venv_config:
            # 尝试使用默认路径
            venv_path = root / ".venv"
        else:
            venv_path = root / venv_config

        if not venv_path.exists():
            return CheckResult(
                success=False,
                message="虚拟环境不存在，请先创建",
            )

        # 安装依赖
        try:
            subprocess.run(
                ["uv", "pip", "install", "-r", str(req_path), "--python", str(venv_path)],
                check=True,
                capture_output=True,
            )
            return CheckResult(
                success=True,
                version=config["path"],
                message="已安装",
            )
        except FileNotFoundError:
            return CheckResult(
                success=False,
                message="安装失败: uv 未安装",
            )
        except subprocess.CalledProcessError as exc:
            stderr = exc.stderr.decode() if exc.stderr else str(exc)
            return CheckResult(
                success=False,
                message=f"安装失败: {stderr}",
            )


module = RequirementsModule()
