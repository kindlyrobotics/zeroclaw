# Mystery Shopper Automator

Automated UX competitive analysis through simulated shopping.

## What It Does

- Browses competitor websites as a customer
- Tests purchase flows, search, checkout friction
- Measures page load times and mobile responsiveness
- Tests customer support response times
- Produces detailed UX audit reports with screenshots

## Tools Used

- `browser` — full interaction testing with screenshots
- `http_request` — performance measurement
- `memory` — benchmark database, historical comparisons
- `shell` — performance analysis
- `file_write` — UX audit reports
- `email` (channel) — deliver reports

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Weekly UX audit — Saturdays at 6am
zeroclaw cron add '0 6 * * 6' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/shop.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per competitor/month**: $200-500/mo
- **Quarterly deep-dive**: $500-1500
- **Continuous monitoring**: $300-800/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for competitor sites
