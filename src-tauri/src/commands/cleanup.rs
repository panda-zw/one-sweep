use std::path::Path;
use tauri::{AppHandle, State};

use crate::cleanup;
use crate::db::scan_cache_repo;
use crate::models::cleanup::{CleanupProgress, CleanupRequest};
use crate::models::safety::SafetyLevel;
use crate::models::scan::ScanCategory;
use crate::AppState;

/// Validate that a scan item's path is plausible for its category.
/// This re-checks paths from the database before cleanup to guard against tampering.
fn is_path_plausible(path: &str, category: &ScanCategory) -> bool {
    // Docker and Homebrew use CLI commands, not file paths — always plausible
    if matches!(
        category,
        ScanCategory::DockerImages
            | ScanCategory::DockerBuildCache
            | ScanCategory::HomebrewCache
            | ScanCategory::XcodeSimulators
    ) {
        return true;
    }

    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return false,
    };

    let p = Path::new(path);

    // Must be under home directory
    if !p.starts_with(&home) {
        return false;
    }

    match category {
        ScanCategory::NodeDependencies => {
            // Must end with or contain node_modules
            p.components().any(|c| c.as_os_str() == "node_modules")
        }
        ScanCategory::NodeCaches => {
            let rel = p.strip_prefix(&home).unwrap_or(p);
            rel.starts_with(".npm")
                || rel.starts_with(".yarn")
                || rel.starts_with(".pnpm-store")
                || rel.starts_with("Library/Caches/Yarn")
                || rel.starts_with("Library/pnpm")
        }
        ScanCategory::XcodeDerivedData => {
            let rel = p.strip_prefix(&home).unwrap_or(p);
            rel.starts_with("Library/Developer/Xcode/DerivedData")
        }
        ScanCategory::GradleCache => {
            let rel = p.strip_prefix(&home).unwrap_or(p);
            rel.starts_with(".gradle")
        }
        ScanCategory::SystemCaches => {
            let rel = p.strip_prefix(&home).unwrap_or(p);
            rel.starts_with("Library/Caches")
        }
        _ => false,
    }
}

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
            .filter(|i| is_path_plausible(&i.path, &i.category))
            .collect::<Vec<_>>()
    };

    if selected_items.is_empty() {
        return Err("No eligible items selected for cleanup".to_string());
    }

    let progress = cleanup::execute_cleanup(selected_items, app_handle, state.inner()).await;
    Ok(progress)
}
