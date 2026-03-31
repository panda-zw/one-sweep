use anyhow::Result;

/// Delete a file or directory at the given path.
pub async fn delete_path(path: &str) -> Result<()> {
    let path = std::path::PathBuf::from(path);
    if !path.exists() {
        return Ok(()); // Already gone
    }
    if path.is_dir() {
        tokio::fs::remove_dir_all(&path).await?;
    } else {
        tokio::fs::remove_file(&path).await?;
    }
    Ok(())
}
