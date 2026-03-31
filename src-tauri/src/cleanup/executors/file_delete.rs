use anyhow::Result;
use std::path::{Path, PathBuf};

/// Known safe path patterns relative to the home directory.
/// Only paths matching these patterns can be deleted.
const SAFE_PATTERNS: &[&str] = &[
    "Library/Caches",
    "Library/Developer/CoreSimulator",
    "Library/Developer/Xcode/DerivedData",
    "Library/pnpm",
    ".npm",
    ".yarn",
    ".pnpm-store",
    ".gradle",
    ".expo",
];

/// Check if a path component is `node_modules`.
fn contains_node_modules(path: &Path) -> bool {
    path.components()
        .any(|c| c.as_os_str() == "node_modules")
}

/// Validate that a path is safe to delete:
/// - Must be under the user's home directory
/// - Must match a known safe pattern or contain node_modules
/// - Resolves symlinks to prevent symlink attacks
fn validate_path(path: &Path) -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?;

    // Resolve symlinks and normalize the path
    let canonical = path.canonicalize()
        .map_err(|e| anyhow::anyhow!("Cannot resolve path {}: {}", path.display(), e))?;

    // Must be under the home directory
    if !canonical.starts_with(&home) {
        anyhow::bail!(
            "Refusing to delete path outside home directory: {}",
            canonical.display()
        );
    }

    // Must not be the home directory itself
    if canonical == home {
        anyhow::bail!("Refusing to delete the home directory");
    }

    // Check against known safe patterns
    let relative = canonical.strip_prefix(&home)
        .map_err(|_| anyhow::anyhow!("Path is not relative to home"))?;

    let is_safe = SAFE_PATTERNS
        .iter()
        .any(|pattern| relative.starts_with(pattern))
        || contains_node_modules(&canonical);

    if !is_safe {
        anyhow::bail!(
            "Path does not match any known safe deletion pattern: {}",
            canonical.display()
        );
    }

    Ok(canonical)
}

/// Delete a file or directory at the given path after validation.
pub async fn delete_path(path: &str) -> Result<()> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Ok(()); // Already gone
    }

    let validated = validate_path(&path)?;

    if validated.is_dir() {
        tokio::fs::remove_dir_all(&validated).await?;
    } else {
        tokio::fs::remove_file(&validated).await?;
    }
    Ok(())
}
