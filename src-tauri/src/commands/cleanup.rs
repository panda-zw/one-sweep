use tauri::{AppHandle, State};

use crate::cleanup;
use crate::db::scan_cache_repo;
use crate::models::cleanup::{CleanupProgress, CleanupRequest};
use crate::models::safety::SafetyLevel;
use crate::AppState;

#[tauri::command]
pub async fn start_cleanup(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    request: CleanupRequest,
) -> Result<CleanupProgress, String> {
    // Resolve items while holding the lock, then release it before async work
    let selected_items = {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        let all_items = scan_cache_repo::get_cached_items(&db).map_err(|e| e.to_string())?;
        all_items
            .into_iter()
            .filter(|i| request.item_ids.contains(&i.id))
            .filter(|i| i.safety == SafetyLevel::Green)
            .collect::<Vec<_>>()
    };

    if selected_items.is_empty() {
        return Err("No eligible items selected for cleanup".to_string());
    }

    let progress = cleanup::execute_cleanup(selected_items, app_handle, state.inner()).await;
    Ok(progress)
}
