"""数据结构：流程状态与历史条目。"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class HistoryEntry:
    timestamp: str
    action: str
    phase: str
    step: int
    summary: str
    git_commit: str | None = None

    def to_dict(self) -> dict[str, Any]:
        data: dict[str, Any] = {
            "timestamp": self.timestamp,
            "action": self.action,
            "phase": self.phase,
            "step": self.step,
            "summary": self.summary,
        }
        if self.git_commit is not None:
            data["git_commit"] = self.git_commit
        return data

    @staticmethod
    def from_dict(data: dict[str, Any]) -> "HistoryEntry":
        timestamp = _require_str(data, "timestamp")
        action = _require_str(data, "action")
        phase = _require_str(data, "phase")
        step = _require_int(data, "step")
        summary = _require_str(data, "summary")
        git_commit = data.get("git_commit")
        if git_commit is not None and not isinstance(git_commit, str):
            raise ValueError("git_commit 必须为字符串或缺失")
        return HistoryEntry(
            timestamp=timestamp,
            action=action,
            phase=phase,
            step=step,
            summary=summary,
            git_commit=git_commit,
        )


@dataclass(frozen=True)
class FlowStatus:
    task_id: str
    current_phase: str
    current_step: int
    started_at: str
    history: list[HistoryEntry]
    # 分支管理相关字段
    source_branch: str | None = None
    start_commit: str | None = None
    task_branch: str | None = None

    def to_dict(self) -> dict[str, Any]:
        data: dict[str, Any] = {
            "task_id": self.task_id,
            "current_phase": self.current_phase,
            "current_step": self.current_step,
            "started_at": self.started_at,
            "history": [h.to_dict() for h in self.history],
        }
        if self.source_branch is not None:
            data["source_branch"] = self.source_branch
        if self.start_commit is not None:
            data["start_commit"] = self.start_commit
        if self.task_branch is not None:
            data["task_branch"] = self.task_branch
        return data

    @staticmethod
    def from_dict(data: dict[str, Any]) -> "FlowStatus":
        task_id = _require_str(data, "task_id")
        current_phase = _require_str(data, "current_phase")
        current_step = _require_int(data, "current_step")
        started_at = _require_str(data, "started_at")
        raw_history = data.get("history")
        if not isinstance(raw_history, list):
            raise ValueError("history 必须为列表")
        history: list[HistoryEntry] = []
        for item in raw_history:
            if not isinstance(item, dict):
                raise ValueError("history 条目必须为对象")
            history.append(HistoryEntry.from_dict(item))
        return FlowStatus(
            task_id=task_id,
            current_phase=current_phase,
            current_step=current_step,
            started_at=started_at,
            history=history,
            source_branch=data.get("source_branch"),
            start_commit=data.get("start_commit"),
            task_branch=data.get("task_branch"),
        )


def _require_str(data: dict[str, Any], key: str) -> str:
    value = data.get(key)
    if not isinstance(value, str) or not value.strip():
        raise ValueError(f"{key} 必须为非空字符串")
    return value


def _require_int(data: dict[str, Any], key: str) -> int:
    value = data.get(key)
    if isinstance(value, bool) or not isinstance(value, int):
        raise ValueError(f"{key} 必须为整数")
    return value

