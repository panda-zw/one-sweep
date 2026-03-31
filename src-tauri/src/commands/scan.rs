use tauri::{AppHandle, State};

use crate::classifier;
use crate::db::scan_cache_repo;
use crate::models::scan::ScanResult;
use crate::scanner;
use crate::AppState;

#[tauri::command]
pub async fn start_scan(
    app_handle: AppHandle,
    state: State<'_, AppState>,
) -> Result<ScanResult, String> {
    let mut result = scanner::run_scan(app_handle)
        .await
        .map_err(|e| e.to_string())?;

    // Classify all items
    for category in &mut result.categories {
        category.items = classifier::classify_all(std::mem::take(&mut category.items));
    }

    // Cache results to DB - lock only for the DB write
    let all_items: Vec<_> = result
        .categories
        .iter()
        .flat_map(|c| c.items.clone())
        .collect();

    {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        scan_cache_repo::upsert_items(&db, &all_items, result.started_at)
            .map_err(|e| e.to_string())?;
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_cached_scan(
    state: State<'_, AppState>,
) -> Result<Option<ScanResult>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    scan_cache_repo::get_cached_scan(&db).map_err(|e| e.to_string())
}
