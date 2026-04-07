use std::process::{Command, Stdio};

/// A repository accessible to the authenticated user, formatted as
/// `owner/repo`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Repo(pub String);

impl Repo {
    #[allow(dead_code)] // used in section 3.2 interactive selector
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Parse newline-separated `owner/repo` output (as produced by
/// `gh api --paginate /user/repos --jq '.[].full_name'`) into a sorted
/// `Vec<Repo>`.
///
/// Blank lines are ignored. Lines that do not contain exactly one `/` are
/// rejected with an error.
pub fn parse_repos(output: &str) -> Result<Vec<Repo>, String> {
    let mut repos: Vec<Repo> = output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let line = line.trim();
            if line.contains('/') && line.len() > 1 {
                Ok(Repo(line.to_string()))
            } else {
                Err(format!("Unexpected repo format: {line:?}"))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    repos.sort();
    Ok(repos)
}

/// Fetch all repositories accessible to the authenticated GitHub user by
/// delegating to `gh api --paginate /user/repos --jq '.[].full_name'`.
///
/// Pagination is handled automatically by the `gh` CLI.
pub fn fetch_repos() -> Result<Vec<Repo>, String> {
    log::debug!("Fetching repository list via `gh api --paginate /user/repos`.");

    let output = Command::new("gh")
        .args(["api", "--paginate", "/user/repos", "--jq", ".[].full_name"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to run `gh api`: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("gh api returned non-zero status: {stderr}");
        return Err(format!("Failed to list repositories: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let repos = parse_repos(&stdout)?;
    log::info!("Fetched {} repositories.", repos.len());
    Ok(repos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_typical_output() {
        let input = "owner-b/repo-z\nowner-a/repo-a\nowner-a/repo-b\n";
        let repos = parse_repos(input).unwrap();
        // Should be sorted
        assert_eq!(repos[0].as_str(), "owner-a/repo-a");
        assert_eq!(repos[1].as_str(), "owner-a/repo-b");
        assert_eq!(repos[2].as_str(), "owner-b/repo-z");
    }

    #[test]
    fn ignores_blank_lines() {
        let input = "\nowner/repo\n\n";
        let repos = parse_repos(input).unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].as_str(), "owner/repo");
    }

    #[test]
    fn trims_whitespace() {
        let input = "  owner/repo  \n";
        let repos = parse_repos(input).unwrap();
        assert_eq!(repos[0].as_str(), "owner/repo");
    }

    #[test]
    fn returns_err_on_line_without_slash() {
        let input = "no-slash-here\n";
        assert!(parse_repos(input).is_err());
    }

    #[test]
    fn returns_empty_vec_for_empty_input() {
        let repos = parse_repos("").unwrap();
        assert!(repos.is_empty());
    }

    #[test]
    fn repo_display_equals_inner_string() {
        let r = Repo("myorg/myrepo".to_string());
        assert_eq!(format!("{r}"), "myorg/myrepo");
    }
}
