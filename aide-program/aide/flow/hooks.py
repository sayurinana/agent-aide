"""环节钩子：PlantUML 与 CHANGELOG 校验。"""

from __future__ import annotations

import shutil
import subprocess
from pathlib import Path
from typing import Any

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
    config: dict[str, Any] | None = None,
) -> None:
    if from_phase == "flow-design" and action in {"next-part", "back-part"}:
        _hook_plantuml(root=root, config=config)
    if from_phase == "docs" and action in {"next-part", "back-part"}:
        _hook_changelog_on_leave_docs(root=root, git=git, status=status)
    if to_phase == "finish" and action == "next-part":
        _hook_clean_task_plans(root=root, config=config)


def run_post_commit_hooks(*, to_phase: str, action: str) -> None:
    if to_phase == "docs" and action in {"start", "next-part", "back-part"}:
        output.info("请更新 CHANGELOG.md")


def _get_plantuml_command(config: dict[str, Any] | None) -> list[str] | None:
    """获取 PlantUML 命令，优先使用配置的 jar 文件。"""
    if config:
        jar_path = config.get("plantuml", {}).get("jar_path")
        java_path = config.get("plantuml", {}).get("java_path", "java")

        if jar_path:
            # 尝试解析 jar 路径
            jar_file = Path(jar_path)
            if not jar_file.is_absolute():
                # 相对路径，相对于 aide-program 目录
                aide_program_dir = Path(__file__).parent.parent.parent
                jar_file = aide_program_dir / jar_path

            if jar_file.exists():
                return [java_path, "-jar", str(jar_file)]

    # 回退到系统 plantuml 命令
    if shutil.which("plantuml"):
        return ["plantuml"]

    return None


def _hook_plantuml(*, root: Path, config: dict[str, Any] | None = None) -> None:
    """PlantUML 校验和构建钩子。"""
    # 获取流程图目录
    diagram_path = ".aide/diagrams"
    if config:
        diagram_path = config.get("flow", {}).get("diagram_path", diagram_path)

    diagram_dir = root / diagram_path

    # 收集所有 .puml 文件
    candidates: list[Path] = []

    # 从配置的流程图目录
    if diagram_dir.exists():
        candidates.extend([p for p in diagram_dir.rglob("*.puml") if p.is_file()])
        candidates.extend([p for p in diagram_dir.rglob("*.plantuml") if p.is_file()])

    # 也检查 docs 和 discuss 目录（向后兼容）
    for base in (root / "docs", root / "discuss"):
        if not base.exists():
            continue
        candidates.extend([p for p in base.rglob("*.puml") if p.is_file()])
        candidates.extend([p for p in base.rglob("*.plantuml") if p.is_file()])

    if not candidates:
        return

    # 获取 PlantUML 命令
    plantuml_cmd = _get_plantuml_command(config)
    if plantuml_cmd is None:
        output.warn("未找到 PlantUML（jar 或系统命令），已跳过校验/PNG 生成")
        return

    # 先校验所有文件
    errors: list[str] = []
    for file_path in candidates:
        result = subprocess.run(
            plantuml_cmd + ["-checkonly", str(file_path)],
            cwd=root,
            text=True,
            capture_output=True,
        )
        if result.returncode != 0:
            detail = (result.stderr or "").strip() or (result.stdout or "").strip()
            errors.append(f"{file_path.name}: {detail}")

    if errors:
        error_msg = "\n".join(errors)
        raise FlowError(f"PlantUML 语法校验失败:\n{error_msg}")

    # 校验通过，生成 PNG
    for file_path in candidates:
        result = subprocess.run(
            plantuml_cmd + ["-tpng", str(file_path)],
            cwd=root,
            text=True,
            capture_output=True,
        )
        if result.returncode != 0:
            detail = (result.stderr or "").strip() or (result.stdout or "").strip()
            raise FlowError(f"PlantUML PNG 生成失败: {file_path} {detail}".strip())

    output.ok(f"PlantUML 处理完成: {len(candidates)} 个文件")


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


def _hook_clean_task_plans(*, root: Path, config: dict[str, Any] | None) -> None:
    """进入 finish 时清理任务计划文件。"""
    # 获取任务计划目录配置
    plans_path = ".aide/task-plans"
    if config:
        plans_path = config.get("task", {}).get("plans_path", plans_path)

    # 移除末尾斜杠（如有）
    plans_path = plans_path.rstrip("/")

    plans_dir = root / plans_path
    if not plans_dir.exists():
        return

    # 收集所有文件
    files = [f for f in plans_dir.iterdir() if f.is_file()]
    if not files:
        return

    # 删除文件
    for f in files:
        f.unlink()

    output.ok(f"已清理任务计划文件: {len(files)} 个")

