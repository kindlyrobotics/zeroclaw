# Newsletter Agent

Automated newsletter research, writing, and delivery.

## What It Does

- Researches trending topics in your niche using browser and HTTP
- Curates and summarizes relevant content
- Writes newsletter editions with consistent voice and formatting
- Sends via email channel on schedule

## Tools Used

- `browser` — research trending content
- `http_request` — fetch RSS feeds and APIs
- `email` (channel) — deliver newsletters
- `memory` — track topics covered, subscriber preferences

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, and browser domains
3. Add cron jobs:

```bash
# Research phase — every Monday at 7am
zeroclaw cron add '0 7 * * 1' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/research.md)"'

# Write and send — every Monday at 10am
zeroclaw cron add '0 10 * * 1' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/compose.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Subscription**: $10-50/mo per subscriber
- **Sponsored editions**: $500-5000/edition
- **White-label for clients**: $1000-3000/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials (SMTP for sending)
- Browser enabled for content research
