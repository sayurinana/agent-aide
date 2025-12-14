"""CLI 入口：解析参数并调度 decide 功能。"""

from __future__ import annotations

import json
import sys
from pathlib import Path

from aide.decide.errors import DecideError
from aide.decide.server import DecideServer
from aide.decide.storage import DecideStorage
from aide.decide.types import DecideInput


def cmd_decide(args) -> bool:
    """aide decide 统一入口。"""
    if getattr(args, "data", None) == "result":
        return cmd_decide_result()
    if getattr(args, "data", None) is None:
        _print_error("缺少参数: 需要传入 JSON 数据或 result")
        return False
    return cmd_decide_submit(args.data)


def cmd_decide_submit(json_data: str) -> bool:
    """提交待定项并启动 Web 服务。"""
    root = Path.cwd()
    storage = DecideStorage(root)

    try:
        raw = json.loads(json_data)
    except json.JSONDecodeError as exc:
        _print_error(f"JSON 解析失败: {exc}", "检查 JSON 格式是否正确")
        return False

    try:
        decide_input = DecideInput.from_dict(raw)
    except DecideError as exc:
        _print_error(f"数据验证失败: {exc}", "检查必填字段是否完整")
        return False

    try:
        storage.save_pending(decide_input)
    except DecideError as exc:
        _print_error(str(exc))
        return False

    server = DecideServer(root, storage)
    return server.start()


def cmd_decide_result() -> bool:
    """读取最新决策结果并输出 JSON。"""
    root = Path.cwd()
    storage = DecideStorage(root)

    try:
        pending = storage.load_pending()
    except DecideError as exc:
        _print_error(str(exc))
        return False

    if pending is None:
        _print_error("未找到待定项数据", "请先执行 aide decide submit '<json>'")
        return False

    session_id = pending.meta.session_id if pending.meta else None
    if not session_id:
        _print_error("决策结果已过期", "pending.json 已被更新，请重新执行 aide decide submit '<json>'")
        return False

    try:
        result = storage.load_result()
    except DecideError as exc:
        _print_error(str(exc))
        return False

    if result is None:
        has_history = any(
            path.is_file()
            and path.name.endswith(".json")
            and path.name != "pending.json"
            for path in storage.decisions_dir.glob("*.json")
        )
        if has_history:
            _print_error("决策结果已过期", "pending.json 已被更新，请重新执行 aide decide submit '<json>'")
        else:
            _print_error("尚无决策结果", "请等待用户在 Web 界面完成操作")
        return False

    payload = json.dumps(result.to_dict(), ensure_ascii=False, separators=(",", ":"))
    print(payload)
    return True


def _print_error(message: str, suggestion: str | None = None) -> None:
    sys.stderr.write(f"✗ {message}\n")
    if suggestion:
        sys.stderr.write(f"  建议: {suggestion}\n")
