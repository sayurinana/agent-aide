use std::fs;
use std::path::{Path, PathBuf};

use crate::decide::types::*;
use crate::utils::now_task_id;
use crate::utils::now_iso;

pub struct DecideStorage {
    #[allow(dead_code)]
    pub root: PathBuf,
    pub aide_dir: PathBuf,
    pub decisions_dir: PathBuf,
    pub pending_path: PathBuf,
}

impl DecideStorage {
    pub fn new(root: &Path) -> Self {
        let aide_dir = root.join(crate::core::config::AIDE_MEMORY_DIR);
        let decisions_dir = aide_dir.join("decisions");
        let pending_path = decisions_dir.join("pending.json");
        Self {
            root: root.to_path_buf(),
            aide_dir,
            decisions_dir,
            pending_path,
        }
    }

    pub fn ensure_ready(&self) -> Result<(), String> {
        if !self.aide_dir.exists() {
            return Err("aide-memory 目录不存在，请先执行 aide init".into());
        }
        fs::create_dir_all(&self.decisions_dir)
            .map_err(|e| format!("创建 decisions 目录失败: {e}"))?;
        Ok(())
    }

    pub fn save_pending(&self, data: &DecideInput) -> Result<String, String> {
        self.ensure_ready()?;
        let session_id = now_task_id();
        let created_at = now_iso();

        let mut payload = data.clone();
        payload.meta = Some(MetaInfo {
            created_at,
            session_id: session_id.clone(),
        });

        self.save_atomic(&self.pending_path, &payload)?;
        Ok(session_id)
    }

    pub fn load_pending(&self) -> Result<Option<DecideInput>, String> {
        self.ensure_ready()?;
        if !self.pending_path.exists() {
            return Ok(None);
        }
        let data = self.load_json::<DecideInput>(&self.pending_path)?;
        Ok(Some(data))
    }

    pub fn get_session_id(&self) -> Result<Option<String>, String> {
        let pending = self.load_pending()?;
        match pending {
            None => Ok(None),
            Some(p) => match &p.meta {
                Some(meta) => Ok(Some(meta.session_id.clone())),
                None => Err("pending.json 缺少 _meta.session_id".into()),
            },
        }
    }

    pub fn save_result(&self, output: &DecideOutput) -> Result<(), String> {
        let pending = self.load_pending()?.ok_or("未找到待定项数据")?;
        let meta = pending
            .meta
            .as_ref()
            .ok_or("pending.json 缺少 _meta.session_id")?;

        let mut input_no_meta = pending.clone();
        input_no_meta.meta = None;

        let record = DecisionRecord {
            input: input_no_meta,
            output: output.clone(),
            completed_at: now_iso(),
        };

        let target = self.decisions_dir.join(format!("{}.json", meta.session_id));
        self.save_atomic(&target, &record)?;
        Ok(())
    }

    pub fn load_result(&self) -> Result<Option<DecideOutput>, String> {
        let session_id = match self.get_session_id()? {
            Some(id) => id,
            None => return Ok(None),
        };
        let record_path = self.decisions_dir.join(format!("{session_id}.json"));
        if !record_path.exists() {
            return Ok(None);
        }
        let record = self.load_json::<DecisionRecord>(&record_path)?;
        Ok(Some(record.output))
    }

    pub fn server_info_path(&self) -> PathBuf {
        self.decisions_dir.join("server.json")
    }

    pub fn save_server_info(&self, pid: u32, port: u16, url: &str) -> Result<(), String> {
        self.ensure_ready()?;
        let info = ServerInfo {
            pid,
            port,
            url: url.to_string(),
            started_at: now_iso(),
        };
        self.save_atomic(&self.server_info_path(), &info)?;
        Ok(())
    }

    pub fn load_server_info(&self) -> Option<ServerInfo> {
        let path = self.server_info_path();
        if !path.exists() {
            return None;
        }
        self.load_json::<ServerInfo>(&path).ok()
    }

    pub fn clear_server_info(&self) {
        let _ = fs::remove_file(self.server_info_path());
    }

    pub fn is_server_running(&self) -> bool {
        let info = match self.load_server_info() {
            Some(i) => i,
            None => return false,
        };
        libc_kill(info.pid as i32, 0) == 0
    }

    fn save_atomic<T: serde::Serialize>(&self, path: &Path, data: &T) -> Result<(), String> {
        let payload = serde_json::to_string_pretty(data)
            .map_err(|e| format!("序列化失败: {e}"))?;
        let tmp_path = path.with_extension("json.tmp");
        fs::write(&tmp_path, format!("{payload}\n"))
            .map_err(|e| format!("写入 {} 失败: {e}", path.display()))?;
        fs::rename(&tmp_path, path)
            .map_err(|e| format!("重命名 {} 失败: {e}", path.display()))?;
        Ok(())
    }

    fn load_json<T: serde::de::DeserializeOwned>(&self, path: &Path) -> Result<T, String> {
        let raw = fs::read_to_string(path)
            .map_err(|e| format!("无法读取 {}: {e}", path.display()))?;
        serde_json::from_str(&raw)
            .map_err(|e| format!("无法解析 {}: {e}", path.display()))
    }
}

/// Check if a process exists (portable approach)
fn libc_kill(pid: i32, sig: i32) -> i32 {
    // Use nix or direct syscall
    #[cfg(unix)]
    {
        unsafe { libc::kill(pid, sig) }
    }
    #[cfg(not(unix))]
    {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decide::types::{DecideInput, DecideItem, DecideOption, DecideOutput, Decision};
    use tempfile::TempDir;

    fn make_option(value: &str, label: &str) -> DecideOption {
        DecideOption {
            value: value.into(),
            label: label.into(),
            score: None,
            pros: None,
            cons: None,
        }
    }

    fn make_input() -> DecideInput {
        DecideInput {
            task: "测试任务".into(),
            source: "test.md".into(),
            items: vec![DecideItem {
                id: 1,
                title: "选择框架".into(),
                options: vec![make_option("a", "选项A"), make_option("b", "选项B")],
                location: None,
                context: None,
                recommend: Some("a".into()),
            }],
            meta: None,
        }
    }

    fn setup_storage() -> (TempDir, DecideStorage) {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(crate::core::config::AIDE_MEMORY_DIR).join("decisions")).unwrap();
        let storage = DecideStorage::new(tmp.path());
        (tmp, storage)
    }

    // === ensure_ready 测试 ===

    #[test]
    fn test_ensure_ready_no_aide_dir() {
        let tmp = TempDir::new().unwrap();
        let storage = DecideStorage::new(tmp.path());
        assert!(storage.ensure_ready().is_err());
    }

    #[test]
    fn test_ensure_ready_ok() {
        let (_tmp, storage) = setup_storage();
        assert!(storage.ensure_ready().is_ok());
    }

    // === save/load pending 测试 ===

    #[test]
    fn test_load_pending_empty() {
        let (_tmp, storage) = setup_storage();
        let result = storage.load_pending().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_save_and_load_pending() {
        let (_tmp, storage) = setup_storage();
        let input = make_input();
        let session_id = storage.save_pending(&input).unwrap();
        assert!(!session_id.is_empty());

        let loaded = storage.load_pending().unwrap().unwrap();
        assert_eq!(loaded.task, "测试任务");
        assert_eq!(loaded.source, "test.md");
        assert_eq!(loaded.items.len(), 1);
        assert!(loaded.meta.is_some());
        assert_eq!(loaded.meta.unwrap().session_id, session_id);
    }

    // === get_session_id 测试 ===

    #[test]
    fn test_get_session_id_none() {
        let (_tmp, storage) = setup_storage();
        assert!(storage.get_session_id().unwrap().is_none());
    }

    #[test]
    fn test_get_session_id_present() {
        let (_tmp, storage) = setup_storage();
        let input = make_input();
        let saved_id = storage.save_pending(&input).unwrap();
        let loaded_id = storage.get_session_id().unwrap().unwrap();
        assert_eq!(saved_id, loaded_id);
    }

    // === save/load result 测试 ===

    #[test]
    fn test_save_and_load_result() {
        let (_tmp, storage) = setup_storage();
        let input = make_input();
        storage.save_pending(&input).unwrap();

        let output = DecideOutput {
            decisions: vec![Decision {
                id: 1,
                chosen: "a".into(),
                note: Some("选 A".into()),
            }],
        };
        storage.save_result(&output).unwrap();

        let loaded = storage.load_result().unwrap().unwrap();
        assert_eq!(loaded.decisions.len(), 1);
        assert_eq!(loaded.decisions[0].id, 1);
        assert_eq!(loaded.decisions[0].chosen, "a");
        assert_eq!(loaded.decisions[0].note.as_deref(), Some("选 A"));
    }

    #[test]
    fn test_load_result_none_when_no_result() {
        let (_tmp, storage) = setup_storage();
        let input = make_input();
        storage.save_pending(&input).unwrap();
        let result = storage.load_result().unwrap();
        assert!(result.is_none());
    }

    // === server info 测试 ===

    #[test]
    fn test_server_info_lifecycle() {
        let (_tmp, storage) = setup_storage();

        assert!(storage.load_server_info().is_none());

        storage
            .save_server_info(12345, 3721, "http://127.0.0.1:3721")
            .unwrap();
        let info = storage.load_server_info().unwrap();
        assert_eq!(info.pid, 12345);
        assert_eq!(info.port, 3721);
        assert_eq!(info.url, "http://127.0.0.1:3721");

        storage.clear_server_info();
        assert!(storage.load_server_info().is_none());
    }

    // === atomic save 测试 ===

    #[test]
    fn test_atomic_save_no_tmp_file_left() {
        let (_tmp, storage) = setup_storage();
        let input = make_input();
        storage.save_pending(&input).unwrap();
        // 确保 .tmp 文件不存在
        assert!(!storage.pending_path.with_extension("json.tmp").exists());
        assert!(storage.pending_path.exists());
    }
}
