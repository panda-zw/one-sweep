use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupRequest {
    pub item_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupStatus {
    Pending,
    InProgress,
    Completed,
    Failed { error: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupProgress {
    pub total_items: usize,
    pub completed_items: usize,
    pub current_item: Option<String>,
    pub bytes_freed: u64,
    pub status: CleanupStatus,
    pub results: Vec<CleanupItemResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupItemResult {
    pub item_id: String,
    pub path: String,
    pub size_bytes: u64,
    pub success: bool,
    pub error: Option<String>,
}
