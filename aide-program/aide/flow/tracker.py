"""FlowTracker：编排一次 flow 动作（校验 → hooks → 落盘 → git → 输出）。"""

from __future__ import annotations

from pathlib import Path

from aide.core import output
from aide.core.config import ConfigManager
from aide.flow.branch import BranchManager
from aide.flow.errors import FlowError
from aide.flow.git import GitIntegration
from aide.flow.hooks import run_post_commit_hooks, run_pre_commit_hooks
from aide.flow.storage import FlowStorage
from aide.flow.types import FlowStatus, HistoryEntry
from aide.flow.utils import now_iso, now_task_id, normalize_text
from aide.flow.validator import FlowValidator

DEFAULT_PHASES = ["task-optimize", "flow-design", "impl", "verify", "docs", "finish"]


class FlowTracker:
    def __init__(self, root: Path, cfg: ConfigManager):
        self.root = root
        self.cfg = cfg
        self.storage = FlowStorage(root)
        self.git = GitIntegration(root)
        self.branch_mgr = BranchManager(root, self.git, cfg)

    def start(self, phase: str, summary: str) -> bool:
        return self._run(action="start", to_phase=phase, text=summary)

    def next_step(self, summary: str) -> bool:
        return self._run(action="next-step", to_phase=None, text=summary)

    def back_step(self, reason: str) -> bool:
        return self._run(action="back-step", to_phase=None, text=reason)

    def next_part(self, phase: str, summary: str) -> bool:
        return self._run(action="next-part", to_phase=phase, text=summary)

    def back_part(self, phase: str, reason: str) -> bool:
        return self._run(action="back-part", to_phase=phase, text=reason)

    def issue(self, description: str) -> bool:
        return self._run(action="issue", to_phase=None, text=description)

    def error(self, description: str) -> bool:
        return self._run(action="error", to_phase=None, text=description)

    def _run(self, *, action: str, to_phase: str | None, text: str) -> bool:
        try:
            self.storage.ensure_ready()
            config = self.cfg.load_config()
            phases = _get_phases(config)
            validator = FlowValidator(phases)

            normalized_text = normalize_text(text)
            if not normalized_text:
                raise FlowError("文本参数不能为空")

            with self.storage.lock():
                if action == "start":
                    assert to_phase is not None
                    validator.validate_start(to_phase)
                    self.storage.archive_existing_status()

                    # 创建任务分支
                    task_id = now_task_id()
                    task_branch = self.branch_mgr.create_task_branch(
                        task_id=task_id,
                        task_summary=normalized_text,
                    )
                    branch_info = self.branch_mgr.get_active_branch_info()

                    status = FlowStatus(
                        task_id=task_id,
                        current_phase=to_phase,
                        current_step=0,
                        started_at=now_iso(),
                        history=[],
                        source_branch=branch_info.source_branch if branch_info else None,
                        start_commit=branch_info.start_commit if branch_info else None,
                        task_branch=task_branch,
                    )
                    updated, commit_msg = self._apply_action(
                        status=status,
                        action=action,
                        from_phase=None,
                        to_phase=to_phase,
                        text=normalized_text,
                        validator=validator,
                    )
                    # 先保存状态，再执行 git 操作
                    self.storage.save_status(updated)
                    final_status = self._do_git_commit(updated, commit_msg)
                    self.storage.save_status(final_status)
                    output.ok(f"任务开始: {to_phase} (分支: {task_branch})")
                    run_post_commit_hooks(to_phase=to_phase, action=action)
                    return True

                status = self.storage.load_status()
                if status is None:
                    raise FlowError("未找到流程状态，请先运行：aide flow start <环节名> \"<总结>\"")

                current_phase = status.current_phase
                validator.validate_phase_exists(current_phase)

                if action == "next-part":
                    assert to_phase is not None
                    validator.validate_next_part(current_phase, to_phase)
                elif action == "back-part":
                    assert to_phase is not None
                    validator.validate_back_part(current_phase, to_phase)
                else:
                    to_phase = current_phase

                updated, commit_msg = self._apply_action(
                    status=status,
                    action=action,
                    from_phase=current_phase,
                    to_phase=to_phase,
                    text=normalized_text,
                    validator=validator,
                )
                # 先保存状态，再执行 git 操作
                self.storage.save_status(updated)
                final_status = self._do_git_commit(updated, commit_msg)
                self.storage.save_status(final_status)

                # 如果进入 finish 环节，执行分支合并（必须在提交后执行）
                if action == "next-part" and to_phase == "finish":
                    # 再次提交状态文件（因为 save_status 更新了 git_commit hash）
                    self.git.add_all()
                    self.git.commit("[aide] finish: 更新状态文件")

                    success, merge_msg = self.branch_mgr.finish_branch_merge(
                        task_summary=normalized_text,
                    )
                    if not success:
                        output.warn(merge_msg)

                if action == "next-part":
                    output.ok(f"进入环节: {to_phase}")
                elif action == "back-part":
                    output.warn(f"回退到环节: {to_phase}")
                elif action == "error":
                    output.err(f"错误已记录: {normalized_text}")

                run_post_commit_hooks(to_phase=to_phase, action=action)
                return True
        except FlowError as exc:
            output.err(str(exc))
            return False

    def _apply_action(
        self,
        *,
        status: FlowStatus,
        action: str,
        from_phase: str | None,
        to_phase: str,
        text: str,
        validator: FlowValidator,
    ) -> tuple[FlowStatus, str]:
        """应用动作，返回 (更新后的状态, commit消息)。不执行 git 操作。"""
        if action in {"next-part", "back-part"} and from_phase is None:
            raise FlowError("内部错误：缺少 from_phase")

        if action == "next-part":
            assert from_phase is not None
            validator.validate_next_part(from_phase, to_phase)
        elif action == "back-part":
            assert from_phase is not None
            validator.validate_back_part(from_phase, to_phase)
        elif action == "start":
            validator.validate_start(to_phase)
        else:
            validator.validate_phase_exists(to_phase)

        config = self.cfg.load_config()
        run_pre_commit_hooks(
            root=self.root,
            git=self.git,
            status=status,
            from_phase=from_phase,
            to_phase=to_phase,
            action=action,
            config=config,
        )

        message = _build_commit_message(action=action, phase=to_phase, text=text)

        next_step = status.current_step + 1
        entry = HistoryEntry(
            timestamp=now_iso(),
            action=action,
            phase=to_phase,
            step=next_step,
            summary=text,
            git_commit=None,  # 暂时为 None，后续在 git 提交后更新
        )

        history = [*status.history, entry]
        updated_status = FlowStatus(
            task_id=status.task_id,
            current_phase=to_phase,
            current_step=next_step,
            started_at=status.started_at,
            history=history,
            source_branch=status.source_branch,
            start_commit=status.start_commit,
            task_branch=status.task_branch,
        )
        return updated_status, message

    def _do_git_commit(self, status: FlowStatus, message: str) -> FlowStatus:
        """执行 git add + commit，并更新状态中的 commit hash。"""
        self.git.add_all()
        commit_hash = self.git.commit(message)

        # 更新最后一条历史记录的 git_commit
        if status.history:
            last_entry = status.history[-1]
            updated_entry = HistoryEntry(
                timestamp=last_entry.timestamp,
                action=last_entry.action,
                phase=last_entry.phase,
                step=last_entry.step,
                summary=last_entry.summary,
                git_commit=commit_hash,
            )
            updated_history = [*status.history[:-1], updated_entry]
            return FlowStatus(
                task_id=status.task_id,
                current_phase=status.current_phase,
                current_step=status.current_step,
                started_at=status.started_at,
                history=updated_history,
                source_branch=status.source_branch,
                start_commit=status.start_commit,
                task_branch=status.task_branch,
            )
        return status


def _get_phases(config: dict) -> list[str]:
    flow_cfg = config.get("flow", {})
    phases = flow_cfg.get("phases", DEFAULT_PHASES)
    if not isinstance(phases, list) or not phases:
        return DEFAULT_PHASES
    return phases


def _build_commit_message(*, action: str, phase: str, text: str) -> str:
    if action == "issue":
        return f"[aide] {phase} issue: {text}"
    if action == "error":
        return f"[aide] {phase} error: {text}"
    if action == "back-step":
        return f"[aide] {phase} back-step: {text}"
    if action == "back-part":
        return f"[aide] {phase} back-part: {text}"
    return f"[aide] {phase}: {text}"

