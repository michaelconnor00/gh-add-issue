use assert_cmd::Command;

/// Runs the binary as `gh` would — expects exit 0 when the user is
/// already authenticated in the environment running the test suite.
#[test]
fn exits_zero_when_authenticated() {
    Command::cargo_bin("gh-add-issue")
        .unwrap()
        .assert()
        .success();
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
