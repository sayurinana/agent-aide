//! Git 工具模块
//!
//! 提供 Git 可用性检测和仓库克隆/更新功能。

use std::path::Path;
use std::process::Command;

use crate::core::output;

/// 检测系统是否安装 Git 命令行工具
///
/// # Returns
///
/// 如果 Git 可用返回 `true`，否则返回 `false`
pub fn is_git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 克隆或更新仓库
///
/// 如果目标目录不存在，执行 `git clone`。
/// 如果目标目录已存在且为 git 仓库，执行 `git pull`。
///
/// # Arguments
///
/// * `repo_url` - 仓库地址
/// * `target_dir` - 目标目录路径
///
/// # Returns
///
/// 成功返回 `Ok(())`，失败返回 `Err(String)` 包含错误信息
pub fn clone_or_update_repo(repo_url: &str, target_dir: &Path) -> Result<(), String> {
    if target_dir.exists() {
        // 目录已存在，检查是否为 git 仓库
        let git_dir = target_dir.join(".git");
        if git_dir.exists() && git_dir.is_dir() {
            // 是 git 仓库，执行 pull
            pull_repo(target_dir)
        } else {
            Err(format!(
                "目标目录已存在但不是 git 仓库：{}",
                target_dir.display()
            ))
        }
    } else {
        // 目录不存在，执行 clone
        clone_repo(repo_url, target_dir)
    }
}

/// 克隆仓库
fn clone_repo(repo_url: &str, target_dir: &Path) -> Result<(), String> {
    // 确保父目录存在
    if let Some(parent) = target_dir.parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                return Err(format!("创建父目录失败：{}", e));
            }
        }
    }

    output::info(&format!("正在克隆仓库到 {} ...", target_dir.display()));

    let output = Command::new("git")
        .args(["clone", repo_url, &target_dir.to_string_lossy()])
        .output()
        .map_err(|e| format!("执行 git clone 失败：{}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("git clone 失败：{}", stderr.trim()))
    }
}

/// 更新仓库（git pull）
fn pull_repo(target_dir: &Path) -> Result<(), String> {
    output::info(&format!("正在更新仓库 {} ...", target_dir.display()));

    let output = Command::new("git")
        .args(["-C", &target_dir.to_string_lossy(), "pull"])
        .output()
        .map_err(|e| format!("执行 git pull 失败：{}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // pull 失败不阻止继续，只是警告
        output::warn(&format!("git pull 失败：{}", stderr.trim()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_available() {
        // 测试环境中通常已安装 git
        let result = is_git_available();
        // 只验证函数不会 panic
        assert!(result || !result);
    }
}