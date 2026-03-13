use std::path::Path;

use crate::core::config::{ConfigManager, get_phases};
use crate::core::output;
use crate::flow::branch::BranchManager;
use crate::flow::git::GitIntegration;
use crate::flow::hooks::{run_post_commit_hooks, run_pre_commit_hooks};
use crate::flow::storage::FlowStorage;
use crate::flow::types::{FlowStatus, HistoryEntry};
use crate::flow::validator::FlowValidator;
use crate::utils::{normalize_text, now_iso, now_task_id};

pub struct FlowTracker {
    root: std::path::PathBuf,
    cfg: ConfigManager,
    storage: FlowStorage,
    git: GitIntegration,
}

impl FlowTracker {
    pub fn new(root: &Path, _cfg: &ConfigManager) -> Self {
        Self {
            root: root.to_path_buf(),
            cfg: ConfigManager::new(root),
            storage: FlowStorage::new(root),
            git: GitIntegration::new(root),
        }
    }

    pub fn start(&mut self, phase: &str, summary: &str) -> bool {
        self.run("start", Some(phase), summary)
    }

    pub fn next_step(&mut self, summary: &str) -> bool {
        self.run("next-step", None, summary)
    }

    pub fn back_step(&mut self, reason: &str) -> bool {
        self.run("back-step", None, reason)
    }

    pub fn next_part(&mut self, phase: &str, summary: &str) -> bool {
        self.run("next-part", Some(phase), summary)
    }

    pub fn back_part(&mut self, phase: &str, reason: &str) -> bool {
        match self.do_back_part(phase, reason) {
            Ok(()) => true,
            Err(e) => {
                output::err(&e);
                false
            }
        }
    }

    pub fn back_confirm(&mut self, key: &str) -> bool {
        match self.do_back_confirm(key) {
            Ok(()) => true,
            Err(e) => {
                output::err(&e);
                false
            }
        }
    }

    pub fn issue(&mut self, description: &str) -> bool {
        self.run("issue", None, description)
    }

    pub fn error(&mut self, description: &str) -> bool {
        self.run("error", None, description)
    }

    pub fn clean(&mut self) -> bool {
        match self.do_clean() {
            Ok(()) => true,
            Err(e) => {
                output::err(&e);
                false
            }
        }
    }

    fn do_back_part(&mut self, phase: &str, reason: &str) -> Result<(), String> {
        self.storage.ensure_ready()?;

        if self.storage.has_pending_back_confirm() {
            if let Some(state) = self.storage.load_back_confirm_state()? {
                output::warn("已存在待确认的返工请求");
                output::info(&format!("目标环节: {}", state.target_part));
                output::info(&format!("原因: {}", state.reason));
                output::info(&format!(
                    "请执行: aide flow back-confirm --key {}",
                    state.pending_key
                ));
                return Err(String::new()); // Silent - already printed
            }
        }

        let key = self.storage.save_back_confirm_state(phase, reason)?;
        output::warn("返工需要确认。请先完成以下准备工作:");
        output::info("1. 触发 rework skill 学习返工流程指南");
        output::info("2. 按照指南更新任务文档（记录返工原因和新需求）");
        output::info("3. 完成准备工作后执行:");
        output::info(&format!("   aide flow back-confirm --key {key}"));
        Ok(())
    }

    fn do_back_confirm(&mut self, key: &str) -> Result<(), String> {
        self.storage.ensure_ready()?;

        let state = self.storage.load_back_confirm_state()?;
        let state = match state {
            Some(s) => s,
            None => return Err("无待确认的返工请求".into()),
        };

        if state.pending_key != key {
            return Err("确认 key 不匹配".into());
        }

        let target_part = state.target_part.clone();
        let reason = state.reason.clone();

        self.storage.clear_back_confirm_state()?;

        let result = self.run("back-part", Some(&target_part), &reason);

        if result {
            output::warn("建议执行 /exit 重新开始对话");
        }

        Ok(())
    }

    fn do_clean(&mut self) -> Result<(), String> {
        self.storage.ensure_ready()?;

        let _lock = self.storage.acquire_lock()?;
        let status = self.storage.load_status()?;
        if status.is_none() {
            return Err("未找到活跃任务，无需清理".into());
        }

        let mut branch_mgr = BranchManager::new(&self.root, &self.cfg);
        let (success, msg) = branch_mgr.clean_branch_merge()?;

        if success {
            output::ok(&format!("强制清理完成: {msg}"));
        } else {
            output::warn(&msg);
        }

        Ok(())
    }

    fn run(&mut self, action: &str, to_phase: Option<&str>, text: &str) -> bool {
        match self.do_run(action, to_phase, text) {
            Ok(()) => true,
            Err(e) => {
                if !e.is_empty() {
                    output::err(&e);
                }
                false
            }
        }
    }

    fn do_run(
        &mut self,
        action: &str,
        to_phase: Option<&str>,
        text: &str,
    ) -> Result<(), String> {
        self.storage.ensure_ready()?;
        let config = self.cfg.load_config();
        let phases = get_phases(&config);
        let validator = FlowValidator::new(phases)?;

        let normalized_text = normalize_text(text);
        if normalized_text.is_empty() {
            return Err("文本参数不能为空".into());
        }

        let _lock = self.storage.acquire_lock()?;

        if action == "start" {
            let to_phase = to_phase.ok_or("内部错误：start 缺少 phase")?;
            validator.validate_start(to_phase)?;
            self.storage.archive_existing_status()?;

            let task_id = now_task_id();
            let mut branch_mgr = BranchManager::new(&self.root, &self.cfg);
            let task_branch = branch_mgr.create_task_branch(&task_id, &normalized_text)?;
            let branch_info = branch_mgr.get_active_branch_info()?;

            let status = FlowStatus {
                task_id,
                current_phase: to_phase.to_string(),
                current_step: 0,
                started_at: now_iso(),
                history: Vec::new(),
                source_branch: branch_info.as_ref().map(|b| b.source_branch.clone()),
                start_commit: branch_info.as_ref().map(|b| b.start_commit.clone()),
                task_branch: Some(task_branch.clone()),
            };

            let (updated, commit_msg) = self.apply_action(
                &status,
                action,
                None,
                to_phase,
                &normalized_text,
                &validator,
                &config,
            )?;
            self.storage.save_status(&updated)?;
            let final_status = self.do_git_commit(&updated, &commit_msg)?;
            self.storage.save_status(&final_status)?;
            output::ok(&format!("任务开始: {to_phase} (分支: {task_branch})"));
            run_post_commit_hooks(to_phase, action);
            return Ok(());
        }

        let status = self
            .storage
            .load_status()?
            .ok_or("未找到流程状态，请先运行：aide flow start <环节名> \"<总结>\"")?;

        let current_phase = status.current_phase.clone();
        validator.validate_phase_exists(&current_phase)?;

        let to_phase = match action {
            "next-part" => {
                let tp = to_phase.ok_or("内部错误：next-part 缺少 phase")?;
                validator.validate_next_part(&current_phase, tp)?;
                tp.to_string()
            }
            "back-part" => {
                let tp = to_phase.ok_or("内部错误：back-part 缺少 phase")?;
                validator.validate_back_part(&current_phase, tp)?;
                tp.to_string()
            }
            _ => current_phase.clone(),
        };

        let (updated, commit_msg) = self.apply_action(
            &status,
            action,
            Some(&current_phase),
            &to_phase,
            &normalized_text,
            &validator,
            &config,
        )?;
        self.storage.save_status(&updated)?;
        let final_status = self.do_git_commit(&updated, &commit_msg)?;
        self.storage.save_status(&final_status)?;

        // finish 环节合并
        if action == "next-part" && to_phase == "finish" {
            let finish_commit = final_status
                .history
                .last()
                .and_then(|e| e.git_commit.clone());
            let finish_timestamp = final_status
                .history
                .last()
                .map(|e| e.timestamp.clone());

            let mut branch_mgr = BranchManager::new(&self.root, &self.cfg);
            let (success, merge_msg) = branch_mgr.finish_branch_merge(
                &normalized_text,
                finish_commit.as_deref(),
                finish_timestamp.as_deref(),
            )?;
            if !success {
                output::warn(&merge_msg);
            }
        }

        match action {
            "next-part" => output::ok(&format!("进入环节: {to_phase}")),
            "back-part" => output::warn(&format!("回退到环节: {to_phase}")),
            "error" => output::err(&format!("错误已记录: {normalized_text}")),
            _ => {}
        }

        run_post_commit_hooks(&to_phase, action);
        Ok(())
    }

    fn apply_action(
        &self,
        status: &FlowStatus,
        action: &str,
        from_phase: Option<&str>,
        to_phase: &str,
        text: &str,
        _validator: &FlowValidator,
        config: &toml::Value,
    ) -> Result<(FlowStatus, String), String> {
        // Run pre-commit hooks
        run_pre_commit_hooks(
            &self.root,
            &self.git,
            Some(status),
            from_phase,
            to_phase,
            action,
            config,
        )?;

        let message = build_commit_message(action, to_phase, text);
        let next_step = status.current_step + 1;

        let entry = HistoryEntry {
            timestamp: now_iso(),
            action: action.to_string(),
            phase: to_phase.to_string(),
            step: next_step,
            summary: text.to_string(),
            git_commit: None,
        };

        let mut history = status.history.clone();
        history.push(entry);

        let updated = FlowStatus {
            task_id: status.task_id.clone(),
            current_phase: to_phase.to_string(),
            current_step: next_step,
            started_at: status.started_at.clone(),
            history,
            source_branch: status.source_branch.clone(),
            start_commit: status.start_commit.clone(),
            task_branch: status.task_branch.clone(),
        };

        Ok((updated, message))
    }

    fn do_git_commit(
        &self,
        status: &FlowStatus,
        message: &str,
    ) -> Result<FlowStatus, String> {
        self.git.add_all()?;
        let commit_hash = self.git.commit(message)?;

        if !status.history.is_empty() {
            let mut updated = status.clone();
            if let Some(last) = updated.history.last_mut() {
                last.git_commit = commit_hash;
            }
            return Ok(updated);
        }

        Ok(status.clone())
    }
}

fn build_commit_message(action: &str, phase: &str, text: &str) -> String {
    match action {
        "issue" => format!("[aide] {phase} issue: {text}"),
        "error" => format!("[aide] {phase} error: {text}"),
        "back-step" => format!("[aide] {phase} back-step: {text}"),
        "back-part" => format!("[aide] {phase} back-part: {text}"),
        _ => format!("[aide] {phase}: {text}"),
    }
}
