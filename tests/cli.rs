use assert_cmd::Command;

/// In a non-TTY test environment the binary passes auth and then fails
/// cleanly at the interactive selector with a message directing the user
/// to use --repo instead. We verify: exit is non-zero AND the error is NOT
/// the auth message — confirming auth succeeded and the process progressed
/// to the selection stage.
#[test]
fn auth_passes_and_selector_fails_gracefully_without_tty() {
    Command::cargo_bin("gh-add-issue")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicates::str::contains("requires a terminal"));
}

/// When GH_TOKEN is set to a deliberately invalid value, `gh auth status`
/// should fail and the binary must exit non-zero with a helpful message on
/// stderr.
#[test]
fn exits_nonzero_when_not_authenticated() {
    Command::cargo_bin("gh-add-issue")
        .unwrap()
        .env("GH_TOKEN", "invalid-token-for-testing")
        .env("GH_CONFIG_DIR", "/dev/null") // isolate from real config
        .assert()
        .failure()
        .stderr(predicates::str::contains("gh auth login"));
}
