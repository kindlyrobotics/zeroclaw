# Freelance Bid Autopilot

Monitors job boards and submits personalized proposals fast.

## What It Does

- Monitors Upwork/Fiverr/Freelancer for new job postings
- Scores job quality (budget, client history, competition)
- Drafts personalized proposals referencing client-specific details
- Alerts for approval before submission
- Tracks win rate and optimizes proposal strategy

## Tools Used

- `browser` — scrape job boards, submit proposals
- `memory` — skill profile, past proposals, win/loss data
- `telegram` (channel) — approval flow before submission
- `file_read` — portfolio/resume for context

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, Telegram token, browser domains
3. Add cron jobs:

```bash
# Job scan — every 15 minutes during business hours
zeroclaw cron add '*/15 8-20 * * 1-5' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/bid.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per freelancer/month**: $50-150/mo
- **Success fee**: 5-10% of first payment on won jobs
- **Premium** (with profile optimization): $100-300/mo

## Required Environment

- `api_key` — AI provider API key
- Telegram bot token
- Browser enabled with freelance platform domains
