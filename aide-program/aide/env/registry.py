"""模块注册表。"""

from __future__ import annotations

from aide.env.modules.base import BaseModule, ModuleInfo


class ModuleRegistry:
    """模块注册表，管理所有可用的环境检测模块。"""

    _modules: dict[str, BaseModule] = {}

    @classmethod
    def register(cls, module: BaseModule) -> None:
        """注册模块。"""
        cls._modules[module.info.name] = module

    @classmethod
    def get(cls, name: str) -> BaseModule | None:
        """获取指定模块。"""
        return cls._modules.get(name)

    @classmethod
    def all(cls) -> dict[str, BaseModule]:
        """获取所有已注册模块。"""
        return cls._modules.copy()

    @classmethod
    def names(cls) -> list[str]:
        """获取所有模块名称。"""
        return list(cls._modules.keys())

    @classmethod
    def list_info(cls) -> list[ModuleInfo]:
        """获取所有模块的元信息。"""
        return [m.info for m in cls._modules.values()]

    @classmethod
    def clear(cls) -> None:
        """清空注册表（用于测试）。"""
        cls._modules.clear()


def register_builtin_modules() -> None:
    """注册内置模块。"""
    from aide.env.modules import (
        python, uv, venv, requirements,
        rust, node, flutter,
        node_deps, android,
    )

    for mod in [python, uv, venv, requirements, rust, node, flutter, node_deps, android]:
        if hasattr(mod, "module"):
            ModuleRegistry.register(mod.module)
