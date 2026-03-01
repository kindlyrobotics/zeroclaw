# Website Monitor Agent

Monitors websites for changes and sends alerts.

## What It Does

- Periodically visits target URLs and takes screenshots
- Compares against previous snapshots for visual/content changes
- Checks uptime, response time, and SSL status via HTTP
- Sends alerts on detected changes via Telegram or email

## Tools Used

- `browser` — screenshot and content extraction
- `http_request` — uptime and response checks
- `memory` — store snapshots for comparison
- `telegram` or `email` (channel) — alert delivery

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, channel tokens, browser and HTTP domains
3. Add cron jobs:

```bash
# Health check — every 15 minutes
zeroclaw cron add '*/15 * * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/diff-report.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per site/month**: $50-200/mo
- **Package** (5 sites): $200-500/mo
- **Enterprise**: $500-2000/mo (custom SLAs)

## Required Environment

- `api_key` — AI provider API key
- Telegram bot token or email credentials
- Browser and HTTP request enabled
