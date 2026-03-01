# Expired Domain Hunter

Finds high-value expiring domains for acquisition.

## What It Does

- Scans expiring domain registrars for domains with SEO value
- Evaluates backlink profiles, traffic, and keyword relevance
- Scores profit potential
- Alerts on high-value opportunities
- Tracks domain portfolio value over time

## Tools Used

- `browser` — scrape auction sites, WHOIS, backlink checkers
- `http_request` — domain APIs
- `memory` — portfolio tracking, valuation history
- `telegram` (channel) — opportunity alerts
- `file_write` — valuation reports

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, Telegram token, browser domains
3. Add cron jobs:

```bash
# Hourly domain scan — during business hours
zeroclaw cron add '0 8-22 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/hunt-domains.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Domain flipping**: direct profit $50-50,000+ per flip
- **Service for investors**: $100-300/mo
- **Portfolio management**: $200-500/mo

## Required Environment

- `api_key` — AI provider API key
- Telegram bot token
- Browser enabled for domain auction sites
