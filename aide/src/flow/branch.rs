use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::core::config::ConfigManager;
use crate::flow::git::GitIntegration;
use crate::utils::now_iso;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub number: i64,
    pub branch_name: String,
    pub source_branch: String,
    pub start_commit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_commit: Option<String>,
    pub task_id: String,
    pub task_summary: String,
    pub started_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(default = "default_active")]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_branch: Option<String>,
}

fn default_active() -> String {
    "active".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchesData {
    pub next_number: i64,
    pub branches: Vec<BranchInfo>,
}

impl Default for BranchesData {
    fn default() -> Self {
        Self {
            next_number: 1,
            branches: Vec::new(),
        }
    }
}

pub fn load_branches_data(path: &Path) -> Result<BranchesData, String> {
    if !path.exists() {
        return Ok(BranchesData::default());
    }

    let content = fs::read_to_string(path).map_err(|e| format!("读取分支概况失败: {e}"))?;
    serde_json::from_str(&content).map_err(|e| format!("读取分支概况失败: {e}"))
}

pub fn render_branches_markdown(data: &BranchesData) -> String {
    let mut lines = vec!["# Git 分支概况\n".to_string()];

    if data.branches.is_empty() {
        lines.push("暂无分支记录。\n".to_string());
        return lines.join("\n");
    }

    for branch in data.branches.iter().rev() {
        lines.push(format!("## {}\n", branch.branch_name));
        lines.push(format!("- **任务**: {}", branch.task_summary));
        lines.push(format!("- **任务ID**: {}", branch.task_id));
        lines.push(format!("- **源分支**: {}", branch.source_branch));
        lines.push(format!(
            "- **起始提交**: {}",
            &branch.start_commit[..7.min(branch.start_commit.len())]
        ));
        if let Some(ec) = &branch.end_commit {
            lines.push(format!("- **结束提交**: {}", &ec[..7.min(ec.len())]));
        }
        lines.push(format!("- **状态**: {}", branch.status));
        let start_time = &branch.started_at[..16.min(branch.started_at.len())];
        lines.push(format!("- **起始时间**: {}", start_time.replace('T', " ")));
        if let Some(ft) = &branch.finished_at {
            let end_time = &ft[..16.min(ft.len())];
            lines.push(format!("- **结束时间**: {}", end_time.replace('T', " ")));
        }
        if let Some(tb) = &branch.temp_branch {
            lines.push(format!("- **临时分支**: {tb}"));
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

pub fn save_branches_data(
    branches_json: &Path,
    branches_md: &Path,
    data: &BranchesData,
) -> Result<(), String> {
    let json_content =
        serde_json::to_string_pretty(data).map_err(|e| format!("序列化分支概况失败: {e}"))?;
    fs::write(branches_json, format!("{json_content}\n"))
        .map_err(|e| format!("保存分支概况失败: {e}"))?;
    fs::write(branches_md, render_branches_markdown(data))
        .map_err(|e| format!("保存分支概况失败: {e}"))?;
    Ok(())
}

fn clean_task_summary(task_summary: &str) -> &str {
    let prefixes = [
        "开始任务准备: ",
        "开始任务准备:",
        "开始任务准备： ",
        "开始任务准备：",
    ];
    for prefix in &prefixes {
        if let Some(rest) = task_summary.strip_prefix(prefix) {
            return rest;
        }
    }
    task_summary
}

pub struct BranchManager {
    pub root: PathBuf,
    pub git: GitIntegration,
    aide_dir: PathBuf,
    branches_json: PathBuf,
    branches_md: PathBuf,
    lock_path: PathBuf,
    logs_dir: PathBuf,
    data: Option<BranchesData>,
    current_branch_info: Option<BranchInfo>,
    cfg: ConfigManager,
}

impl BranchManager {
    pub fn new(root: &Path, _cfg: &ConfigManager) -> Self {
        let aide_dir = root.join(crate::core::config::AIDE_MEMORY_DIR);
        Self {
            root: root.to_path_buf(),
            git: GitIntegration::new(root),
            branches_json: aide_dir.join("branches.json"),
            branches_md: aide_dir.join("branches.md"),
            lock_path: aide_dir.join("flow-status.lock"),
            logs_dir: aide_dir.join("logs"),
            cfg: ConfigManager::new(root),
            aide_dir,
            data: None,
            current_branch_info: None,
        }
    }

    fn cleanup_lock_file(&self) {
        let _ = fs::remove_file(&self.lock_path);
    }

    fn cleanup_task_files(&self, task_id: &str) {
        let _ = fs::create_dir_all(&self.logs_dir);

        // 1. 删除所有 .lock 文件
        if let Ok(entries) = fs::read_dir(&self.aide_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "lock") {
                    let _ = fs::remove_file(&path);
                }
            }
        }

        // 2. 删除任务细则文件
        if let Some(spec) = self
            .cfg
            .get_value("task.spec")
            .and_then(|v| v.as_str().map(String::from))
        {
            let spec_path = self.root.join(&spec);
            if spec_path.exists() {
                let _ = fs::remove_file(&spec_path);
            }
        }

        // 3. 清空任务原文件
        if let Some(source) = self
            .cfg
            .get_value("task.source")
            .and_then(|v| v.as_str().map(String::from))
        {
            let source_path = self.root.join(&source);
            if source_path.exists() {
                let _ = fs::write(&source_path, "");
            }
        }

        // 4. 备份并删除 flow-status.json
        let status_path = self.aide_dir.join("flow-status.json");
        if status_path.exists() {
            let backup_name = format!("{task_id}-status.json");
            let backup_path = self.logs_dir.join(&backup_name);
            let _ = fs::copy(&status_path, &backup_path);
            let _ = fs::remove_file(&status_path);
        }

        // 5. 删除 decisions/*.json
        let decisions_dir = self.aide_dir.join("decisions");
        if decisions_dir.exists() {
            if let Ok(entries) = fs::read_dir(&decisions_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|ext| ext == "json") {
                        let _ = fs::remove_file(&path);
                    }
                }
            }
        }

        // 6. 删除 pending-items.json
        let pending_path = self.aide_dir.join("pending-items.json");
        let _ = fs::remove_file(&pending_path);

        // 7. 删除流程图目录下的文件
        let diagram_path = self
            .cfg
            .get_value("flow.diagram_path")
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "aide-memory/memory/diagram".into());
        let diagram_dir = self.root.join(&diagram_path);
        if diagram_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&diagram_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            let ext = ext.to_string_lossy();
                            if ext == "puml" || ext == "plantuml" || ext == "png" {
                                let _ = fs::remove_file(&path);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn load_branches(&mut self) -> Result<&BranchesData, String> {
        if self.data.is_some() {
            return Ok(self.data.as_ref().unwrap());
        }

        self.data = Some(load_branches_data(&self.branches_json)?);
        Ok(self.data.as_ref().unwrap())
    }

    pub fn save_branches(&self) -> Result<(), String> {
        let data = match &self.data {
            Some(d) => d,
            None => return Ok(()),
        };

        let _ = fs::create_dir_all(&self.aide_dir);
        save_branches_data(&self.branches_json, &self.branches_md, data)
    }

    pub fn create_task_branch(
        &mut self,
        task_id: &str,
        task_summary: &str,
    ) -> Result<String, String> {
        self.git.ensure_repo()?;
        self.load_branches()?;

        // 确保 git 状态干净
        if !self.git.is_clean()? {
            self.git.add_all()?;
            self.git.commit("[aide] 保存未提交的变更")?;
        }

        // 确保有提交历史
        if !self.git.has_commits() {
            let gitkeep = self.root.join(".gitkeep");
            if !gitkeep.exists() {
                let _ = fs::write(&gitkeep, "");
            }
            self.git.add_all()?;
            self.git.commit("[aide] 初始提交")?;
        }

        let source_branch = self.git.get_current_branch()?;
        let start_commit = self.git.rev_parse_head()?;

        let data = self.data.as_mut().unwrap();
        let branch_number = data.next_number;
        let branch_name = format!("aide/{branch_number:03}");

        self.git.checkout_new_branch(&branch_name, None)?;

        let branch_info = BranchInfo {
            number: branch_number,
            branch_name: branch_name.clone(),
            source_branch,
            start_commit,
            end_commit: None,
            task_id: task_id.to_string(),
            task_summary: task_summary.to_string(),
            started_at: now_iso(),
            finished_at: None,
            status: "active".into(),
            temp_branch: None,
        };

        data.branches.push(branch_info.clone());
        data.next_number = branch_number + 1;
        self.current_branch_info = Some(branch_info);

        self.save_branches()?;
        Ok(branch_name)
    }

    pub fn get_active_branch_info(&mut self) -> Result<Option<BranchInfo>, String> {
        if self.current_branch_info.is_some() {
            return Ok(self.current_branch_info.clone());
        }

        self.load_branches()?;
        let current_branch = self.git.get_current_branch()?;
        let data = self.data.as_ref().unwrap();

        for branch in &data.branches {
            if branch.branch_name == current_branch && branch.status == "active" {
                self.current_branch_info = Some(branch.clone());
                return Ok(Some(branch.clone()));
            }
        }

        Ok(None)
    }

    pub fn finish_branch_merge(
        &mut self,
        task_summary: &str,
        end_commit: Option<&str>,
        finished_at: Option<&str>,
    ) -> Result<(bool, String), String> {
        let branch_info = match self.get_active_branch_info()? {
            Some(bi) => bi,
            None => return Ok((true, "未找到活跃的任务分支，跳过合并".into())),
        };

        let start_commit = self.resolve_branch_base(&branch_info);
        let source_branch = &branch_info.source_branch;

        if self.git.has_commits_since(&start_commit, source_branch)? {
            self.merge_with_temp_branch(
                &branch_info,
                &start_commit,
                task_summary,
                false,
                end_commit,
                finished_at,
            )
        } else {
            self.merge_normal(&branch_info, task_summary, false, end_commit, finished_at)
        }
    }

    pub fn clean_branch_merge(&mut self) -> Result<(bool, String), String> {
        if !self.git.is_clean()? {
            self.git.add_all()?;
            self.git.commit("[aide] 强制清理前保存未提交的变更")?;
        }

        let branch_info = match self.get_active_branch_info()? {
            Some(bi) => bi,
            None => return Err("未找到活跃的任务分支".into()),
        };

        let start_commit = self.resolve_branch_base(&branch_info);
        let source_branch = &branch_info.source_branch;

        if self.git.has_commits_since(&start_commit, source_branch)? {
            self.merge_with_temp_branch(&branch_info, &start_commit, "强制清理", true, None, None)
        } else {
            self.merge_normal(&branch_info, "强制清理", true, None, None)
        }
    }

    fn merge_normal(
        &mut self,
        branch_info: &BranchInfo,
        task_summary: &str,
        is_force_clean: bool,
        end_commit: Option<&str>,
        finished_at: Option<&str>,
    ) -> Result<(bool, String), String> {
        let source_branch = branch_info.source_branch.clone();
        let task_branch = branch_info.branch_name.clone();
        let task_id = branch_info.task_id.clone();
        let branch_number = branch_info.number;

        let (end_commit_val, finished_at_val) = if is_force_clean {
            self.git.add_all()?;
            self.git
                .commit(&format!("[aide] 强制清理: {task_summary}"))?;
            (self.git.rev_parse_head()?, now_iso())
        } else {
            (
                end_commit
                    .map(String::from)
                    .unwrap_or_else(|| self.git.rev_parse_head().unwrap_or_default()),
                finished_at.map(String::from).unwrap_or_else(now_iso),
            )
        };

        // 更新分支状态
        let status = if is_force_clean {
            "force-cleaned"
        } else {
            "finished"
        };
        self.update_branch_data(branch_number, |b| {
            b.end_commit = Some(end_commit_val.clone());
            b.finished_at = Some(finished_at_val.clone());
            b.status = status.to_string();
        });
        self.current_branch_info = None;
        self.save_branches()?;

        // 提交状态更新
        self.git.add_all()?;
        let status_msg = if is_force_clean {
            "[aide] 强制清理: 更新状态"
        } else {
            "[aide] finish: 更新状态"
        };
        self.git.commit(status_msg)?;

        // 清理任务文件
        self.cleanup_task_files(&task_id);

        // 创建清理提交
        self.git.add_all()?;
        self.git.commit("[aide] 清理任务临时文件")?;

        // 切回源分支
        self.git.checkout(&source_branch)?;
        self.cleanup_lock_file();

        // squash 合并
        self.git.merge_squash(&task_branch)?;

        // 收尾提交
        self.git.add_all()?;
        let clean_summary = clean_task_summary(&branch_info.task_summary);
        let commit_msg = if is_force_clean {
            format!("任务中断，清理：{task_branch} - {clean_summary}")
        } else {
            format!("完成：{task_branch} - {clean_summary}")
        };
        self.git.commit(&commit_msg)?;

        Ok((true, format!("任务分支已合并到 {source_branch}")))
    }

    fn merge_with_temp_branch(
        &mut self,
        branch_info: &BranchInfo,
        branch_base_commit: &str,
        task_summary: &str,
        is_force_clean: bool,
        end_commit: Option<&str>,
        finished_at: Option<&str>,
    ) -> Result<(bool, String), String> {
        let task_branch = branch_info.branch_name.clone();
        let task_id = branch_info.task_id.clone();
        let branch_number = branch_info.number;
        let temp_branch = format!("{task_branch}-merge");

        let (end_commit_val, finished_at_val) = if is_force_clean {
            self.git.add_all()?;
            self.git
                .commit(&format!("[aide] 强制清理: {task_summary}"))?;
            (self.git.rev_parse_head()?, now_iso())
        } else {
            (
                end_commit
                    .map(String::from)
                    .unwrap_or_else(|| self.git.rev_parse_head().unwrap_or_default()),
                finished_at.map(String::from).unwrap_or_else(now_iso),
            )
        };

        let status = if is_force_clean {
            "force-cleaned-to-temp"
        } else {
            "merged-to-temp"
        };
        self.update_branch_data(branch_number, |b| {
            b.end_commit = Some(end_commit_val.clone());
            b.finished_at = Some(finished_at_val.clone());
            b.status = status.to_string();
            b.temp_branch = Some(temp_branch.clone());
        });
        self.current_branch_info = None;
        self.save_branches()?;

        // 提交状态更新
        self.git.add_all()?;
        let status_msg = if is_force_clean {
            "[aide] 强制清理: 更新状态"
        } else {
            "[aide] finish: 更新状态"
        };
        self.git.commit(status_msg)?;

        // 清理任务文件
        self.cleanup_task_files(&task_id);

        // 创建清理提交
        self.git.add_all()?;
        self.git.commit("[aide] 清理任务临时文件")?;

        // 从起始提交检出临时分支
        self.git
            .checkout_new_branch(&temp_branch, Some(branch_base_commit))?;
        self.cleanup_lock_file();

        // squash 合并
        self.git.merge_squash(&task_branch)?;

        // 创建压缩提交
        self.git.add_all()?;
        let clean_summary = clean_task_summary(task_summary);
        let commit_msg = if is_force_clean {
            format!("[aide] 强制清理压缩提交: {clean_summary}")
        } else {
            format!("[aide] 任务压缩提交: {clean_summary}")
        };
        self.git.commit(&commit_msg)?;

        let action_name = if is_force_clean {
            "强制清理"
        } else {
            "任务完成"
        };
        Ok((
            false,
            format!(
                "\u{26A0} 源分支 {} 有新提交\n已在临时分支 {temp_branch} 完成{action_name}合并\n请手动处理后续操作",
                branch_info.source_branch
            ),
        ))
    }

    fn update_branch_data<F>(&mut self, branch_number: i64, update_fn: F)
    where
        F: FnOnce(&mut BranchInfo),
    {
        if let Some(data) = &mut self.data {
            for branch in &mut data.branches {
                if branch.number == branch_number {
                    update_fn(branch);
                    break;
                }
            }
        }
    }

    fn resolve_branch_base(&self, branch_info: &BranchInfo) -> String {
        self.git
            .merge_base(&branch_info.branch_name, &branch_info.source_branch)
            .unwrap_or_else(|_| branch_info.start_commit.clone())
    }
}
