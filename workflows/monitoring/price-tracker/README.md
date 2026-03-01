# Price Tracker Agent

Monitors product prices and sends alerts on changes.

## What It Does

- Scrapes configured product pages for current prices
- Compares against saved price history
- Alerts on price drops, increases, or availability changes
- Maintains price trend data in memory

## Tools Used

- `browser` — scrape product pages
- `memory` — price history
- `telegram` (channel) — price alerts

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, Telegram token, browser domains
3. Add cron jobs:

```bash
# Price check — every 6 hours
zeroclaw cron add '0 */6 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/price-alert.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per tracker/month**: $20-100/mo
- **Bundle** (10 products): $100-300/mo
- **B2B competitive pricing**: $500-2000/mo

## Required Environment

- `api_key` — AI provider API key
- Telegram bot token
- Browser enabled with retailer domains
