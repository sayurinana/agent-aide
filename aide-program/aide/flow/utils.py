"""工具函数：时间戳与文本处理。"""

from __future__ import annotations

from datetime import datetime


def now_iso() -> str:
    return datetime.now().astimezone().isoformat(timespec="seconds")


def now_task_id() -> str:
    return datetime.now().astimezone().strftime("%Y-%m-%dT%H-%M-%S")


def normalize_text(value: str) -> str:
    return value.strip()

