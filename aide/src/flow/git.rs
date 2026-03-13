use std::path::{Path, PathBuf};
use std::process::Command;

pub struct GitIntegration {
    pub root: PathBuf,
}

impl GitIntegration {
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
        }
    }

    pub fn ensure_available(&self) -> Result<(), String> {
        if which_git().is_none() {
            return Err("未找到 git 命令，请先安装 git".into());
        }
        Ok(())
    }

    pub fn ensure_repo(&self) -> Result<(), String> {
        self.ensure_available()?;
        let result = self.run(&["rev-parse", "--is-inside-work-tree"]);
        match result {
            Ok(output) if output.contains("true") => Ok(()),
            _ => Err("当前目录不是 git 仓库，请先执行 git init 或切换到正确目录".into()),
        }
    }

    pub fn add_all(&self) -> Result<(), String> {
        self.ensure_repo()?;
        self.run_checked(&["add", "-A", "--", ".", ":!*.lock"])
            .map_err(|e| format!("git add 失败: {e}"))?;
        Ok(())
    }

    pub fn commit(&self, message: &str) -> Result<Option<String>, String> {
        self.ensure_repo()?;
        // Check if there are staged changes
        let diff = self.run_exit_code(&["diff", "--cached", "--quiet"]);
        if diff == 0 {
            return Ok(None); // Nothing to commit
        }

        self.run_checked(&["commit", "-m", message])
            .map_err(|e| format!("git commit 失败: {e}"))?;
        let hash = self.rev_parse_head()?;
        Ok(Some(hash))
    }

    pub fn rev_parse_head(&self) -> Result<String, String> {
        self.run(&["rev-parse", "HEAD"])
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("获取 commit hash 失败: {e}"))
    }

    pub fn status_porcelain(&self, path: &str) -> Result<String, String> {
        self.run(&["status", "--porcelain", "--", path])
            .map_err(|e| format!("git status 失败: {e}"))
    }

    pub fn commit_touches_path(&self, commit_hash: &str, path: &str) -> Result<bool, String> {
        let output = self
            .run(&["show", "--name-only", "--pretty=format:", commit_hash])
            .map_err(|e| format!("读取提交内容失败: {commit_hash}: {e}"))?;
        let files: Vec<&str> = output.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
        Ok(files.contains(&path))
    }

    pub fn get_current_branch(&self) -> Result<String, String> {
        self.run(&["rev-parse", "--abbrev-ref", "HEAD"])
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("获取当前分支失败: {e}"))
    }

    pub fn is_clean(&self) -> Result<bool, String> {
        let output = self
            .run(&["status", "--porcelain"])
            .map_err(|e| format!("检查 git 状态失败: {e}"))?;
        Ok(output.trim().is_empty())
    }

    pub fn has_commits(&self) -> bool {
        self.run(&["rev-parse", "HEAD"]).is_ok()
    }

    pub fn checkout_new_branch(&self, name: &str, start_point: Option<&str>) -> Result<(), String> {
        let mut args = vec!["checkout", "-b", name];
        if let Some(sp) = start_point {
            args.push(sp);
        }
        self.run_checked(&args)
            .map_err(|e| format!("创建并切换到分支 {name} 失败: {e}"))?;
        Ok(())
    }

    pub fn checkout(&self, branch: &str) -> Result<(), String> {
        self.run_checked(&["checkout", branch])
            .map_err(|e| format!("切换到分支 {branch} 失败: {e}"))?;
        Ok(())
    }

    pub fn has_commits_since(&self, commit: &str, branch: &str) -> Result<bool, String> {
        let output = self
            .run(&["rev-list", &format!("{commit}..{branch}"), "--count"])
            .map_err(|e| format!("检查分支 {branch} 新提交失败: {e}"))?;
        let count: i64 = output.trim().parse().unwrap_or(0);
        Ok(count > 0)
    }

    pub fn merge_squash(&self, branch: &str) -> Result<(), String> {
        self.run_checked(&["merge", "--squash", branch])
            .map_err(|e| format!("squash 合并分支 {branch} 失败: {e}"))?;
        Ok(())
    }

    fn run(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.root)
            .output()
            .map_err(|e| format!("执行 git 命令失败: {e}"))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Err(if stderr.is_empty() { stdout } else { stderr })
        }
    }

    fn run_checked(&self, args: &[&str]) -> Result<(), String> {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.root)
            .output()
            .map_err(|e| format!("执行 git 命令失败: {e}"))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Err(if stderr.is_empty() { stdout } else { stderr })
        }
    }

    fn run_exit_code(&self, args: &[&str]) -> i32 {
        Command::new("git")
            .args(args)
            .current_dir(&self.root)
            .output()
            .map(|o| o.status.code().unwrap_or(-1))
            .unwrap_or(-1)
    }
}

fn which_git() -> Option<PathBuf> {
    // Try running git --version to check availability
    match Command::new("git").arg("--version").output() {
        Ok(output) if output.status.success() => Some(PathBuf::from("git")),
        _ => None,
    }
}
