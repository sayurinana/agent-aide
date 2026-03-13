use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: String,
    pub action: String,
    pub phase: String,
    pub step: i64,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowStatus {
    pub task_id: String,
    pub current_phase: String,
    pub current_step: i64,
    pub started_at: String,
    pub history: Vec<HistoryEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackConfirmState {
    pub pending_key: String,
    pub target_part: String,
    pub reason: String,
    pub created_at: String,
}
