"""BranchManager：管理 aide flow 任务分支编号和概况文档。"""

from __future__ import annotations

import json
import shutil
from dataclasses import dataclass
from pathlib import Path
from typing import Any, TYPE_CHECKING

from aide.flow.errors import FlowError
from aide.flow.git import GitIntegration
from aide.flow.utils import now_iso

if TYPE_CHECKING:
    from aide.core.config import ConfigManager


@dataclass
class BranchInfo:
    """分支信息记录"""
    number: int
    branch_name: str
    source_branch: str
    start_commit: str
    end_commit: str | None
    task_id: str
    task_summary: str
    started_at: str
    finished_at: str | None
    status: str  # "active", "finished", "merged-to-temp"
    temp_branch: str | None = None

    def to_dict(self) -> dict[str, Any]:
        data: dict[str, Any] = {
            "number": self.number,
            "branch_name": self.branch_name,
            "source_branch": self.source_branch,
            "start_commit": self.start_commit,
            "task_id": self.task_id,
            "task_summary": self.task_summary,
            "started_at": self.started_at,
            "status": self.status,
        }
        if self.end_commit is not None:
            data["end_commit"] = self.end_commit
        if self.finished_at is not None:
            data["finished_at"] = self.finished_at
        if self.temp_branch is not None:
            data["temp_branch"] = self.temp_branch
        return data

    @staticmethod
    def from_dict(data: dict[str, Any]) -> "BranchInfo":
        return BranchInfo(
            number=data["number"],
            branch_name=data["branch_name"],
            source_branch=data["source_branch"],
            start_commit=data["start_commit"],
            end_commit=data.get("end_commit"),
            task_id=data["task_id"],
            task_summary=data["task_summary"],
            started_at=data["started_at"],
            finished_at=data.get("finished_at"),
            status=data.get("status", "active"),
            temp_branch=data.get("temp_branch"),
        )


@dataclass
class BranchesData:
    """分支概况数据"""
    next_number: int
    branches: list[BranchInfo]

    def to_dict(self) -> dict[str, Any]:
        return {
            "next_number": self.next_number,
            "branches": [b.to_dict() for b in self.branches],
        }

    @staticmethod
    def from_dict(data: dict[str, Any]) -> "BranchesData":
        next_number = data.get("next_number", 1)
        branches_raw = data.get("branches", [])
        branches = [BranchInfo.from_dict(b) for b in branches_raw]
        return BranchesData(next_number=next_number, branches=branches)


class BranchManager:
    """管理 aide flow 任务分支"""

    def __init__(self, root: Path, git: GitIntegration, cfg: "ConfigManager"):
        self.root = root
        self.git = git
        self.cfg = cfg
        self.aide_dir = root / ".aide"
        self.branches_json = self.aide_dir / "branches.json"
        self.branches_md = self.aide_dir / "branches.md"
        self.lock_path = self.aide_dir / "flow-status.lock"
        self.logs_dir = self.aide_dir / "logs"
        self._data: BranchesData | None = None
        self._current_branch_info: BranchInfo | None = None

    def _cleanup_lock_file(self) -> None:
        """清理 lock 文件，避免分支切换时的冲突"""
        try:
            if self.lock_path.exists():
                self.lock_path.unlink()
        except OSError:
            pass

    def _cleanup_task_files(self, task_id: str) -> None:
        """清理任务相关文件

        - 删除 .aide/*.lock
        - 删除任务细则文件 (task.spec)
        - 清空任务原文件 (task.source)，保留文件本身
        - 备份并删除 flow-status.json
        - 备份并删除 decisions/*.json
        - 删除 pending-items.json
        - 删除流程图目录下的所有文件 (.puml, .plantuml, .png)
        """
        # 确保 logs 目录存在
        self.logs_dir.mkdir(parents=True, exist_ok=True)

        # 1. 删除所有 .lock 文件
        for lock_file in self.aide_dir.glob("*.lock"):
            try:
                lock_file.unlink()
            except OSError:
                pass

        # 2. 删除任务细则文件
        task_spec = self.cfg.get_value("task.spec")
        if task_spec:
            spec_path = self.root / task_spec
            if spec_path.exists():
                try:
                    spec_path.unlink()
                except OSError:
                    pass

        # 3. 清空任务原文件（保留文件本身）
        task_source = self.cfg.get_value("task.source")
        if task_source:
            source_path = self.root / task_source
            if source_path.exists():
                try:
                    source_path.write_text("", encoding="utf-8")
                except OSError:
                    pass

        # 4. 备份并删除 flow-status.json
        status_path = self.aide_dir / "flow-status.json"
        if status_path.exists():
            backup_name = f"{task_id}-status.json"
            backup_path = self.logs_dir / backup_name
            try:
                shutil.copy2(status_path, backup_path)
                status_path.unlink()
            except OSError:
                pass

        # 5. 备份并删除 decisions/*.json
        decisions_dir = self.aide_dir / "decisions"
        if decisions_dir.exists():
            decision_files = list(decisions_dir.glob("*.json"))
            if decision_files:
                # 创建备份目录
                backup_decisions_dir = self.logs_dir / f"{task_id}-decisions"
                backup_decisions_dir.mkdir(parents=True, exist_ok=True)
                for decision_file in decision_files:
                    try:
                        shutil.copy2(decision_file, backup_decisions_dir / decision_file.name)
                        decision_file.unlink()
                    except OSError:
                        pass

        # 6. 删除 pending-items.json
        pending_items_path = self.aide_dir / "pending-items.json"
        if pending_items_path.exists():
            try:
                pending_items_path.unlink()
            except OSError:
                pass

        # 7. 删除流程图目录下的所有文件
        diagram_path = self.cfg.get_value("flow.diagram_path") or ".aide/diagrams"
        diagram_dir = self.root / diagram_path
        if diagram_dir.exists() and diagram_dir.is_dir():
            for diagram_file in diagram_dir.iterdir():
                if diagram_file.is_file() and diagram_file.suffix in (".puml", ".plantuml", ".png"):
                    try:
                        diagram_file.unlink()
                    except OSError:
                        pass

    def load_branches(self) -> BranchesData:
        """加载分支概况"""
        if self._data is not None:
            return self._data

        if not self.branches_json.exists():
            self._data = BranchesData(next_number=1, branches=[])
            return self._data

        try:
            content = self.branches_json.read_text(encoding="utf-8")
            data = json.loads(content)
            self._data = BranchesData.from_dict(data)
            return self._data
        except (json.JSONDecodeError, KeyError, TypeError) as e:
            raise FlowError(f"读取分支概况失败: {e}") from e

    def save_branches(self) -> None:
        """保存分支概况（同时生成 JSON 和 MD）"""
        if self._data is None:
            return

        self.aide_dir.mkdir(parents=True, exist_ok=True)

        # 保存 JSON
        json_content = json.dumps(
            self._data.to_dict(),
            ensure_ascii=False,
            indent=2,
        )
        self.branches_json.write_text(json_content + "\n", encoding="utf-8")

        # 生成并保存 MD
        md_content = self._generate_markdown()
        self.branches_md.write_text(md_content, encoding="utf-8")

    def _generate_markdown(self) -> str:
        """生成 Markdown 格式的分支概况"""
        if self._data is None:
            return "# Git 分支概况\n\n暂无分支记录。\n"

        lines = ["# Git 分支概况\n"]

        if not self._data.branches:
            lines.append("暂无分支记录。\n")
            return "\n".join(lines)

        for branch in reversed(self._data.branches):
            lines.append(f"## {branch.branch_name}\n")
            lines.append(f"- **任务**: {branch.task_summary}")
            lines.append(f"- **任务ID**: {branch.task_id}")
            lines.append(f"- **源分支**: {branch.source_branch}")
            lines.append(f"- **起始提交**: {branch.start_commit[:7]}")
            if branch.end_commit:
                lines.append(f"- **结束提交**: {branch.end_commit[:7]}")
            lines.append(f"- **状态**: {branch.status}")
            time_str = branch.started_at[:16].replace("T", " ")
            if branch.finished_at:
                time_str += f" ~ {branch.finished_at[11:16]}"
            lines.append(f"- **时间**: {time_str}")
            if branch.temp_branch:
                lines.append(f"- **临时分支**: {branch.temp_branch}")
            lines.append("")

        return "\n".join(lines)

    def get_next_branch_number(self) -> int:
        """获取下一个分支编号"""
        data = self.load_branches()
        return data.next_number

    def create_task_branch(
        self,
        task_id: str,
        task_summary: str,
    ) -> str:
        """创建任务分支并记录信息

        返回创建的分支名称
        """
        self.git.ensure_repo()
        data = self.load_branches()

        # 确保 git 状态干净
        if not self.git.is_clean():
            self.git.add_all()
            self.git.commit("[aide] 保存未提交的变更")

        # 确保有提交历史
        if not self.git.has_commits():
            gitkeep = self.root / ".gitkeep"
            if not gitkeep.exists():
                gitkeep.touch()
            self.git.add_all()
            self.git.commit("[aide] 初始提交")

        # 记录起始信息
        source_branch = self.git.get_current_branch()
        start_commit = self.git.rev_parse_head()

        # 创建分支名
        branch_number = data.next_number
        branch_name = f"aide/{branch_number:03d}"

        # 创建并切换到任务分支
        self.git.checkout_new_branch(branch_name)

        # 记录分支信息
        branch_info = BranchInfo(
            number=branch_number,
            branch_name=branch_name,
            source_branch=source_branch,
            start_commit=start_commit,
            end_commit=None,
            task_id=task_id,
            task_summary=task_summary,
            started_at=now_iso(),
            finished_at=None,
            status="active",
        )

        # 更新数据
        data.branches.append(branch_info)
        data.next_number = branch_number + 1
        self._data = data
        self._current_branch_info = branch_info

        # 保存
        self.save_branches()

        return branch_name

    def get_active_branch_info(self) -> BranchInfo | None:
        """获取当前活跃的分支信息"""
        if self._current_branch_info is not None:
            return self._current_branch_info

        data = self.load_branches()
        current_branch = self.git.get_current_branch()

        for branch in data.branches:
            if branch.branch_name == current_branch and branch.status == "active":
                self._current_branch_info = branch
                return branch

        return None

    def record_branch_finish(
        self,
        status: str = "finished",
        end_commit: str | None = None,
        temp_branch: str | None = None,
    ) -> None:
        """记录分支结束信息"""
        data = self.load_branches()
        branch_info = self.get_active_branch_info()

        if branch_info is None:
            return

        # 更新分支信息
        for i, branch in enumerate(data.branches):
            if branch.number == branch_info.number:
                data.branches[i] = BranchInfo(
                    number=branch.number,
                    branch_name=branch.branch_name,
                    source_branch=branch.source_branch,
                    start_commit=branch.start_commit,
                    end_commit=end_commit or self.git.rev_parse_head(),
                    task_id=branch.task_id,
                    task_summary=branch.task_summary,
                    started_at=branch.started_at,
                    finished_at=now_iso(),
                    status=status,
                    temp_branch=temp_branch,
                )
                break

        self._data = data
        self._current_branch_info = None
        self.save_branches()

    def finish_branch_merge(self, task_summary: str) -> tuple[bool, str]:
        """执行分支合并逻辑

        返回 (是否成功, 消息)
        """
        branch_info = self.get_active_branch_info()

        if branch_info is None:
            return True, "未找到活跃的任务分支，跳过合并"

        source_branch = branch_info.source_branch
        start_commit = branch_info.start_commit
        task_branch = branch_info.branch_name

        # 检查源分支是否有新提交
        if self.git.has_commits_since(start_commit, source_branch):
            # 源分支有新提交，使用临时分支策略
            return self._merge_with_temp_branch(
                branch_info=branch_info,
                task_summary=task_summary,
            )
        else:
            # 正常合并流程
            return self._merge_normal(
                branch_info=branch_info,
                task_summary=task_summary,
            )

    def _merge_normal(
        self,
        branch_info: BranchInfo,
        task_summary: str,
    ) -> tuple[bool, str]:
        """正常合并流程：清理 → 临时提交 → squash 合并 → 收尾提交"""
        source_branch = branch_info.source_branch
        task_branch = branch_info.branch_name
        task_id = branch_info.task_id
        start_commit = branch_info.start_commit

        # 1. 执行任务文件清理（在工作分支上）
        self._cleanup_task_files(task_id)

        # 2. 创建临时提交保存清理后的变更
        self.git.add_all()
        self.git.commit("[aide] 清理任务临时文件")

        # 3. 切回源分支
        self.git.checkout(source_branch)

        # 切换分支后清理 lock 文件
        self._cleanup_lock_file()

        # 4. squash 合并任务分支
        self.git.merge_squash(task_branch)

        # 5. 先更新分支记录（不再记录 end_commit）
        self.record_branch_finish(status="finished")

        # 6. 创建收尾提交（包含 squash 内容和分支记录更新）
        # 格式：{起始哈希}的任务收尾
        self.git.add_all()
        short_hash = start_commit[:7] if start_commit else "unknown"
        commit_msg = f"{short_hash}的任务收尾"
        self.git.commit(commit_msg)

        return True, f"任务分支已合并到 {source_branch}"

    def _merge_with_temp_branch(
        self,
        branch_info: BranchInfo,
        task_summary: str,
    ) -> tuple[bool, str]:
        """临时分支合并策略：源分支有新提交时使用"""
        start_commit = branch_info.start_commit
        task_branch = branch_info.branch_name
        temp_branch = f"{task_branch}-merge"

        # 从起始提交检出临时分支
        self.git.checkout_new_branch(temp_branch, start_commit)

        # 切换分支后清理 lock 文件
        self._cleanup_lock_file()

        # 在临时分支执行 squash 合并
        self.git.merge_squash(task_branch)

        # 创建提交
        self.git.add_all()
        commit_msg = f"[aide] 任务压缩提交: {task_summary}"
        end_commit = self.git.commit(commit_msg)

        # 记录完成（保留任务分支和临时分支）
        self.record_branch_finish(
            status="merged-to-temp",
            end_commit=end_commit,
            temp_branch=temp_branch,
        )

        return False, (
            f"⚠ 源分支 {branch_info.source_branch} 有新提交\n"
            f"已在临时分支 {temp_branch} 完成合并\n"
            f"请手动处理后续操作"
        )
