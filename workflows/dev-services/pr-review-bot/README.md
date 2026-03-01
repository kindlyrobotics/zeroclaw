# PR Review Bot

Automated pull request code review agent.

## What It Does

- Monitors repositories for new PRs (via webhook or polling)
- Reviews code changes for quality, bugs, and style
- Posts review comments with specific feedback
- Suggests improvements and catches common issues

## Tools Used

- `shell` — git operations, diff analysis
- `file_read` — code inspection
- `http_request` — GitHub/GitLab API
- `discord` or `slack` (channel) — review notifications
- `memory` — project conventions, past review patterns

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, channel tokens, HTTP domains
3. Add cron jobs:

```bash
# Check for new PRs — every 30 minutes
zeroclaw cron add '*/30 * * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/review.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per repo/month**: $200-500/mo
- **Per organization**: $500-2000/mo
- **Enterprise**: $2000-5000/mo

## Required Environment

- `api_key` — AI provider API key
- GitHub/GitLab personal access token
- Discord or Slack bot token
- HTTP request enabled for Git provider APIs
