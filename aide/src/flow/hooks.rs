use std::fs;
use std::path::Path;
use std::process::Command;

use crate::core::output;
use crate::flow::git::GitIntegration;
use crate::flow::types::FlowStatus;

pub fn run_pre_commit_hooks(
    root: &Path,
    git: &GitIntegration,
    status: Option<&FlowStatus>,
    from_phase: Option<&str>,
    to_phase: &str,
    action: &str,
    config: &toml::Value,
) -> Result<(), String> {
    if from_phase == Some("flow-design") && (action == "next-part" || action == "back-part") {
        hook_plantuml(root, config)?;
    }
    if from_phase == Some("docs") && (action == "next-part" || action == "back-part") {
        hook_changelog_on_leave_docs(root, git, status)?;
    }
    if to_phase == "finish" && action == "next-part" {
        hook_clean_task_plans(root, config);
    }
    Ok(())
}

pub fn run_post_commit_hooks(to_phase: &str, action: &str) {
    if to_phase == "docs" && (action == "start" || action == "next-part" || action == "back-part") {
        output::info("请更新 CHANGELOG.md");
    }
}

fn get_plantuml_command(config: &toml::Value) -> Option<Vec<String>> {
    // 优先使用全局配置中的自包含可执行程序
    if let Some(path) = crate::core::plantuml::get_plantuml_path(config) {
        if path.exists() {
            return Some(vec![path.to_string_lossy().to_string()]);
        }
    }

    // 回退到系统 PATH 中的 plantuml 命令
    if Command::new("plantuml")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        return Some(vec!["plantuml".to_string()]);
    }

    None
}

fn hook_plantuml(root: &Path, config: &toml::Value) -> Result<(), String> {
    let diagram_path = config
        .get("flow")
        .and_then(|f| f.get("diagram_path"))
        .and_then(|v| v.as_str())
        .unwrap_or("aide-memory/memory/diagram");

    let diagram_dir = root.join(diagram_path);

    let mut candidates: Vec<std::path::PathBuf> = Vec::new();

    // 收集 .puml / .plantuml 文件
    for dir in [&diagram_dir, &root.join("docs"), &root.join("discuss")] {
        if dir.exists() {
            collect_puml_files(dir, &mut candidates);
        }
    }

    if candidates.is_empty() {
        return Ok(());
    }

    let plantuml_cmd = match get_plantuml_command(config) {
        Some(cmd) => cmd,
        None => {
            output::warn("未找到 PlantUML（jar 或系统命令），已跳过校验/PNG 生成");
            return Ok(());
        }
    };

    // 语法检查
    let mut errors = Vec::new();
    for file_path in &candidates {
        let mut cmd = Command::new(&plantuml_cmd[0]);
        for arg in &plantuml_cmd[1..] {
            cmd.arg(arg);
        }
        cmd.arg("-checkonly").arg(file_path).current_dir(root);

        let result = cmd.output();
        if let Ok(output) = result {
            if !output.status.success() {
                let detail = String::from_utf8_lossy(&output.stderr).trim().to_string();
                let detail = if detail.is_empty() {
                    String::from_utf8_lossy(&output.stdout).trim().to_string()
                } else {
                    detail
                };
                errors.push(format!(
                    "{}: {detail}",
                    file_path.file_name().unwrap_or_default().to_string_lossy()
                ));
            }
        }
    }

    if !errors.is_empty() {
        return Err(format!("PlantUML 语法校验失败:\n{}", errors.join("\n")));
    }

    // 生成 PNG
    for file_path in &candidates {
        let mut cmd = Command::new(&plantuml_cmd[0]);
        for arg in &plantuml_cmd[1..] {
            cmd.arg(arg);
        }
        cmd.arg("-tpng").arg(file_path).current_dir(root);

        let result = cmd.output();
        if let Ok(output) = result {
            if !output.status.success() {
                let detail = String::from_utf8_lossy(&output.stderr).trim().to_string();
                return Err(format!(
                    "PlantUML PNG 生成失败: {} {detail}",
                    file_path.display()
                ));
            }
        }
    }

    output::ok(&format!("PlantUML 处理完成: {} 个文件", candidates.len()));
    Ok(())
}

fn collect_puml_files(dir: &Path, candidates: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_puml_files(&path, candidates);
            } else if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy();
                    if ext == "puml" || ext == "plantuml" {
                        candidates.push(path);
                    }
                }
            }
        }
    }
}

fn hook_changelog_on_leave_docs(
    root: &Path,
    git: &GitIntegration,
    status: Option<&FlowStatus>,
) -> Result<(), String> {
    let changelog = root.join("CHANGELOG.md");
    if !changelog.exists() {
        return Err("离开 docs 前需要更新 CHANGELOG.md（当前文件不存在）".into());
    }

    git.ensure_repo()?;

    // 检查工作目录中是否有未暂存的 CHANGELOG.md 变更
    let porcelain = git.status_porcelain("CHANGELOG.md")?;
    if !porcelain.trim().is_empty() {
        return Ok(());
    }

    let status = match status {
        Some(s) => s,
        None => {
            return Err("离开 docs 前需要更新 CHANGELOG.md（未找到流程状态）".into());
        }
    };

    for entry in &status.history {
        if entry.phase != "docs" {
            continue;
        }
        if let Some(commit) = &entry.git_commit {
            if git.commit_touches_path(commit, "CHANGELOG.md")? {
                return Ok(());
            }
        }
    }

    Err("离开 docs 前需要更新 CHANGELOG.md（未检测到 docs 阶段的更新记录）".into())
}

fn hook_clean_task_plans(root: &Path, config: &toml::Value) {
    let plans_path = config
        .get("task")
        .and_then(|t| t.get("plans_path"))
        .and_then(|v| v.as_str())
        .unwrap_or("aide-memory/task-plans")
        .trim_end_matches('/');

    let plans_dir = root.join(plans_path);
    if !plans_dir.exists() {
        return;
    }

    let mut count = 0;
    if let Ok(entries) = fs::read_dir(&plans_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let _ = fs::remove_file(&path);
                count += 1;
            }
        }
    }

    if count > 0 {
        output::ok(&format!("已清理任务计划文件: {count} 个"));
    }
}
