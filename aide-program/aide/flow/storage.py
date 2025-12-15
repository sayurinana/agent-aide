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

    def list_all_tasks(self) -> list[dict]:
        """列出所有任务（当前 + 归档），返回按时间倒序排列的任务摘要列表。"""
        tasks = []

        # 当前任务
        current = self.load_status()
        if current is not None:
            tasks.append({
                "task_id": current.task_id,
                "phase": current.current_phase,
                "started_at": current.started_at,
                "summary": current.history[0].summary if current.history else "",
                "is_current": True,
            })

        # 归档任务
        if self.logs_dir.exists():
            for f in self.logs_dir.glob("flow-status.*.json"):
                try:
                    raw = f.read_text(encoding="utf-8")
                    data = json.loads(raw)
                    status = FlowStatus.from_dict(data)
                    tasks.append({
                        "task_id": status.task_id,
                        "phase": status.current_phase,
                        "started_at": status.started_at,
                        "summary": status.history[0].summary if status.history else "",
                        "is_current": False,
                    })
                except Exception:
                    continue

        # 按 task_id 倒序（task_id 是时间戳格式）
        tasks.sort(key=lambda x: x["task_id"], reverse=True)
        return tasks

    def load_task_by_id(self, task_id: str) -> FlowStatus | None:
        """根据 task_id 加载任务状态（当前或归档）。"""
        # 先检查当前任务
        current = self.load_status()
        if current is not None and current.task_id == task_id:
            return current

        # 检查归档
        archive_path = self.logs_dir / f"flow-status.{task_id}.json"
        if archive_path.exists():
            try:
                raw = archive_path.read_text(encoding="utf-8")
                data = json.loads(raw)
                return FlowStatus.from_dict(data)
            except Exception as exc:
                raise FlowError(f"读取归档任务失败: {exc}")

        return None

