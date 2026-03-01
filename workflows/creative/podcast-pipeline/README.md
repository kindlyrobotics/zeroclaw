# Podcast Production Pipeline

End-to-end podcast production from research to promotion.

## What It Does

- Researches trending topics in a niche
- Generates episode outlines and talking points
- Scouts and reaches out to potential guests
- Prepares interview questions based on guest research
- Generates show notes and social media promotion posts
- Manages episode calendar

## Tools Used

- `browser` — topic research, guest scouting
- `http_request` — podcast platform APIs
- `memory` — episode archive, guest database, topic backlog
- `file_write` — outlines, show notes, social posts
- `email` (channel) — guest outreach, content delivery
- `shell` — file processing

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Weekly topic research — Mondays at 8am
zeroclaw cron add '0 8 * * 1' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/produce.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per podcaster/month**: $100-300/mo
- **Premium** (guest booking): $300-500/mo
- **Full production package**: $500-1000/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for research
