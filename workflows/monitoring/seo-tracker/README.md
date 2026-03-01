# SEO Tracker Agent

Monitors search engine rankings and sends performance reports.

## What It Does

- Checks keyword rankings by searching Google
- Tracks position changes over time
- Monitors competitor rankings for the same keywords
- Delivers weekly ranking reports via email

## Tools Used

- `browser` — search engine queries
- `memory` — ranking history
- `email` (channel) — deliver reports

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Daily rank check — 6am
zeroclaw cron add '0 6 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/rank-report.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per domain/month**: $200-500/mo
- **Agency package** (10 domains): $1000-3000/mo
- **With recommendations**: $500-1500/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for search queries
