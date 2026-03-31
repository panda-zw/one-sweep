use anyhow::Result;
use std::process::Command;

/// Delete all Xcode simulator devices.
pub async fn delete_simulators() -> Result<()> {
    let output = tokio::task::spawn_blocking(|| {
        Command::new("xcrun")
            .args(["simctl", "delete", "all"])
            .output()
    })
    .await??;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("xcrun simctl delete all failed: {}", stderr);
    }
    Ok(())
}
