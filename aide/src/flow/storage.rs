use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use crate::flow::types::{BackConfirmState, FlowStatus};
use crate::utils::{now_iso, now_task_id};

pub struct FlowStorage {
    #[allow(dead_code)]
    pub root: PathBuf,
    pub aide_dir: PathBuf,
    pub status_path: PathBuf,
    pub lock_path: PathBuf,
    pub tmp_path: PathBuf,
    pub logs_dir: PathBuf,
    pub back_confirm_path: PathBuf,
}

impl FlowStorage {
    pub fn new(root: &Path) -> Self {
        let aide_dir = root.join(crate::core::config::AIDE_MEMORY_DIR);
        Self {
            root: root.to_path_buf(),
            status_path: aide_dir.join("flow-status.json"),
            lock_path: aide_dir.join("flow-status.lock"),
            tmp_path: aide_dir.join("flow-status.json.tmp"),
            logs_dir: aide_dir.join("logs"),
            back_confirm_path: aide_dir.join("back-confirm-state.json"),
            aide_dir,
        }
    }

    pub fn ensure_ready(&self) -> Result<(), String> {
        if !self.aide_dir.exists() {
            return Err("未找到 aide-memory 目录，请先运行：aide init".into());
        }
        let _ = fs::create_dir_all(&self.logs_dir);
        Ok(())
    }

    pub fn acquire_lock(&self) -> Result<LockGuard, String> {
        let timeout = Duration::from_secs(3);
        let poll = Duration::from_millis(200);
        let start = Instant::now();

        loop {
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&self.lock_path)
            {
                Ok(_file) => {
                    let _ = fs::write(&self.lock_path, std::process::id().to_string());
                    return Ok(LockGuard {
                        path: self.lock_path.clone(),
                    });
                }
                Err(_) => {
                    if start.elapsed() >= timeout {
                        return Err(
                            "状态文件被占用，请稍后重试或删除 aide-memory/flow-status.lock".into(),
                        );
                    }
                    thread::sleep(poll);
                }
            }
        }
    }

    pub fn load_status(&self) -> Result<Option<FlowStatus>, String> {
        if !self.status_path.exists() {
            return Ok(None);
        }
        let raw = fs::read_to_string(&self.status_path)
            .map_err(|e| format!("状态文件读取失败: {e}"))?;
        let status: FlowStatus =
            serde_json::from_str(&raw).map_err(|e| format!("状态文件解析失败: {e}"))?;
        Ok(Some(status))
    }

    pub fn save_status(&self, status: &FlowStatus) -> Result<(), String> {
        let payload =
            serde_json::to_string_pretty(status).map_err(|e| format!("序列化状态失败: {e}"))?;
        fs::write(&self.tmp_path, format!("{payload}\n"))
            .map_err(|e| format!("写入状态文件失败: {e}"))?;
        fs::rename(&self.tmp_path, &self.status_path)
            .map_err(|e| format!("重命名状态文件失败: {e}"))?;
        Ok(())
    }

    pub fn archive_existing_status(&self) -> Result<(), String> {
        if !self.status_path.exists() {
            return Ok(());
        }
        let suffix = match self.load_status() {
            Ok(Some(s)) => s.task_id,
            _ => now_task_id(),
        };
        let target = self.logs_dir.join(format!("flow-status.{suffix}.json"));
        fs::rename(&self.status_path, &target)
            .map_err(|e| format!("归档旧状态失败: {e}"))?;
        Ok(())
    }

    pub fn list_all_tasks(&self) -> Result<Vec<TaskSummary>, String> {
        let mut tasks = Vec::new();

        // 当前任务
        if let Some(current) = self.load_status()? {
            let summary = current
                .history
                .first()
                .map(|h| h.summary.clone())
                .unwrap_or_default();
            tasks.push(TaskSummary {
                task_id: current.task_id,
                phase: current.current_phase,
                started_at: current.started_at,
                summary,
                is_current: true,
            });
        }

        // 归档任务
        if self.logs_dir.exists() {
            if let Ok(entries) = fs::read_dir(&self.logs_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if name.starts_with("flow-status.") && name.ends_with(".json") {
                        if let Ok(raw) = fs::read_to_string(&path) {
                            if let Ok(status) = serde_json::from_str::<FlowStatus>(&raw) {
                                let summary = status
                                    .history
                                    .first()
                                    .map(|h| h.summary.clone())
                                    .unwrap_or_default();
                                tasks.push(TaskSummary {
                                    task_id: status.task_id,
                                    phase: status.current_phase,
                                    started_at: status.started_at,
                                    summary,
                                    is_current: false,
                                });
                            }
                        }
                    }
                }
            }
        }

        // 按 task_id 倒序
        tasks.sort_by(|a, b| b.task_id.cmp(&a.task_id));
        Ok(tasks)
    }

    pub fn load_task_by_id(&self, task_id: &str) -> Result<Option<FlowStatus>, String> {
        // 先检查当前任务
        if let Some(current) = self.load_status()? {
            if current.task_id == task_id {
                return Ok(Some(current));
            }
        }

        // 检查归档
        let archive_path = self.logs_dir.join(format!("flow-status.{task_id}.json"));
        if archive_path.exists() {
            let raw = fs::read_to_string(&archive_path)
                .map_err(|e| format!("读取归档任务失败: {e}"))?;
            let status: FlowStatus =
                serde_json::from_str(&raw).map_err(|e| format!("读取归档任务失败: {e}"))?;
            return Ok(Some(status));
        }

        Ok(None)
    }

    // === Back-confirm 状态管理 ===

    pub fn has_pending_back_confirm(&self) -> bool {
        self.back_confirm_path.exists()
    }

    pub fn load_back_confirm_state(&self) -> Result<Option<BackConfirmState>, String> {
        if !self.back_confirm_path.exists() {
            return Ok(None);
        }
        let raw = fs::read_to_string(&self.back_confirm_path)
            .map_err(|e| format!("读取 back-confirm 状态失败: {e}"))?;
        let state: BackConfirmState =
            serde_json::from_str(&raw).map_err(|e| format!("读取 back-confirm 状态失败: {e}"))?;
        Ok(Some(state))
    }

    pub fn save_back_confirm_state(
        &self,
        target_part: &str,
        reason: &str,
    ) -> Result<String, String> {
        let key = format!("{:012x}", rand::random::<u64>() & 0xFFFF_FFFF_FFFF);
        let state = BackConfirmState {
            pending_key: key.clone(),
            target_part: target_part.into(),
            reason: reason.into(),
            created_at: now_iso(),
        };
        let payload = serde_json::to_string_pretty(&state)
            .map_err(|e| format!("序列化 back-confirm 状态失败: {e}"))?;
        fs::write(&self.back_confirm_path, format!("{payload}\n"))
            .map_err(|e| format!("保存 back-confirm 状态失败: {e}"))?;
        Ok(key)
    }

    pub fn clear_back_confirm_state(&self) -> Result<(), String> {
        if self.back_confirm_path.exists() {
            let _ = fs::remove_file(&self.back_confirm_path);
        }
        Ok(())
    }
}

pub struct LockGuard {
    path: PathBuf,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[derive(Debug)]
pub struct TaskSummary {
    pub task_id: String,
    pub phase: String,
    #[allow(dead_code)]
    pub started_at: String,
    pub summary: String,
    pub is_current: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flow::types::HistoryEntry;
    use tempfile::TempDir;

    fn make_status(task_id: &str, phase: &str) -> FlowStatus {
        FlowStatus {
            task_id: task_id.into(),
            current_phase: phase.into(),
            current_step: 1,
            started_at: "2024-01-01T00:00:00+08:00".into(),
            history: vec![HistoryEntry {
                timestamp: "2024-01-01T00:00:00+08:00".into(),
                action: "start".into(),
                phase: phase.into(),
                step: 1,
                summary: "测试任务".into(),
                git_commit: None,
            }],
            source_branch: None,
            start_commit: None,
            task_branch: None,
        }
    }

    fn setup_storage() -> (TempDir, FlowStorage) {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(crate::core::config::AIDE_MEMORY_DIR).join("logs")).unwrap();
        let storage = FlowStorage::new(tmp.path());
        (tmp, storage)
    }

    // === ensure_ready 测试 ===

    #[test]
    fn test_ensure_ready_no_aide_dir() {
        let tmp = TempDir::new().unwrap();
        let storage = FlowStorage::new(tmp.path());
        assert!(storage.ensure_ready().is_err());
    }

    #[test]
    fn test_ensure_ready_ok() {
        let (_tmp, storage) = setup_storage();
        assert!(storage.ensure_ready().is_ok());
    }

    // === load/save status 测试 ===

    #[test]
    fn test_load_status_empty() {
        let (_tmp, storage) = setup_storage();
        let result = storage.load_status().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_save_and_load_status() {
        let (_tmp, storage) = setup_storage();
        let status = make_status("20240101T00-00-00", "impl");
        storage.save_status(&status).unwrap();
        let loaded = storage.load_status().unwrap().unwrap();
        assert_eq!(loaded.task_id, "20240101T00-00-00");
        assert_eq!(loaded.current_phase, "impl");
        assert_eq!(loaded.current_step, 1);
        assert_eq!(loaded.history.len(), 1);
    }

    // === archive 测试 ===

    #[test]
    fn test_archive_no_status() {
        let (_tmp, storage) = setup_storage();
        assert!(storage.archive_existing_status().is_ok());
    }

    #[test]
    fn test_archive_existing_status() {
        let (_tmp, storage) = setup_storage();
        let status = make_status("20240101T00-00-00", "impl");
        storage.save_status(&status).unwrap();
        assert!(storage.status_path.exists());
        storage.archive_existing_status().unwrap();
        assert!(!storage.status_path.exists());
        let archived = storage
            .logs_dir
            .join("flow-status.20240101T00-00-00.json");
        assert!(archived.exists());
    }

    // === list_all_tasks 测试 ===

    #[test]
    fn test_list_all_tasks_empty() {
        let (_tmp, storage) = setup_storage();
        let tasks = storage.list_all_tasks().unwrap();
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_list_all_tasks_with_current_and_archived() {
        let (_tmp, storage) = setup_storage();

        // 当前任务
        let status = make_status("20240102T00-00-00", "verify");
        storage.save_status(&status).unwrap();

        // 归档任务
        let archived_status = make_status("20240101T00-00-00", "finish");
        let archived_json = serde_json::to_string_pretty(&archived_status).unwrap();
        std::fs::write(
            storage
                .logs_dir
                .join("flow-status.20240101T00-00-00.json"),
            archived_json,
        )
        .unwrap();

        let tasks = storage.list_all_tasks().unwrap();
        assert_eq!(tasks.len(), 2);
        // 倒序，最新的在前
        assert!(tasks[0].is_current);
        assert!(!tasks[1].is_current);
    }

    // === load_task_by_id 测试 ===

    #[test]
    fn test_load_task_by_id_current() {
        let (_tmp, storage) = setup_storage();
        let status = make_status("20240101T00-00-00", "impl");
        storage.save_status(&status).unwrap();
        let result = storage.load_task_by_id("20240101T00-00-00").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().task_id, "20240101T00-00-00");
    }

    #[test]
    fn test_load_task_by_id_archived() {
        let (_tmp, storage) = setup_storage();
        let status = make_status("20240101T00-00-00", "finish");
        let json = serde_json::to_string_pretty(&status).unwrap();
        std::fs::write(
            storage
                .logs_dir
                .join("flow-status.20240101T00-00-00.json"),
            json,
        )
        .unwrap();
        let result = storage.load_task_by_id("20240101T00-00-00").unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_load_task_by_id_not_found() {
        let (_tmp, storage) = setup_storage();
        let result = storage.load_task_by_id("nonexistent").unwrap();
        assert!(result.is_none());
    }

    // === back-confirm 状态测试 ===

    #[test]
    fn test_back_confirm_lifecycle() {
        let (_tmp, storage) = setup_storage();

        // 初始无状态
        assert!(!storage.has_pending_back_confirm());
        assert!(storage.load_back_confirm_state().unwrap().is_none());

        // 保存状态
        let key = storage
            .save_back_confirm_state("task-optimize", "需要重新设计")
            .unwrap();
        assert!(!key.is_empty());
        assert!(storage.has_pending_back_confirm());

        // 加载状态
        let state = storage.load_back_confirm_state().unwrap().unwrap();
        assert_eq!(state.pending_key, key);
        assert_eq!(state.target_part, "task-optimize");
        assert_eq!(state.reason, "需要重新设计");

        // 清除
        storage.clear_back_confirm_state().unwrap();
        assert!(!storage.has_pending_back_confirm());
    }

    // === 文件锁测试 ===

    #[test]
    fn test_acquire_lock_success() {
        let (_tmp, storage) = setup_storage();
        let guard = storage.acquire_lock();
        assert!(guard.is_ok());
        assert!(storage.lock_path.exists());
        // 释放锁
        drop(guard);
        assert!(!storage.lock_path.exists());
    }
}
