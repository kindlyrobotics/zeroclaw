# Competitor Intelligence War Room

Continuous competitive monitoring across multiple surfaces.

## What It Does

- Monitors competitor websites for changes (pricing, features, pages)
- Tracks competitor job postings for hiring signals
- Watches social media activity and press releases
- Detects technology stack changes
- Generates weekly intelligence briefs with strategic implications

## Tools Used

- `browser` — scrape competitor sites, job boards, social media
- `http_request` — news APIs, tech detection
- `memory` — historical snapshots, trend tracking
- `file_write` — intelligence reports
- `email` (channel) — deliver briefs
- `delegate` — per-competitor agents

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Daily competitor scan — 6am
zeroclaw cron add '0 6 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/monitor.md)"'

# Weekly intelligence brief — Fridays at 4pm
zeroclaw cron add '0 16 * * 5' 'agent -m "Generate weekly competitive intelligence brief from this weeks data"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per competitor/month**: $200-500/mo
- **Startup package** (3 competitors): $500-1000/mo
- **Enterprise** (10+ competitors): $2000-5000/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled
