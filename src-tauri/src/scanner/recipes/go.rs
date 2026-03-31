use anyhow::Result;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

use super::{calculate_dir_size_async, get_last_modified, hash_id};

/// Scan for Go build cache and module cache.
pub async fn scan_go_cache() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let mut items = Vec::new();

    let cache_locations = [
        (home.join("Library/Caches/go-build"), "Go build cache", "Compiled Go packages - rebuilt automatically"),
        (home.join("go/pkg/mod"), "Go module cache", "Downloaded Go modules - re-downloaded with go mod download"),
        (home.join(".cache/go-build"), "Go build cache (alt)", "Compiled Go packages - rebuilt automatically"),
    ];

    for (path, name, description) in cache_locations {
        if !path.exists() {
            continue;
        }
        let size = calculate_dir_size_async(&path).await?;
        if size == 0 {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        items.push(ScanItem {
            id: hash_id(&path_str, "go_cache"),
            path: path_str,
            display_name: name.to_string(),
            description: description.to_string(),
            size_bytes: size,
            safety: SafetyLevel::Green,
            category: ScanCategory::GoCache,
            last_modified: get_last_modified(&path),
        });
    }

    Ok(items)
}
