# Arbitrage Finder Agent

Monitors prices across platforms to identify arbitrage opportunities.

## What It Does

- Scrapes multiple marketplaces for product/asset prices
- Compares prices across platforms in real-time
- Calculates profit margins after fees
- Alerts on profitable arbitrage opportunities via Telegram
- Tracks historical spreads in memory

## Tools Used

- `browser` — scrape marketplace prices
- `http_request` — API-based price feeds
- `memory` — price history and opportunity tracking
- `telegram` (channel) — opportunity alerts

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, Telegram token, browser and HTTP domains
3. Add cron jobs:

```bash
# Price scan — every 2 hours
zeroclaw cron add '0 */2 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/compare.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per alert subscription/month**: $50-300/mo
- **Percentage of profit**: 5-15% of arbitrage profit
- **Premium tier** (more markets): $200-500/mo

## Required Environment

- `api_key` — AI provider API key
- Telegram bot token
- Browser enabled with marketplace domains
- HTTP request enabled for price APIs
