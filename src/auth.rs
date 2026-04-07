use std::process::Command;

/// Verify that the user has an active, authenticated `gh` session.
///
/// Runs `gh auth status` and checks the exit code.
/// Returns `Ok(())` when authenticated, or an `Err` with a user-facing
/// message when not.
pub fn check_auth() -> Result<(), String> {
    let status = Command::new("gh")
        .args(["auth", "status"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("Failed to run `gh auth status`: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(
            "Not authenticated with GitHub. Run `gh auth login` and try again.".to_string(),
        )
    }
}
