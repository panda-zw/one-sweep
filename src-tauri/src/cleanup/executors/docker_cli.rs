use anyhow::Result;
use std::process::Command;

/// Prune unused Docker images.
pub async fn prune_images() -> Result<()> {
    let output = tokio::task::spawn_blocking(|| {
        Command::new("docker")
            .args(["system", "prune", "-a", "-f"])
            .output()
    })
    .await??;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("docker system prune failed: {}", stderr);
    }
    Ok(())
}

/// Prune Docker build cache.
pub async fn prune_build_cache() -> Result<()> {
    let output = tokio::task::spawn_blocking(|| {
        Command::new("docker")
            .args(["builder", "prune", "-a", "-f"])
            .output()
    })
    .await??;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("docker builder prune failed: {}", stderr);
    }
    Ok(())
}
