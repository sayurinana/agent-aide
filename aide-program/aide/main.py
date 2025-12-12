"""aide 命令行入口。"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path
from typing import Any

from aide.core import output
from aide.core.config import ConfigManager
from aide.env.ensure import EnvManager


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    if not hasattr(args, "func"):
        parser.print_help()
        return 0
    try:
        result = args.func(args)
    except KeyboardInterrupt:
        output.err("操作已取消")
        return 1
    if result is False:
        return 1
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="aide", description="Aide 工作流辅助工具")
    subparsers = parser.add_subparsers(dest="command")

    init_parser = subparsers.add_parser("init", help="初始化 .aide 目录与默认配置")
    init_parser.set_defaults(func=handle_init)

    env_parser = subparsers.add_parser("env", help="环境管理")
    env_sub = env_parser.add_subparsers(dest="env_command")
    ensure_parser = env_sub.add_parser("ensure", help="检测并修复运行环境")
    ensure_parser.add_argument("--runtime", action="store_true", help="仅检查 aide 运行时环境")
    ensure_parser.set_defaults(func=handle_env_ensure)

    config_parser = subparsers.add_parser("config", help="配置管理")
    config_sub = config_parser.add_subparsers(dest="config_command")
    get_parser = config_sub.add_parser("get", help="读取配置值")
    get_parser.add_argument("key", help="使用点号分隔的键名，如 task.source")
    get_parser.set_defaults(func=handle_config_get)

    set_parser = config_sub.add_parser("set", help="设置配置值")
    set_parser.add_argument("key", help="使用点号分隔的键名，如 task.source")
    set_parser.add_argument("value", help="要写入的值，支持 bool/int/float/字符串")
    set_parser.set_defaults(func=handle_config_set)

    parser.add_argument("--version", action="version", version="aide dev")
    return parser


def handle_init(args: argparse.Namespace) -> bool:
    root = Path.cwd()
    cfg = ConfigManager(root)
    cfg.ensure_config()
    cfg.ensure_gitignore()
    output.ok("初始化完成，.aide/ 与默认配置已准备就绪")
    return True


def handle_env_ensure(args: argparse.Namespace) -> bool:
    if args.env_command != "ensure":
        output.err("请指定 env 子命令，如: aide env ensure")
        return False
    root = Path.cwd()
    cfg = ConfigManager(root)
    manager = EnvManager(root)
    return manager.ensure(runtime_only=args.runtime, cfg=cfg)


def handle_config_get(args: argparse.Namespace) -> bool:
    root = Path.cwd()
    cfg = ConfigManager(root)
    value = cfg.get_value(args.key)
    if value is None:
        output.warn(f"未找到配置项 {args.key}")
        return False
    output.info(f"{args.key} = {value!r}")
    return True


def handle_config_set(args: argparse.Namespace) -> bool:
    root = Path.cwd()
    cfg = ConfigManager(root)
    parsed_value = _parse_value(args.value)
    cfg.set_value(args.key, parsed_value)
    return True


def _parse_value(raw: str) -> Any:
    lowered = raw.lower()
    if lowered in {"true", "false"}:
        return lowered == "true"
    try:
        if "." in raw:
            return float(raw)
        return int(raw)
    except ValueError:
        return raw


if __name__ == "__main__":
    sys.exit(main())
