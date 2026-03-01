# RFP / Tender Sniper Agent

Monitors government procurement portals for contract opportunities and drafts proposals.

## What It Does

- Scrapes procurement portals (SAM.gov, state bid boards, EU TED) for new RFPs
- Filters by keyword, NAICS code, budget range, and agency type
- Scores bid/no-bid probability based on past win patterns
- Drafts proposal outlines with deadline countdown
- Alerts via Telegram/Discord with opportunity details

## Tools Used

- `browser` — scrape procurement portals
- `http_request` — API-based portals
- `shell` — PDF parsing for bid documents
- `memory` — bid history, win/loss patterns, org capabilities
- `file_write` — proposal drafts
- `telegram`/`discord` (channel) — opportunity alerts

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key and channel tokens
3. Configure browser domains for target procurement portals
4. Add cron jobs:

```bash
# Daily opportunity scan — 8am
zeroclaw cron add '0 8 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/scan.md)"'
```

5. Start: `zeroclaw daemon`

## Revenue Model

- **Per contractor/month**: $200-500/mo
- **Success fee**: 1-3% of won contract value
- **Enterprise**: $500-2000/mo for multi-agency monitoring

## Required Environment

- `api_key` — AI provider API key
- Telegram or Discord bot token
- Browser enabled with procurement portal domains
