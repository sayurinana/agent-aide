"""状态文件读写：锁、原子写入、归档。"""

from __future__ import annotations

import json
import os
import time
from contextlib import contextmanager
from pathlib import Path

from aide.flow.errors import FlowError
from aide.flow.types import FlowStatus
from aide.flow.utils import now_task_id


class FlowStorage:
    def __init__(self, root: Path):
        self.root = root
        self.aide_dir = self.root / ".aide"
        self.status_path = self.aide_dir / "flow-status.json"
        self.lock_path = self.aide_dir / "flow-status.lock"
        self.tmp_path = self.aide_dir / "flow-status.json.tmp"
        self.logs_dir = self.aide_dir / "logs"

    def ensure_ready(self) -> None:
        if not self.aide_dir.exists():
            raise FlowError("未找到 .aide 目录，请先运行：aide init")
        self.logs_dir.mkdir(parents=True, exist_ok=True)

    @contextmanager
    def lock(self, timeout_seconds: float = 3.0, poll_seconds: float = 0.2):
        self.ensure_ready()
        start = time.time()
        fd: int | None = None
        while True:
            try:
                fd = os.open(str(self.lock_path), os.O_CREAT | os.O_EXCL | os.O_WRONLY)
                os.write(fd, str(os.getpid()).encode("utf-8"))
                break
            except FileExistsError:
                if time.time() - start >= timeout_seconds:
                    raise FlowError("状态文件被占用，请稍后重试或删除 .aide/flow-status.lock")
                time.sleep(poll_seconds)
        try:
            yield
        finally:
            if fd is not None:
                try:
                    os.close(fd)
                except OSError:
                    pass
            try:
                self.lock_path.unlink(missing_ok=True)
            except Exception:
                pass

    def load_status(self) -> FlowStatus | None:
        if not self.status_path.exists():
            return None
        try:
            raw = self.status_path.read_text(encoding="utf-8")
            data = json.loads(raw)
            if not isinstance(data, dict):
                raise ValueError("状态文件顶层必须为对象")
            return FlowStatus.from_dict(data)
        except Exception as exc:
            raise FlowError(f"状态文件解析失败: {exc}")

    def save_status(self, status: FlowStatus) -> None:
        payload = json.dumps(status.to_dict(), ensure_ascii=False, indent=2) + "\n"
        try:
            self.tmp_path.write_text(payload, encoding="utf-8")
            os.replace(self.tmp_path, self.status_path)
        except Exception as exc:
            raise FlowError(f"写入状态文件失败: {exc}")

    def archive_existing_status(self) -> None:
        if not self.status_path.exists():
            return
        suffix = now_task_id()
        try:
            current = self.load_status()
            suffix = current.task_id
        except FlowError:
            pass
        target = self.logs_dir / f"flow-status.{suffix}.json"
        try:
            os.replace(self.status_path, target)
        except Exception as exc:
            raise FlowError(f"归档旧状态失败: {exc}")

