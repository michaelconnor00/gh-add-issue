use crate::repos::Repo;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::io::IsTerminal;

/// Resolve a raw selection index (0-based) to the corresponding `Repo`.
///
/// This pure function is extracted from the interactive picker so that it can
/// be unit-tested without a terminal.
pub fn resolve_selection(repos: &[Repo], index: usize) -> Result<&Repo, String> {
    repos.get(index).ok_or_else(|| {
        format!(
            "Selection index {index} is out of range (list has {} repositories).",
            repos.len()
        )
    })
}

/// Present an interactive fuzzy-search selector populated with `repos` and
/// return a reference to the chosen `Repo`.
///
/// The user can filter by typing (fuzzy match) and navigate with arrow keys.
/// Returns `Err` if the list is empty, stdin is not a TTY, or the user aborts.
pub fn select_repo(repos: &[Repo]) -> Result<&Repo, String> {
    if repos.is_empty() {
        return Err("No repositories found for the authenticated user.".to_string());
    }

    if !std::io::stdin().is_terminal() {
        return Err(
            "Interactive selection requires a terminal. \
             Use --repo to specify a repository non-interactively."
                .to_string(),
        );
    }

    let items: Vec<&str> = repos.iter().map(|r| r.as_str()).collect();

    log::debug!("Presenting fuzzy selector with {} repositories.", items.len());

    let index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a repository")
        .items(&items)
        .default(0)
        .interact()
        .map_err(|e| format!("Repository selection failed: {e}"))?;

    log::debug!("User selected index {index}: {}", repos[index].as_str());

    resolve_selection(repos, index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repos::Repo;

    fn make_repos(names: &[&str]) -> Vec<Repo> {
        names.iter().map(|n| Repo(n.to_string())).collect()
    }

    #[test]
    fn resolve_returns_correct_repo() {
        let repos = make_repos(&["owner/alpha", "owner/beta", "owner/gamma"]);
        assert_eq!(resolve_selection(&repos, 0).unwrap().as_str(), "owner/alpha");
        assert_eq!(resolve_selection(&repos, 2).unwrap().as_str(), "owner/gamma");
    }

    #[test]
    fn resolve_returns_err_for_out_of_range_index() {
        let repos = make_repos(&["owner/alpha"]);
        let err = resolve_selection(&repos, 1).unwrap_err();
        assert!(err.contains("out of range"), "unexpected: {err}");
    }

    #[test]
    fn resolve_returns_err_for_empty_list() {
        let repos: Vec<Repo> = vec![];
        let err = resolve_selection(&repos, 0).unwrap_err();
        assert!(err.contains("out of range"), "unexpected: {err}");
    }

    #[test]
    fn select_repo_returns_err_when_list_is_empty() {
        let repos: Vec<Repo> = vec![];
        let err = select_repo(&repos).unwrap_err();
        assert!(
            err.contains("No repositories found"),
            "unexpected: {err}"
        );
    }

    /// In a test environment stdin is not a terminal, so select_repo must
    /// return the non-TTY error before attempting to render any UI.
    #[test]
    fn select_repo_returns_err_when_not_a_tty() {
        let repos = make_repos(&["owner/repo"]);
        // Tests run with stdin that is not a terminal (pipe/redirect).
        if !std::io::stdin().is_terminal() {
            let err = select_repo(&repos).unwrap_err();
            assert!(
                err.contains("requires a terminal"),
                "unexpected: {err}"
            );
        }
    }
}
