use tauri::State;

use crate::db::audit_repo;
use crate::models::audit::AuditEntry;
use crate::AppState;

#[tauri::command]
pub fn get_audit_log(
    state: State<'_, AppState>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<AuditEntry>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    audit_repo::list_entries(&db, limit.unwrap_or(100), offset.unwrap_or(0))
        .map_err(|e| e.to_string())
}
