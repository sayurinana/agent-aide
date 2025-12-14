"""Git 操作封装：add、commit、查询提交变更文件。"""

from __future__ import annotations

import shutil
import subprocess
from pathlib import Path

from aide.flow.errors import FlowError


class GitIntegration:
    def __init__(self, root: Path):
        self.root = root

    def ensure_available(self) -> None:
        if shutil.which("git") is None:
            raise FlowError("未找到 git 命令，请先安装 git")

    def ensure_repo(self) -> None:
        self.ensure_available()
        result = self._run(["rev-parse", "--is-inside-work-tree"], check=False)
        if result.returncode != 0 or "true" not in (result.stdout or ""):
            raise FlowError("当前目录不是 git 仓库，请先执行 git init 或切换到正确目录")

    def add_all(self) -> None:
        self.ensure_repo()
        result = self._run(["add", "."], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error("git add 失败", result))

    def commit(self, message: str) -> str | None:
        self.ensure_repo()
        diff = self._run(["diff", "--cached", "--quiet"], check=False)
        if diff.returncode == 0:
            return None
        if diff.returncode != 1:
            raise FlowError(_format_git_error("git diff 失败", diff))

        result = self._run(["commit", "-m", message], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error("git commit 失败", result))
        return self.rev_parse_head()

    def rev_parse_head(self) -> str:
        result = self._run(["rev-parse", "HEAD"], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error("获取 commit hash 失败", result))
        return (result.stdout or "").strip()

    def status_porcelain(self, path: str) -> str:
        result = self._run(["status", "--porcelain", "--", path], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error("git status 失败", result))
        return result.stdout or ""

    def commit_touches_path(self, commit_hash: str, path: str) -> bool:
        result = self._run(["show", "--name-only", "--pretty=format:", commit_hash], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error(f"读取提交内容失败: {commit_hash}", result))
        files = [line.strip() for line in (result.stdout or "").splitlines() if line.strip()]
        return path in files

    def _run(self, args: list[str], check: bool) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            ["git", *args],
            cwd=self.root,
            text=True,
            capture_output=True,
            check=check,
        )


def _format_git_error(prefix: str, result: subprocess.CompletedProcess[str]) -> str:
    detail = (result.stderr or "").strip() or (result.stdout or "").strip()
    if not detail:
        return prefix
    return f"{prefix}: {detail}"
