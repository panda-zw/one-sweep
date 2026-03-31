use anyhow::Result;
use walkdir::WalkDir;

use crate::models::safety::SafetyLevel;
use crate::models::scan::{ScanCategory, ScanItem};

use super::{calculate_dir_size_async, get_last_modified, hash_id, is_hidden, project_search_roots};

/// Scan for Python caches: pip cache, conda pkgs.
pub async fn scan_python_caches() -> Result<Vec<ScanItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
    let mut items = Vec::new();

    let cache_locations = [
        (home.join("Library/Caches/pip"), "pip download cache", "pip package downloads - re-downloaded with pip install"),
        (home.join(".cache/pip"), "pip cache (alt)", "pip package downloads - re-downloaded with pip install"),
        (home.join(".conda/pkgs"), "Conda package cache", "Downloaded conda packages - re-downloaded when needed"),
        (home.join("miniconda3/pkgs"), "Miniconda package cache", "Downloaded conda packages - re-downloaded when needed"),
        (home.join("anaconda3/pkgs"), "Anaconda package cache", "Downloaded conda packages - re-downloaded when needed"),
        (home.join(".mypy_cache"), "mypy type cache", "Type checking cache - regenerated on next run"),
        (home.join(".ruff_cache"), "Ruff linter cache", "Linter cache - regenerated on next run"),
    ];

    for (path, name, description) in cache_locations {
        if !path.exists() {
            continue;
        }
        let size = calculate_dir_size_async(&path).await?;
        if size < 10_000_000 {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        items.push(ScanItem {
            id: hash_id(&path_str, "python_caches"),
            path: path_str,
            display_name: name.to_string(),
            description: description.to_string(),
            size_bytes: size,
            safety: SafetyLevel::Green,
            category: ScanCategory::PythonCaches,
            last_modified: get_last_modified(&path),
        });
    }

    Ok(items)
}

/// Scan for Python virtual environments (.venv, venv, .tox) in projects.
pub async fn scan_python_venvs() -> Result<Vec<ScanItem>> {
    let search_roots = project_search_roots();
    let mut items = Vec::new();

    let venv_names = ["venv", ".venv", ".tox", "env", ".env"];

    for root in search_roots {
        let root_clone = root.clone();
        let venv_names_clone = venv_names;
        let found: Vec<_> = tokio::task::spawn_blocking(move || {
            let mut paths = Vec::new();
            for entry in WalkDir::new(&root_clone)
                .max_depth(3)
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
                let name = entry.file_name().to_string_lossy().to_string();
                if entry.file_type().is_dir() && venv_names_clone.contains(&name.as_str()) {
                    // Verify it's actually a venv by checking for pyvenv.cfg or bin/python
                    let is_venv = entry.path().join("pyvenv.cfg").exists()
                        || entry.path().join("bin/python").exists()
                        || entry.path().join("Scripts/python.exe").exists();
                    if is_venv {
                        let project_name = entry
                            .path()
                            .parent()
                            .and_then(|p| p.file_name())
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        paths.push((entry.path().to_path_buf(), project_name, name));
                    }
                }
            }
            paths
        })
        .await?;

        for (path, project_name, venv_name) in found {
            let size = calculate_dir_size_async(&path).await?;
            if size == 0 {
                continue;
            }
            let path_str = path.to_string_lossy().to_string();
            items.push(ScanItem {
                id: hash_id(&path_str, "python_venvs"),
                path: path_str,
                display_name: format!("{}/{}", project_name, venv_name),
                description: "Python virtual environment - recreated with pip install".to_string(),
                size_bytes: size,
                safety: SafetyLevel::Green,
                category: ScanCategory::PythonVenvs,
                last_modified: get_last_modified(&path),
            });
        }
    }

    Ok(items)
}
