use anyhow::Result;
use std::process::Command;

/// Clean up Homebrew cache.
pub async fn cleanup() -> Result<()> {
    let output = tokio::task::spawn_blocking(|| {
        Command::new("brew")
            .args(["cleanup", "--prune=all"])
            .output()
    })
    .await??;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("brew cleanup failed: {}", stderr);
    }
    Ok(())
}
