# Trademark & Brand Watchdog

Monitors for unauthorized brand use and counterfeits.

## What It Does

- Scans marketplaces for counterfeit product listings
- Checks for impersonator accounts on social media
- Monitors domain registrations for typosquatting
- Drafts cease-and-desist letters and DMCA notices
- Tracks takedown status and outcomes

## Tools Used

- `browser` — marketplace scraping, social scanning, WHOIS lookups
- `http_request` — domain registration APIs
- `memory` — brand assets, infringement history, takedown tracking
- `file_write` — C&D letters, DMCA notices
- `email` (channel) — alerts and takedown submissions

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Daily brand scan — 7am
zeroclaw cron add '0 7 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/scan-brand.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per brand/month**: $150-400/mo
- **Per takedown**: $25-75
- **Enterprise** (portfolio): $500-2000/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for marketplace and social platform domains
