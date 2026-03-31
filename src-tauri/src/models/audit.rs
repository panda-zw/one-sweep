use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: i64,
    pub item_path: String,
    pub item_display_name: String,
    pub category: String,
    pub size_bytes: u64,
    pub deleted_at: i64,
    pub success: bool,
    pub error_message: Option<String>,
}
