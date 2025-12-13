"""环境管理器。"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from aide.core import output
from aide.core.config import ConfigManager
from aide.env.registry import ModuleRegistry, register_builtin_modules

# 运行时模块（--runtime 时使用）
RUNTIME_MODULES = ["python", "uv"]

# 默认启用的模块
DEFAULT_MODULES = ["python", "uv", "venv", "requirements"]


def parse_module_name(name: str) -> tuple[str, str | None]:
    """解析模块名称，支持实例化命名。

    Args:
        name: 模块名称，如 "node_deps" 或 "node_deps:react"

    Returns:
        (模块类型, 实例名) - 实例名可能为 None
    """
    if ":" in name:
        parts = name.split(":", 1)
        return parts[0], parts[1]
    return name, None


def validate_modules(module_names: list[str]) -> tuple[bool, list[str]]:
    """验证模块名称是否有效。

    Args:
        module_names: 要验证的模块名称列表

    Returns:
        (是否全部有效, 无效的模块类型列表)
    """
    register_builtin_modules()
    available = set(ModuleRegistry.names())
    invalid = []
    for name in module_names:
        module_type, _ = parse_module_name(name)
        if module_type not in available:
            invalid.append(module_type)
    return len(invalid) == 0, invalid


class EnvManager:
    """环境管理器。"""

    def __init__(self, root: Path, cfg: ConfigManager):
        self.root = root
        self.cfg = cfg
        self.verbose = False
        # 确保模块已注册
        register_builtin_modules()

    def list_modules(self) -> None:
        """列出所有可用模块（aide env list）。"""
        config = self.cfg.load_config()
        enabled = self._get_enabled_modules(config)

        print("可用模块:")
        print(f"  {'模块':<14} {'描述':<20} {'能力':<16} {'需要配置'}")
        print("  " + "─" * 60)

        for info in ModuleRegistry.list_info():
            caps = ", ".join(info.capabilities)
            req_cfg = "是" if info.requires_config else "否"
            if info.config_keys:
                req_cfg += f" [{', '.join(info.config_keys)}]"
            print(f"  {info.name:<14} {info.description:<20} {caps:<16} {req_cfg}")

        print()
        if enabled:
            print(f"当前启用: {', '.join(enabled)}")
        else:
            output.warn("未配置启用模块列表")

    def ensure(
        self,
        runtime_only: bool = False,
        modules: list[str] | None = None,
        check_only: bool = False,
        verbose: bool = False,
    ) -> bool:
        """检测并修复环境。

        Args:
            runtime_only: 仅检测运行时环境
            modules: 指定要检测的模块
            check_only: 仅检测不修复（--all 模式）
            verbose: 显示详细配置信息

        Returns:
            是否全部成功
        """
        self.verbose = verbose
        config = self.cfg.load_config()
        enabled_modules = self._get_enabled_modules(config)

        # verbose: 输出基础信息
        if verbose:
            self._print_verbose_header(config, enabled_modules)

        # 确定要检测的模块列表
        if runtime_only:
            target_modules = RUNTIME_MODULES
        elif modules:
            target_modules = modules
        elif check_only:
            # --all 模式
            if not enabled_modules:
                output.warn("未配置启用模块列表，将检测所有支持的模块")
                target_modules = ModuleRegistry.names()
            else:
                target_modules = enabled_modules
        else:
            target_modules = enabled_modules

        if not target_modules:
            output.warn("没有要检测的模块")
            return True

        if verbose:
            print(f"  目标模块: {', '.join(target_modules)}")
            print()

        # 执行检测
        all_success = True
        results: list[tuple[str, bool, str]] = []

        for name in target_modules:
            is_enabled = name in enabled_modules
            success, msg = self._process_module(
                name=name,
                config=config,
                is_enabled=is_enabled,
                check_only=check_only,
            )
            results.append((name, success, msg))
            if not success and is_enabled:
                all_success = False
                break  # 启用模块失败时停止

        # 输出最终状态
        if all_success and not check_only:
            # 构建摘要信息
            summary_parts = []
            for name, success, msg in results:
                if success and msg:
                    summary_parts.append(f"{name}:{msg}")
            if summary_parts:
                output.ok(f"环境就绪 ({', '.join(summary_parts)})")

        return all_success

    def _print_verbose_header(self, config: dict[str, Any], enabled_modules: list[str]) -> None:
        """输出详细模式的头部信息。"""
        print("=" * 60)
        print("环境检测详细信息")
        print("=" * 60)
        print()
        print(f"  工作目录: {self.root}")
        print(f"  配置文件: {self.cfg.config_path}")
        print(f"  配置存在: {'是' if self.cfg.config_path.exists() else '否'}")
        print()
        print(f"  启用模块: {', '.join(enabled_modules) if enabled_modules else '(未配置)'}")
        print()

    def _print_verbose_module(self, name: str, module_config: dict[str, Any]) -> None:
        """输出模块的详细配置信息。"""
        print(f"  [{name}] 配置:")
        if not module_config:
            print("    (无配置)")
        else:
            for key, value in module_config.items():
                if key.startswith("_"):
                    continue  # 跳过内部字段
                if key == "path":
                    # 对于路径，显示绝对路径
                    abs_path = self.root / value
                    print(f"    {key}: {value}")
                    print(f"    {key} (绝对): {abs_path}")
                    print(f"    {key} (存在): {'是' if abs_path.exists() else '否'}")
                else:
                    print(f"    {key}: {value}")

    def _get_enabled_modules(self, config: dict[str, Any]) -> list[str]:
        """获取已启用的模块列表。"""
        env_config = config.get("env", {})
        return env_config.get("modules", DEFAULT_MODULES)

    def _get_module_config(self, name: str, config: dict[str, Any]) -> dict[str, Any]:
        """获取模块配置。

        支持实例化命名，如 node_deps:react 会查找 [env."node_deps:react"]
        """
        env_config = config.get("env", {})

        # 支持实例化命名：先尝试完整名称（如 node_deps:react）
        module_config = env_config.get(name, {})

        # 如果没找到且是实例化命名，尝试不带引号的格式
        if not module_config and ":" in name:
            # TOML 中可能存储为 env."node_deps:react" 或嵌套格式
            pass  # 已经在上面尝试过了

        # 兼容旧格式：如果值是字符串而不是字典，转换为 {"path": value}
        if isinstance(module_config, str):
            module_config = {"path": module_config}

        # 兼容旧格式：如果没有配置但存在旧格式字段
        if name == "venv" and not module_config:
            if "venv" in env_config and isinstance(env_config["venv"], str):
                module_config = {"path": env_config["venv"]}
        elif name == "requirements" and not module_config:
            if "requirements" in env_config and isinstance(env_config["requirements"], str):
                module_config = {"path": env_config["requirements"]}

        # 为 requirements 模块注入 venv 路径
        if name == "requirements":
            venv_config = self._get_module_config("venv", config)
            if "path" in venv_config:
                module_config["_venv_path"] = venv_config["path"]

        # 从 runtime 配置获取 python 版本要求
        if name == "python" and "min_version" not in module_config:
            runtime = config.get("runtime", {})
            if "python_min" in runtime:
                module_config["min_version"] = runtime["python_min"]

        return module_config

    def _process_module(
        self,
        name: str,
        config: dict[str, Any],
        is_enabled: bool,
        check_only: bool,
    ) -> tuple[bool, str]:
        """处理单个模块的检测/修复。

        支持实例化命名，如 node_deps:react

        Returns:
            (是否成功, 版本/路径信息)
        """
        # 解析模块名称，支持实例化命名
        module_type, instance_name = parse_module_name(name)

        module = ModuleRegistry.get(module_type)
        if not module:
            if is_enabled:
                output.err(f"{name}: 未知模块")
                return False, ""
            else:
                output.warn(f"{name}: 未知模块")
                return True, ""

        # 获取配置时使用完整名称（包含实例名）
        module_config = self._get_module_config(name, config)

        # verbose: 输出模块配置
        if self.verbose:
            self._print_verbose_module(name, module_config)

        # 检查类型B模块的配置
        valid, err_msg = module.validate_config(module_config)
        if not valid:
            if is_enabled:
                output.err(f"{name}: 已启用但{err_msg}")
                return False, ""
            else:
                output.warn(f"{name}: {err_msg}，跳过检测")
                return True, ""

        # 执行检测
        result = module.check(module_config, self.root)

        if result.success:
            version_info = result.version or ""
            extra = f" ({result.message})" if result.message else ""
            output.ok(f"{name}: {version_info}{extra}")
            return True, version_info

        # 检测失败
        if check_only:
            # --all 模式：仅报告
            output.warn(f"{name}: {result.message}")
            return True, ""

        if result.can_ensure and module.info.can_ensure:
            # 尝试修复
            output.info(f"{name}: {result.message}，尝试修复...")
            ensure_result = module.ensure(module_config, self.root)

            if ensure_result.success:
                msg = ensure_result.message or "已修复"
                output.ok(f"{name}: {msg}")
                return True, ensure_result.version or ""
            else:
                if is_enabled:
                    output.err(f"{name}: {ensure_result.message}")
                    return False, ""
                else:
                    output.warn(f"{name}: {ensure_result.message}")
                    return True, ""
        else:
            # 不可修复
            if is_enabled:
                extra = " (此模块不支持自动修复)" if not module.info.can_ensure else ""
                output.err(f"{name}: {result.message}{extra}")
                return False, ""
            else:
                output.warn(f"{name}: {result.message}")
                return True, ""

    def set_modules(self, module_names: list[str]) -> bool:
        """设置启用的模块列表（带验证）。

        Args:
            module_names: 要启用的模块名称列表

        Returns:
            是否设置成功
        """
        # 验证模块名称
        valid, invalid = validate_modules(module_names)
        if not valid:
            available = ModuleRegistry.names()
            output.err(f"未知模块: {', '.join(invalid)}")
            output.info(f"可用模块: {', '.join(available)}")
            return False

        # 设置配置
        self.cfg.set_value("env.modules", module_names)
        return True

    def set_module_config(self, module_name: str, key: str, value: Any) -> bool:
        """设置模块配置（带验证）。

        支持实例化命名，如 node_deps:react.path

        Args:
            module_name: 模块名称（可包含实例名，如 node_deps:react）
            key: 配置键
            value: 配置值

        Returns:
            是否设置成功
        """
        # 解析模块名称，支持实例化命名
        module_type, _ = parse_module_name(module_name)

        # 验证模块类型是否存在
        module = ModuleRegistry.get(module_type)
        if not module:
            available = ModuleRegistry.names()
            output.err(f"未知模块: {module_type}")
            output.info(f"可用模块: {', '.join(available)}")
            return False

        # 设置配置，使用完整模块名（包含实例名）
        # 注意：TOML 中带冒号的键需要引号，但 set_value 会自动处理
        config_key = f"env.{module_name}.{key}"
        self.cfg.set_value(config_key, value)
        return True
