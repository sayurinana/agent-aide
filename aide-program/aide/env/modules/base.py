"""模块基类定义。"""

from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


@dataclass
class CheckResult:
    """检测结果。"""

    success: bool
    version: str | None = None
    message: str | None = None
    can_ensure: bool = False  # 失败时是否可修复


@dataclass
class ModuleInfo:
    """模块元信息。"""

    name: str
    description: str
    capabilities: list[str] = field(default_factory=lambda: ["check"])
    requires_config: bool = False  # 是否需要配置（类型B模块）
    config_keys: list[str] = field(default_factory=list)  # 需要的配置键

    @property
    def can_ensure(self) -> bool:
        """是否支持 ensure 操作。"""
        return "ensure" in self.capabilities


class BaseModule(ABC):
    """模块基类。"""

    @property
    @abstractmethod
    def info(self) -> ModuleInfo:
        """返回模块元信息。"""
        pass

    @abstractmethod
    def check(self, config: dict[str, Any], root: Path) -> CheckResult:
        """检测环境。

        Args:
            config: 模块配置（来自 [env.模块名]）
            root: 项目根目录

        Returns:
            CheckResult: 检测结果
        """
        pass

    def ensure(self, config: dict[str, Any], root: Path) -> CheckResult:
        """修复环境（可选实现）。

        Args:
            config: 模块配置
            root: 项目根目录

        Returns:
            CheckResult: 修复结果
        """
        return CheckResult(
            success=False,
            message="此模块不支持自动修复",
        )

    def validate_config(self, config: dict[str, Any]) -> tuple[bool, str | None]:
        """验证模块配置是否完整。

        Args:
            config: 模块配置

        Returns:
            (是否有效, 错误信息)
        """
        if not self.info.requires_config:
            return True, None

        missing = [k for k in self.info.config_keys if k not in config]
        if missing:
            return False, f"缺少配置项: {', '.join(missing)}"
        return True, None
