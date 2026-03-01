# Report Generator Agent

Automated report creation from data sources.

## What It Does

- Collects data from multiple sources (web, APIs, files)
- Analyzes and synthesizes data into structured reports
- Generates reports on schedule or on demand
- Delivers via email with consistent formatting

## Tools Used

- `browser` — web data collection
- `http_request` — API data sources
- `file_read` — local data files
- `shell` — data processing
- `email` (channel) — report delivery
- `memory` — report templates, historical data

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, data source access
3. Add cron jobs:

```bash
# Weekly report — Fridays at 5pm
zeroclaw cron add '0 17 * * 5' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/report.md)"'

# Monthly report — 1st of each month
zeroclaw cron add '0 9 1 * *' 'agent -m "Generate monthly summary report"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per report**: $100-1000
- **Monthly reporting**: $300-2000/mo
- **Custom dashboard reports**: $1000-5000 setup

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser and HTTP enabled for data sources
