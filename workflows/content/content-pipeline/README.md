# Content Pipeline Agent

End-to-end content production: research, outline, draft, and deliver.

## What It Does

- Takes content briefs (topics, keywords, format) via messaging
- Researches the topic using browser and HTTP
- Creates outlines, then full drafts
- Delivers finished content via email
- Tracks content calendar in memory

## Tools Used

- `browser` — topic research
- `http_request` — API and data lookups
- `email` (channel) — deliver finished content
- `memory` — content calendar and briefs

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Start: `zeroclaw daemon`
4. Send content briefs via configured channel

Event-driven workflow — no cron needed for on-demand content. Optionally add scheduled content:

```bash
# Weekly content batch — Mondays at 8am
zeroclaw cron add '0 8 * * 1' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/draft.md)"'
```

## Revenue Model

- **Per article**: $100-500
- **Monthly retainer**: $1000-5000/mo (X articles per month)
- **Content package**: $2000-10000 for batch

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled
