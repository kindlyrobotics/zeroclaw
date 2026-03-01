# Regulatory License Renewal Tracker

Tracks professional and business license deadlines.

## What It Does

- Maintains inventory of all active licenses and permits
- Monitors renewal deadlines with escalating alerts
- Checks regulatory sites for requirement changes
- Pre-fills renewal applications
- Generates compliance status reports

## Tools Used

- `browser` — scrape licensing portals, download forms
- `http_request` — regulatory databases
- `memory` — license inventory, deadlines, requirements
- `shell` — PDF form filling
- `file_write` — pre-filled applications, compliance reports
- `email` (channel) — deadline alerts and reports

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Daily deadline check — 8am
zeroclaw cron add '0 8 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/check-renewals.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per business/month**: $100-300/mo
- **Per license tracked**: $20-50/mo
- **Multi-state businesses**: $300-1000/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for licensing portals
