# Requirements

This document lists the requirements for the GH Add Issue Github CLI extension.

## Functional Requirements

1. The extension must provide a command to create a new issue.
2. The command default behaviour will provide an interactive prompt prepopulated with a list of repositories accessible to the authenticated GitHub user.
3. The user must be able to select a repository from the list, either by typing the name (resulting in a fuzzy search) or using arrow keys to navigate.
4. After selecting a repository, the user must be prompted to enter the issue title and description.
5. The extension must validate that the issue title is not empty. A description is strongly encouraged but may be left blank.
6. The extension will also have prompts for assignees, labels, project, and milestone, but these are optional and can be skipped by the user.
7. The extension must create the issue in the selected repository with the provided title and description.
8. The extension must handle errors gracefully, providing informative messages to the user if the issue creation fails.
9. The extension must verify that the user is authenticated via `gh auth` before proceeding, and exit with a clear error message if not.
10. The extension must support non-interactive usage via flags (`--repo`, `--title`, `--body`) to allow scripting and automation.
11. The extension must be compatible with GitHub CLI version 2.0 or later.
12. The extension is to be developed using the Rust programming language.