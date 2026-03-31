use anyhow::Result;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

use super::{calculate_dir_size_async, get_last_modified, hash_id};

/// Scan for Maven local repository.
pub async fn scan_maven_cache() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let mut items = Vec::new();

    let path = home.join(".m2/repository");
    if !path.exists() {
        return Ok(items);
    }

    let size = calculate_dir_size_async(&path).await?;
    if size == 0 {
        return Ok(items);
    }

    let path_str = path.to_string_lossy().to_string();
    items.push(ScanItem {
        id: hash_id(&path_str, "maven_cache"),
        path: path_str,
        display_name: "Maven local repository".to_string(),
        description: "Downloaded Java/Kotlin dependencies - re-downloaded on next build".to_string(),
        size_bytes: size,
        safety: SafetyLevel::Green,
        category: ScanCategory::MavenCache,
        last_modified: get_last_modified(&path),
    });

    Ok(items)
}
