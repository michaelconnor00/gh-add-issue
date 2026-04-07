use std::process::{Command, Stdio};

/// Inner implementation — accepts the command and args so tests can
/// substitute a known-good or known-bad command instead of calling `gh`.
pub(crate) fn check_auth_with_command(cmd: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(cmd)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| format!("Failed to run `{cmd}`: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err("Not authenticated with GitHub. Run `gh auth login` and try again.".to_string())
    }
}

/// Verify that the user has an active, authenticated `gh` session.
///
/// Runs `gh auth status` and checks the exit code.
/// Returns `Ok(())` when authenticated, or an `Err` with a user-facing
/// message when not.
pub fn check_auth() -> Result<(), String> {
    check_auth_with_command("gh", &["auth", "status"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_ok_when_command_succeeds() {
        // `true` always exits 0 — simulates an authenticated session.
        assert!(check_auth_with_command("true", &[]).is_ok());
    }

    #[test]
    fn returns_err_when_command_fails() {
        // `false` always exits 1 — simulates an unauthenticated state.
        let err = check_auth_with_command("false", &[]).unwrap_err();
        assert!(
            err.contains("gh auth login"),
            "unexpected error message: {err}"
        );
    }

    #[test]
    fn returns_err_when_command_not_found() {
        let err = check_auth_with_command("this-binary-does-not-exist", &[]).unwrap_err();
        assert!(
            err.contains("Failed to run"),
            "unexpected error message: {err}"
        );
    }
}
