"""aide 命令行入口。"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path
from typing import Any

from aide.core import output
from aide.core.config import ConfigManager, find_project_root
from aide.env.manager import EnvManager
from aide.flow.tracker import FlowTracker


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

    # aide init
    init_parser = subparsers.add_parser("init", help="初始化 .aide 目录与默认配置")
    init_parser.set_defaults(func=handle_init)

    # aide env
    env_parser = subparsers.add_parser("env", help="环境管理")
    env_sub = env_parser.add_subparsers(dest="env_command")

    # aide env ensure
    ensure_parser = env_sub.add_parser("ensure", help="检测并修复运行环境")
    ensure_parser.add_argument(
        "--runtime",
        action="store_true",
        help="仅检查 aide 运行时环境（python + uv）",
    )
    ensure_parser.add_argument(
        "--modules",
        type=str,
        help="指定要检测的模块（逗号分隔）",
    )
    ensure_parser.add_argument(
        "--all",
        action="store_true",
        dest="check_all",
        help="检测所有已启用模块（仅检查不修复）",
    )
    ensure_parser.add_argument(
        "-v", "--verbose",
        action="store_true",
        help="显示详细配置信息",
    )
    ensure_parser.set_defaults(func=handle_env_ensure)

    # aide env list
    list_parser = env_sub.add_parser("list", help="列出所有可用模块")
    list_parser.set_defaults(func=handle_env_list)

    # aide env set
    set_parser = env_sub.add_parser("set", help="设置环境配置（带验证）")
    set_parser.add_argument("key", help="配置键：modules 或 模块名.配置项")
    set_parser.add_argument("value", help="配置值")
    set_parser.set_defaults(func=handle_env_set)

    # aide env（无子命令时等同于 ensure）
    env_parser.set_defaults(func=handle_env_default)

    # aide config
    config_parser = subparsers.add_parser("config", help="配置管理")
    config_sub = config_parser.add_subparsers(dest="config_command")

    get_parser = config_sub.add_parser("get", help="读取配置值")
    get_parser.add_argument("key", help="使用点号分隔的键名，如 task.source")
    get_parser.set_defaults(func=handle_config_get)

    set_parser = config_sub.add_parser("set", help="设置配置值")
    set_parser.add_argument("key", help="使用点号分隔的键名，如 task.source")
    set_parser.add_argument("value", help="要写入的值，支持 bool/int/float/字符串")
    set_parser.set_defaults(func=handle_config_set)

    # aide flow
    flow_parser = subparsers.add_parser("flow", help="进度追踪与 git 集成")
    flow_sub = flow_parser.add_subparsers(dest="flow_command")

    flow_start = flow_sub.add_parser("start", help="开始新任务")
    flow_start.add_argument("phase", help="环节名（来自 flow.phases）")
    flow_start.add_argument("summary", help="本次操作的简要说明")
    flow_start.set_defaults(func=handle_flow_start)

    flow_next_step = flow_sub.add_parser("next-step", help="记录步骤前进")
    flow_next_step.add_argument("summary", help="本次操作的简要说明")
    flow_next_step.set_defaults(func=handle_flow_next_step)

    flow_back_step = flow_sub.add_parser("back-step", help="记录步骤回退")
    flow_back_step.add_argument("reason", help="回退原因")
    flow_back_step.set_defaults(func=handle_flow_back_step)

    flow_next_part = flow_sub.add_parser("next-part", help="进入下一环节")
    flow_next_part.add_argument("phase", help="目标环节名（相邻下一环节）")
    flow_next_part.add_argument("summary", help="本次操作的简要说明")
    flow_next_part.set_defaults(func=handle_flow_next_part)

    flow_back_part = flow_sub.add_parser("back-part", help="回退到之前环节")
    flow_back_part.add_argument("phase", help="目标环节名（任意之前环节）")
    flow_back_part.add_argument("reason", help="回退原因")
    flow_back_part.set_defaults(func=handle_flow_back_part)

    flow_issue = flow_sub.add_parser("issue", help="记录一般问题（不阻塞继续）")
    flow_issue.add_argument("description", help="问题描述")
    flow_issue.set_defaults(func=handle_flow_issue)

    flow_error = flow_sub.add_parser("error", help="记录严重错误（需要用户关注）")
    flow_error.add_argument("description", help="错误描述")
    flow_error.set_defaults(func=handle_flow_error)

    # aide flow status
    flow_status = flow_sub.add_parser("status", help="查看当前任务状态")
    flow_status.set_defaults(func=handle_flow_status)

    # aide flow list
    flow_list = flow_sub.add_parser("list", help="列出所有任务")
    flow_list.set_defaults(func=handle_flow_list)

    # aide flow show <task_id>
    flow_show = flow_sub.add_parser("show", help="查看指定任务的详细状态")
    flow_show.add_argument("task_id", help="任务 ID（时间戳格式，如 20251215-103000）")
    flow_show.set_defaults(func=handle_flow_show)

    # aide flow clean
    flow_clean = flow_sub.add_parser("clean", help="强制清理当前任务（工作区需干净）")
    flow_clean.set_defaults(func=handle_flow_clean)

    flow_parser.set_defaults(func=handle_flow_help)

    # aide decide
    decide_parser = subparsers.add_parser("decide", help="待定项确认与决策记录")
    decide_subparsers = decide_parser.add_subparsers(dest="decide_cmd")

    # aide decide submit <file>
    decide_submit_parser = decide_subparsers.add_parser("submit", help="提交待定项数据并启动 Web 服务")
    decide_submit_parser.add_argument("file", help="待定项 JSON 数据文件路径")
    decide_submit_parser.set_defaults(func=handle_decide_submit)

    # aide decide result
    decide_result_parser = decide_subparsers.add_parser("result", help="获取用户决策结果")
    decide_result_parser.set_defaults(func=handle_decide_result)

    decide_parser.set_defaults(func=handle_decide_help)

    parser.add_argument("--version", action="version", version="aide dev")
    return parser


def handle_init(args: argparse.Namespace) -> bool:
    # 使用当前工作目录（原地初始化，类似 git init）
    root = Path.cwd()
    cfg = ConfigManager(root)
    cfg.ensure_config()
    cfg.ensure_gitignore()
    output.ok("初始化完成，.aide/ 与默认配置已准备就绪")
    return True


def handle_env_default(args: argparse.Namespace) -> bool:
    """aide env（无子命令）等同于 aide env ensure。"""
    if args.env_command is None:
        # 无子命令，执行默认的 ensure
        root = find_project_root()
        cfg = ConfigManager(root)
        manager = EnvManager(root, cfg)
        return manager.ensure()
    return True


def handle_env_ensure(args: argparse.Namespace) -> bool:
    """aide env ensure 处理。"""
    root = find_project_root()
    cfg = ConfigManager(root)
    manager = EnvManager(root, cfg)

    # 解析 --modules 参数
    modules = None
    if args.modules:
        modules = [m.strip() for m in args.modules.split(",") if m.strip()]

    return manager.ensure(
        runtime_only=args.runtime,
        modules=modules,
        check_only=args.check_all,
        verbose=args.verbose,
    )


def handle_env_list(args: argparse.Namespace) -> bool:
    """aide env list 处理。"""
    root = find_project_root()
    cfg = ConfigManager(root)
    manager = EnvManager(root, cfg)
    manager.list_modules()
    return True


def handle_env_set(args: argparse.Namespace) -> bool:
    """aide env set 处理。"""
    root = find_project_root()
    cfg = ConfigManager(root)
    manager = EnvManager(root, cfg)

    key = args.key
    value = args.value

    if key == "modules":
        # 设置启用的模块列表
        module_names = [m.strip() for m in value.split(",") if m.strip()]
        return manager.set_modules(module_names)
    elif "." in key:
        # 设置模块配置，如 venv.path
        parts = key.split(".", 1)
        module_name = parts[0]
        config_key = parts[1]
        parsed_value = _parse_value(value)
        return manager.set_module_config(module_name, config_key, parsed_value)
    else:
        # 无效的键格式
        output.err(f"无效的配置键: {key}")
        output.info("用法: aide env set modules <模块列表>")
        output.info("      aide env set <模块名>.<配置项> <值>")
        return False


def handle_config_get(args: argparse.Namespace) -> bool:
    root = find_project_root()
    cfg = ConfigManager(root)
    value = cfg.get_value(args.key)
    if value is None:
        output.warn(f"未找到配置项 {args.key}")
        return False
    output.info(f"{args.key} = {value!r}")
    return True


def handle_config_set(args: argparse.Namespace) -> bool:
    root = find_project_root()
    cfg = ConfigManager(root)
    parsed_value = _parse_value(args.value)
    cfg.set_value(args.key, parsed_value)
    return True


def handle_flow_help(args: argparse.Namespace) -> bool:
    output.info("用法: aide flow <start|next-step|back-step|next-part|back-part|issue|error> ...")
    return True


def handle_flow_start(args: argparse.Namespace) -> bool:
    root = find_project_root()
    cfg = ConfigManager(root)
    tracker = FlowTracker(root, cfg)
    return tracker.start(args.phase, args.summary)


def handle_flow_next_step(args: argparse.Namespace) -> bool:
    root = find_project_root()
    cfg = ConfigManager(root)
    tracker = FlowTracker(root, cfg)
    return tracker.next_step(args.summary)


def handle_flow_back_step(args: argparse.Namespace) -> bool:
    root = find_project_root()
    cfg = ConfigManager(root)
    tracker = FlowTracker(root, cfg)
    return tracker.back_step(args.reason)


def handle_flow_next_part(args: argparse.Namespace) -> bool:
    root = find_project_root()
    cfg = ConfigManager(root)
    tracker = FlowTracker(root, cfg)
    return tracker.next_part(args.phase, args.summary)


def handle_flow_back_part(args: argparse.Namespace) -> bool:
    root = find_project_root()
    cfg = ConfigManager(root)
    tracker = FlowTracker(root, cfg)
    return tracker.back_part(args.phase, args.reason)


def handle_flow_issue(args: argparse.Namespace) -> bool:
    root = find_project_root()
    cfg = ConfigManager(root)
    tracker = FlowTracker(root, cfg)
    return tracker.issue(args.description)


def handle_flow_error(args: argparse.Namespace) -> bool:
    root = find_project_root()
    cfg = ConfigManager(root)
    tracker = FlowTracker(root, cfg)
    return tracker.error(args.description)


def handle_flow_clean(args: argparse.Namespace) -> bool:
    """aide flow clean - 强制清理当前任务。"""
    root = find_project_root()
    cfg = ConfigManager(root)
    tracker = FlowTracker(root, cfg)
    return tracker.clean()


def handle_flow_status(args: argparse.Namespace) -> bool:
    """aide flow status - 查看当前任务状态。"""
    from aide.flow.storage import FlowStorage

    root = find_project_root()
    storage = FlowStorage(root)

    try:
        status = storage.load_status()
    except Exception as exc:
        output.err(f"读取状态失败: {exc}")
        return False

    if status is None:
        output.info("当前无活跃任务")
        return True

    # 获取最新的历史条目
    latest = status.history[-1] if status.history else None

    output.info(f"任务 ID: {status.task_id}")
    output.info(f"环节: {status.current_phase}")
    output.info(f"步骤: {status.current_step}")
    output.info(f"开始时间: {status.started_at}")
    if latest:
        output.info(f"最新操作: {latest.summary}")
        output.info(f"操作时间: {latest.timestamp}")
        if latest.git_commit:
            output.info(f"Git 提交: {latest.git_commit[:7]}")
    return True


def handle_flow_list(args: argparse.Namespace) -> bool:
    """aide flow list - 列出所有任务。"""
    from aide.flow.storage import FlowStorage

    root = find_project_root()
    storage = FlowStorage(root)

    try:
        tasks = storage.list_all_tasks()
    except Exception as exc:
        output.err(f"读取任务列表失败: {exc}")
        return False

    if not tasks:
        output.info("暂无任务记录")
        return True

    output.info("任务列表:")
    for i, task in enumerate(tasks, 1):
        marker = "*" if task["is_current"] else " "
        phase = task["phase"]
        summary = task["summary"][:30] + "..." if len(task["summary"]) > 30 else task["summary"]
        print(f"  {marker}[{i}] {task['task_id']} ({phase}) {summary}")

    output.info("提示: 使用 aide flow show <task_id> 查看详细状态")
    return True


def handle_flow_show(args: argparse.Namespace) -> bool:
    """aide flow show <task_id> - 查看指定任务的详细状态。"""
    from aide.flow.storage import FlowStorage

    root = find_project_root()
    storage = FlowStorage(root)

    try:
        status = storage.load_task_by_id(args.task_id)
    except Exception as exc:
        output.err(f"读取任务失败: {exc}")
        return False

    if status is None:
        output.err(f"未找到任务: {args.task_id}")
        return False

    output.info(f"任务 ID: {status.task_id}")
    output.info(f"当前环节: {status.current_phase}")
    output.info(f"当前步骤: {status.current_step}")
    output.info(f"开始时间: {status.started_at}")
    output.info("")
    output.info("历史记录:")

    for entry in status.history:
        commit_str = f" [{entry.git_commit[:7]}]" if entry.git_commit else ""
        print(f"  [{entry.phase}] {entry.summary}{commit_str}")
        print(f"         {entry.timestamp} ({entry.action})")

    return True


def handle_decide_help(args: argparse.Namespace) -> bool:
    print("usage: aide decide {submit,result} ...")
    print("")
    print("子命令:")
    print("  submit <file>  从文件读取待定项数据，启动后台 Web 服务")
    print("  result         获取用户决策结果")
    print("")
    print("示例:")
    print("  aide decide submit ./pending-items.json")
    print("  aide decide result")
    return True


def handle_decide_submit(args: argparse.Namespace) -> bool:
    from aide.decide import cmd_decide_submit
    return cmd_decide_submit(args.file)


def handle_decide_result(args: argparse.Namespace) -> bool:
    from aide.decide import cmd_decide_result
    return cmd_decide_result()


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
