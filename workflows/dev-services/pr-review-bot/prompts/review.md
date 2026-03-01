# PR Review Task

Check for new pull requests and review them.

## Steps

1. Query the GitHub/GitLab API for open PRs not yet reviewed
2. Recall already-reviewed PRs from memory (tag "pr:reviewed")
3. For each new PR:
   a. Fetch the diff via API
   b. Read the PR description and context
   c. Analyze changes for:
      - Bugs and logic errors
      - Security issues
      - Performance concerns
      - Code quality and style
      - Test coverage
   d. Compile review feedback
   e. Post review comments via API (or send via channel)
   f. Save to memory with tag "pr:reviewed"
4. Send summary to Discord/Slack:
   - PRs reviewed this run
   - Critical issues found (if any)

## Review Comment Format

```
**[severity]** file.rs:42

Description of the issue.

Suggestion:
```code
// suggested fix
```
```
