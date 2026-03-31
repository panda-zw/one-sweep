use anyhow::Result;
use walkdir::WalkDir;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

use super::{calculate_dir_size_async, get_last_modified, hash_id, is_hidden, project_search_roots};

/// Scan for Rust target/ directories in projects.
pub async fn scan_rust_targets() -> Result<Vec<ScanItem>> {
    let search_roots = project_search_roots();
    let mut items = Vec::new();

    for root in search_roots {
        let root_clone = root.clone();
        let found: Vec<_> = tokio::task::spawn_blocking(move || {
            let mut paths = Vec::new();
            for entry in WalkDir::new(&root_clone)
                .max_depth(4)
                .into_iter()
                .filter_entry(|e| {
                    let name = e.file_name().to_string_lossy();
                    !is_hidden(e) && name != ".git" && name != "node_modules" && name != "target"
                })
            {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };
                if entry.file_name() == "target" && entry.file_type().is_dir() {
                    if let Some(parent) = entry.path().parent() {
                        if parent.join("Cargo.toml").exists() {
                            let project_name = parent
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            paths.push((entry.path().to_path_buf(), project_name));
                        }
                    }
                }
            }
            paths
        })
        .await?;

        for (path, project_name) in found {
            let size = calculate_dir_size_async(&path).await?;
            if size == 0 {
                continue;
            }
            let path_str = path.to_string_lossy().to_string();
            items.push(ScanItem {
                id: hash_id(&path_str, "rust_targets"),
                path: path_str,
                display_name: format!("{}/target", project_name),
                description: "Rust compiled artifacts - rebuilt with cargo build".to_string(),
                size_bytes: size,
                safety: SafetyLevel::Green,
                category: ScanCategory::RustTargets,
                last_modified: get_last_modified(&path),
            });
        }
    }

    Ok(items)
}

/// Scan for Cargo registry and cache.
pub async fn scan_cargo_caches() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let mut items = Vec::new();

    let cache_locations = [
        (home.join(".cargo/registry/cache"), "Cargo registry cache", "Downloaded crate archives - re-downloaded when needed"),
        (home.join(".cargo/registry/src"), "Cargo registry sources", "Extracted crate source code - re-downloaded when needed"),
        (home.join(".cargo/git/db"), "Cargo git checkouts", "Git dependency checkouts - re-cloned when needed"),
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
            id: hash_id(&path_str, "cargo_caches"),
            path: path_str,
            display_name: name.to_string(),
            description: description.to_string(),
            size_bytes: size,
            safety: SafetyLevel::Green,
            category: ScanCategory::CargoCaches,
            last_modified: get_last_modified(&path),
        });
    }

    Ok(items)
}
