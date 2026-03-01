# Job Hunter Agent

Monitors job boards and sends curated job alerts.

## What It Does

- Scrapes job boards for new postings matching criteria
- Filters and scores jobs by relevance
- Sends curated daily digest of new opportunities
- Tracks applied positions in memory

## Tools Used

- `browser` — scrape job boards
- `http_request` — job board APIs
- `memory` — track seen jobs and applications
- `telegram` or `email` (channel) — deliver alerts

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, channel tokens, browser domains
3. Add cron jobs:

```bash
# Job search — twice daily
zeroclaw cron add '0 8,17 * * 1-5' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/filter-jobs.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per user/month**: $30-100/mo
- **Premium** (with application tracking): $100-200/mo
- **Enterprise** (hiring team alerts): $300-1000/mo

## Required Environment

- `api_key` — AI provider API key
- Telegram bot token or email credentials
- Browser enabled with job board domains
