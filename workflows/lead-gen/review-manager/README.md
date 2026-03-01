# Review Manager Agent

Monitors online reviews and drafts professional responses.

## What It Does

- Scrapes review sites (Google, Yelp, etc.) for new reviews
- Analyzes sentiment and urgency
- Drafts appropriate responses for each review
- Sends drafts via email for approval before posting
- Tracks review trends over time in memory

## Tools Used

- `browser` — scrape review sites
- `email` (channel) — deliver draft responses
- `memory` — track reviews and trends

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, and browser domains in config
3. Add cron jobs:

```bash
# Check for new reviews — twice daily
zeroclaw cron add '0 8,16 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/respond.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per client/month**: $300-800/mo
- **Per location**: $200-500/mo for multi-location businesses
- **Setup fee**: $250-500

## Required Environment

- `api_key` — AI provider API key
- Email credentials for delivering draft responses
- Browser enabled with review site domains
