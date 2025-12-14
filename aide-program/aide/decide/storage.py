"""决策数据的读写与原子化保存。"""

from __future__ import annotations

import json
import os
from datetime import datetime
from pathlib import Path
from typing import Any

from aide.decide.errors import DecideError
from aide.decide.types import DecideInput, DecideOutput, DecisionRecord, MetaInfo
from aide.flow.utils import now_task_id


class DecideStorage:
    """管理 pending.json 与历史记录文件。"""

    def __init__(self, root: Path):
        self.root = root
        self.aide_dir = self.root / ".aide"
        self.decisions_dir = self.aide_dir / "decisions"
        self.pending_path = self.decisions_dir / "pending.json"

    def ensure_ready(self) -> None:
        if not self.aide_dir.exists():
            raise DecideError(".aide 目录不存在，请先执行 aide init")
        self.decisions_dir.mkdir(parents=True, exist_ok=True)

    def save_pending(self, data: DecideInput) -> str:
        """保存待定项数据并生成 session_id。"""
        self.ensure_ready()
        session_id = now_task_id()
        created_at = datetime.now().astimezone().isoformat(timespec="seconds")
        meta = MetaInfo(created_at=created_at, session_id=session_id)
        payload = data.with_meta(meta)
        self._save_atomic(self.pending_path, payload.to_dict())
        return session_id

    def load_pending(self) -> DecideInput | None:
        """读取 pending.json，若不存在返回 None。"""
        self.ensure_ready()
        if not self.pending_path.exists():
            return None
        data = self._load_json(self.pending_path)
        return DecideInput.from_dict(data)

    def get_session_id(self) -> str | None:
        pending = self.load_pending()
        if pending is None:
            return None
        if pending.meta is None:
            raise DecideError("pending.json 缺少 _meta.session_id")
        return pending.meta.session_id

    def save_result(self, output: DecideOutput) -> None:
        """保存用户决策为历史记录。"""
        pending = self.load_pending()
        if pending is None:
            raise DecideError("未找到待定项数据，请先执行 aide decide submit '<json>'")
        if pending.meta is None:
            raise DecideError("pending.json 缺少 _meta.session_id")
        record = DecisionRecord(
            input=pending.without_meta(),
            output=output,
            completed_at=datetime.now().astimezone().isoformat(timespec="seconds"),
        )
        target = self.decisions_dir / f"{pending.meta.session_id}.json"
        self._save_atomic(target, record.to_dict())

    def load_result(self) -> DecideOutput | None:
        """读取当前会话的决策结果。"""
        session_id = self.get_session_id()
        if session_id is None:
            return None
        record_path = self.decisions_dir / f"{session_id}.json"
        if not record_path.exists():
            return None
        data = self._load_json(record_path)
        record = DecisionRecord.from_dict(data)
        return record.output

    def has_pending(self) -> bool:
        return self.pending_path.exists()

    def has_result(self) -> bool:
        session_id = None
        try:
            session_id = self.get_session_id()
        except DecideError:
            return False
        if not session_id:
            return False
        record_path = self.decisions_dir / f"{session_id}.json"
        return record_path.exists()

    def _save_atomic(self, path: Path, data: dict[str, Any]) -> None:
        payload = json.dumps(data, ensure_ascii=False, indent=2) + "\n"
        tmp_path = path.with_suffix(path.suffix + ".tmp")
        try:
            tmp_path.write_text(payload, encoding="utf-8")
            os.replace(tmp_path, path)
        except Exception as exc:
            raise DecideError(f"写入 {path.name} 失败: {exc}")

    def _load_json(self, path: Path) -> dict[str, Any]:
        try:
            raw = path.read_text(encoding="utf-8")
            data = json.loads(raw)
            if not isinstance(data, dict):
                raise DecideError(f"{path.name} 顶层必须为对象")
            return data
        except DecideError:
            raise
        except Exception as exc:
            raise DecideError(f"无法解析 {path.name}: {exc}")

