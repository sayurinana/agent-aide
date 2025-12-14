"""CLI 入口：解析参数并调度 decide 功能。"""

from __future__ import annotations

import json
import subprocess
import sys
import time
from pathlib import Path

from aide.core import output
from aide.decide.errors import DecideError
from aide.decide.storage import DecideStorage
from aide.decide.types import DecideInput


def cmd_decide_submit(file_path: str) -> bool:
    """从文件读取数据，启动后台 Web 服务。"""
    root = Path.cwd()
    storage = DecideStorage(root)

    # 1. 读取 JSON 文件
    json_file = Path(file_path)
    if not json_file.is_absolute():
        json_file = root / json_file

    if not json_file.exists():
        _print_error(f"文件不存在: {file_path}")
        return False

    try:
        raw = json.loads(json_file.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        _print_error(f"JSON 解析失败: {exc}", "检查 JSON 格式是否正确")
        return False

    # 2. 验证数据格式
    try:
        decide_input = DecideInput.from_dict(raw)
    except DecideError as exc:
        _print_error(f"数据验证失败: {exc}", "检查必填字段是否完整")
        return False

    # 3. 保存到 pending.json
    try:
        storage.save_pending(decide_input)
    except DecideError as exc:
        _print_error(str(exc))
        return False

    # 4. 启动后台服务
    return _start_daemon(root, storage)


def _start_daemon(root: Path, storage: DecideStorage) -> bool:
    """启动后台服务进程。"""
    # 启动 daemon 进程
    daemon_module = "aide.decide.daemon"
    try:
        subprocess.Popen(
            [sys.executable, "-m", daemon_module, str(root)],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            start_new_session=True,  # 脱离父进程
        )
    except Exception as exc:
        _print_error(f"启动后台服务失败: {exc}")
        return False

    # 等待服务启动（检查状态文件）
    for _ in range(50):  # 最多等待 5 秒
        time.sleep(0.1)
        info = storage.load_server_info()
        if info and "url" in info:
            output.info("Web 服务已启动")
            output.info(f"请访问: {info['url']}")
            output.info("用户完成决策后执行 aide decide result 获取结果")
            return True

    _print_error("服务启动超时", "请检查端口是否被占用")
    return False


def cmd_decide_result() -> bool:
    """获取决策结果（服务在用户提交后自动关闭）。"""
    root = Path.cwd()
    storage = DecideStorage(root)

    # 检查 pending
    try:
        pending = storage.load_pending()
    except DecideError as exc:
        _print_error(str(exc))
        return False

    if pending is None:
        _print_error("未找到待定项数据", "请先执行 aide decide submit <file>")
        return False

    session_id = pending.meta.session_id if pending.meta else None
    if not session_id:
        _print_error("数据异常", "pending.json 缺少 session_id，请重新执行 aide decide submit")
        return False

    # 检查结果
    try:
        result = storage.load_result()
    except DecideError as exc:
        _print_error(str(exc))
        return False

    if result is None:
        # 检查服务是否还在运行
        if storage.is_server_running():
            _print_error("尚无决策结果", "请等待用户在 Web 界面完成操作")
        else:
            # 服务已关闭但没有结果，可能是超时或异常
            _print_error("尚无决策结果", "服务可能已超时关闭，请重新执行 aide decide submit")
        return False

    # 输出结果
    payload = json.dumps(result.to_dict(), ensure_ascii=False, separators=(",", ":"))
    print(payload)

    # 清理服务状态文件（如存在）
    storage.clear_server_info()
    return True


def _print_error(message: str, suggestion: str | None = None) -> None:
    sys.stderr.write(f"✗ {message}\n")
    if suggestion:
        sys.stderr.write(f"  建议: {suggestion}\n")
