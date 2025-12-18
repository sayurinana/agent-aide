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
        # 使用 -A 确保删除的文件也被暂存，排除 .lock 文件避免锁文件被提交
        result = self._run(["add", "-A", "--", ".", ":!*.lock"], check=False)
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

    # === 分支管理新增方法 ===

    def get_current_branch(self) -> str:
        """获取当前分支名"""
        result = self._run(["rev-parse", "--abbrev-ref", "HEAD"], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error("获取当前分支失败", result))
        return (result.stdout or "").strip()

    def is_clean(self) -> bool:
        """检查工作目录是否干净（无未提交的变更）"""
        result = self._run(["status", "--porcelain"], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error("检查 git 状态失败", result))
        return not (result.stdout or "").strip()

    def has_commits(self) -> bool:
        """检查是否有提交历史"""
        result = self._run(["rev-parse", "HEAD"], check=False)
        return result.returncode == 0

    def create_branch(self, name: str, start_point: str | None = None) -> None:
        """创建新分支"""
        args = ["branch", name]
        if start_point:
            args.append(start_point)
        result = self._run(args, check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error(f"创建分支 {name} 失败", result))

    def checkout(self, branch: str) -> None:
        """切换到指定分支"""
        result = self._run(["checkout", branch], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error(f"切换到分支 {branch} 失败", result))

    def checkout_new_branch(self, name: str, start_point: str | None = None) -> None:
        """创建并切换到新分支"""
        args = ["checkout", "-b", name]
        if start_point:
            args.append(start_point)
        result = self._run(args, check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error(f"创建并切换到分支 {name} 失败", result))

    def has_commits_since(self, commit: str, branch: str) -> bool:
        """检查指定分支自某提交后是否有新提交"""
        result = self._run(["rev-list", f"{commit}..{branch}", "--count"], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error(f"检查分支 {branch} 新提交失败", result))
        count = int((result.stdout or "0").strip())
        return count > 0

    def reset_soft(self, commit: str) -> None:
        """软重置到指定提交"""
        result = self._run(["reset", "--soft", commit], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error(f"软重置到 {commit} 失败", result))

    def merge_squash(self, branch: str) -> None:
        """squash 合并指定分支"""
        result = self._run(["merge", "--squash", branch], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error(f"squash 合并分支 {branch} 失败", result))

    def amend(self) -> str:
        """将暂存区内容追加到上一次提交（不修改提交消息）"""
        result = self._run(["commit", "--amend", "--no-edit"], check=False)
        if result.returncode != 0:
            raise FlowError(_format_git_error("git commit --amend 失败", result))
        return self.rev_parse_head()

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
