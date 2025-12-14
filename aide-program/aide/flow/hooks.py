"""环节钩子：PlantUML 与 CHANGELOG 校验。"""

from __future__ import annotations

import shutil
import subprocess
from pathlib import Path

from aide.core import output
from aide.flow.errors import FlowError
from aide.flow.git import GitIntegration
from aide.flow.types import FlowStatus


def run_pre_commit_hooks(
    *,
    root: Path,
    git: GitIntegration,
    status: FlowStatus | None,
    from_phase: str | None,
    to_phase: str,
    action: str,
) -> None:
    if from_phase == "flow-design" and action in {"next-part", "back-part"}:
        _hook_plantuml(root=root)
    if from_phase == "docs" and action in {"next-part", "back-part"}:
        _hook_changelog_on_leave_docs(root=root, git=git, status=status)


def run_post_commit_hooks(*, to_phase: str, action: str) -> None:
    if to_phase == "docs" and action in {"start", "next-part", "back-part"}:
        output.info("请更新 CHANGELOG.md")


def _hook_plantuml(*, root: Path) -> None:
    docs_dir = root / "docs"
    discuss_dir = root / "discuss"
    candidates: list[Path] = []
    for base in (docs_dir, discuss_dir):
        if not base.exists():
            continue
        candidates.extend([p for p in base.rglob("*.puml") if p.is_file()])
        candidates.extend([p for p in base.rglob("*.plantuml") if p.is_file()])

    if not candidates:
        return

    if shutil.which("plantuml") is None:
        output.warn("未找到 plantuml，已跳过 PlantUML 校验/PNG 生成")
        return

    for file_path in candidates:
        result = subprocess.run(
            ["plantuml", "-tpng", str(file_path)],
            cwd=root,
            text=True,
            capture_output=True,
        )
        if result.returncode != 0:
            detail = (result.stderr or "").strip() or (result.stdout or "").strip()
            raise FlowError(f"PlantUML 处理失败: {file_path} {detail}".strip())


def _hook_changelog_on_leave_docs(*, root: Path, git: GitIntegration, status: FlowStatus | None) -> None:
    changelog = root / "CHANGELOG.md"
    if not changelog.exists():
        raise FlowError("离开 docs 前需要更新 CHANGELOG.md（当前文件不存在）")

    git.ensure_repo()
    if git.status_porcelain("CHANGELOG.md").strip():
        return

    if status is None:
        raise FlowError("离开 docs 前需要更新 CHANGELOG.md（未找到流程状态）")

    for entry in status.history:
        if entry.phase != "docs":
            continue
        if not entry.git_commit:
            continue
        if git.commit_touches_path(entry.git_commit, "CHANGELOG.md"):
            return

    raise FlowError("离开 docs 前需要更新 CHANGELOG.md（未检测到 docs 阶段的更新记录）")

