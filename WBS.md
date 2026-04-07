# Work Breakdown Structure — gh-add-issue

## 1. Project Setup

### 1.1 Rust Project Initialisation
- 1.1.1 Initialise Cargo project (`cargo init`)
- 1.1.2 Configure `Cargo.toml` with required dependencies (e.g. `clap`, `dialoguer`, `skim` or `fuzzy-matcher`, `serde`, `reqwest` or `octocrab`)
- 1.1.3 Establish source directory structure (`main.rs`, modules)

### 1.2 GitHub CLI Extension Structure
- 1.2.1 Add `gh-add-issue` binary entry point conforming to `gh` extension conventions
- 1.2.2 Verify extension loads and is invocable via `gh add-issue`

### 1.3 Build Pipeline
- 1.3.1 Create `script/build.sh` for cross-platform compilation
- 1.3.2 Configure CI workflow (lint, test, build)

---

## 2. Authentication

### 2.1 Auth Status Check
- 2.1.1 Invoke `gh auth status` (or equivalent API call) at startup
- 2.1.2 Parse result to confirm an active, authenticated session exists

### 2.2 Auth Failure Handling
- 2.2.1 Display a clear, user-friendly error message when not authenticated
- 2.2.2 Exit with a non-zero status code

---

## 3. Repository Selection (Interactive Mode)

### 3.1 Repository List Retrieval
- 3.1.1 Query the GitHub API for repositories accessible to the authenticated user
- 3.1.2 Handle pagination to ensure the full list is returned
- 3.1.3 Format repository entries as `owner/repo` strings

### 3.2 Interactive Selection UI
- 3.2.1 Render the repository list in an interactive selector
- 3.2.2 Implement fuzzy search so the user can filter by typing
- 3.2.3 Support arrow-key navigation through the list
- 3.2.4 Confirm selection and pass the chosen repository to the next step

---

## 4. Issue Input (Interactive Mode)

### 4.1 Title Prompt
- 4.1.1 Display a prompted text input for the issue title
- 4.1.2 Validate that the title is not empty; re-prompt if blank

### 4.2 Description Prompt
- 4.2.1 Display a multi-line text input for the issue body
- 4.2.2 Allow the user to leave the description blank and continue

### 4.3 Optional Field Prompts
- 4.3.1 Assignees — prompt for one or more GitHub usernames; skippable
- 4.3.2 Labels — prompt for one or more label names; skippable
- 4.3.3 Project — prompt for a project name or number; skippable
- 4.3.4 Milestone — prompt for a milestone title or number; skippable

---

## 5. Non-Interactive Mode

### 5.1 Flag Definitions
- 5.1.1 Define `--repo` flag (required in non-interactive mode, format `owner/repo`)
- 5.1.2 Define `--title` flag (required in non-interactive mode)
- 5.1.3 Define `--body` flag (optional)

### 5.2 Flag Validation
- 5.2.1 Validate `--repo` is provided and correctly formatted
- 5.2.2 Validate `--title` is provided and non-empty
- 5.2.3 Surface clear error messages for missing or malformed flags

### 5.3 Flag-to-Issue Passthrough
- 5.3.1 Map flag values directly to issue creation payload, bypassing interactive prompts

---

## 6. Issue Creation

### 6.1 API Request Construction
- 6.1.1 Build the issue payload (title, body, assignees, labels, project, milestone)
- 6.1.2 Resolve the correct API endpoint for the selected repository

### 6.2 Issue Submission
- 6.2.1 Submit the issue creation request via the GitHub API (via `gh api` or direct HTTP)
- 6.2.2 Parse the API response to extract the new issue URL and number

### 6.3 Success Output
- 6.3.1 Display the created issue number and URL to the user
- 6.3.2 Exit with status code `0`

---

## 7. Error Handling

### 7.1 Network Errors
- 7.1.1 Detect and report connectivity failures with a descriptive message

### 7.2 API Errors
- 7.2.1 Parse GitHub API error responses (4xx, 5xx)
- 7.2.2 Surface the API error message to the user in plain language
- 7.2.3 Exit with a non-zero status code on any API failure

### 7.3 Input Validation Errors
- 7.3.1 Report missing required fields (title in non-interactive mode)
- 7.3.2 Report invalid repository format or unresolvable repository

---

## 8. Testing

### 8.1 Unit Tests
- 8.1.1 Input validation logic (title, repo format)
- 8.1.2 Flag parsing and non-interactive passthrough
- 8.1.3 API response parsing (success and error cases)

### 8.2 Integration Tests
- 8.2.1 End-to-end test against a real or mocked GitHub API
- 8.2.2 Test interactive flow using simulated input
- 8.2.3 Test non-interactive flow via flags

---

## 9. Documentation

### 9.1 README
- 9.1.1 Installation instructions
- 9.1.2 Prerequisites (Rust toolchain, `gh` CLI ≥ 2.0, authentication)
- 9.1.3 Usage examples — interactive and non-interactive modes
- 9.1.4 Description of all supported flags
