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

    let rel = p.strip_prefix(&home).unwrap_or(p);

    match category {
        ScanCategory::NodeDependencies => {
            p.components().any(|c| c.as_os_str() == "node_modules")
        }
        ScanCategory::NodeCaches => {
            rel.starts_with(".npm")
                || rel.starts_with(".yarn")
                || rel.starts_with(".pnpm-store")
                || rel.starts_with("Library/Caches/Yarn")
                || rel.starts_with("Library/pnpm")
        }
        ScanCategory::XcodeDerivedData => {
            rel.starts_with("Library/Developer/Xcode/DerivedData")
        }
        ScanCategory::GradleCache => {
            rel.starts_with(".gradle")
        }
        ScanCategory::SystemCaches => {
            rel.starts_with("Library/Caches")
        }
        ScanCategory::RustTargets => {
            p.components().any(|c| c.as_os_str() == "target")
        }
        ScanCategory::CargoCaches => {
            rel.starts_with(".cargo/registry") || rel.starts_with(".cargo/git")
        }
        ScanCategory::PythonCaches => {
            rel.starts_with("Library/Caches/pip")
                || rel.starts_with(".cache/pip")
                || rel.starts_with(".conda/pkgs")
                || rel.starts_with("miniconda3/pkgs")
                || rel.starts_with("anaconda3/pkgs")
                || rel.starts_with(".mypy_cache")
                || rel.starts_with(".ruff_cache")
        }
        ScanCategory::PythonVenvs => {
            let name = p.file_name().unwrap_or_default().to_string_lossy();
            name == "venv" || name == ".venv" || name == ".tox" || name == "env"
        }
        ScanCategory::GoCache => {
            rel.starts_with("go/pkg")
                || rel.starts_with("Library/Caches/go-build")
                || rel.starts_with(".cache/go-build")
        }
        ScanCategory::MavenCache => {
            rel.starts_with(".m2/repository")
        }
        ScanCategory::RubyCache => {
            rel.starts_with(".gem")
                || rel.starts_with(".bundle/cache")
                || rel.starts_with("Library/Caches/com.apple.rubygems")
        }
        ScanCategory::DotnetCache => {
            rel.starts_with(".nuget")
                || rel.starts_with(".dotnet")
                || rel.starts_with(".local/share/NuGet")
        }
        ScanCategory::FlutterCache => {
            rel.starts_with(".pub-cache")
                || rel.starts_with("Library/Caches/flutter")
                || rel.starts_with(".dartServer")
        }
        ScanCategory::CocoaPodsCache => {
            rel.starts_with("Library/Caches/CocoaPods")
                || rel.starts_with(".cocoapods")
        }
        ScanCategory::ComposerCache => {
            rel.starts_with(".composer/cache")
                || rel.starts_with("Library/Caches/composer")
        }
        ScanCategory::IdeCaches => {
            rel.starts_with("Library/Application Support/Code/")
                || rel.starts_with("Library/Application Support/Cursor/")
                || rel.starts_with("Library/Caches/JetBrains")
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
