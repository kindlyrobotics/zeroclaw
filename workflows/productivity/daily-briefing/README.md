# Daily Briefing Agent

Automated morning briefing with news, weather, and tasks.

## What It Does

- Researches relevant news and updates every morning
- Compiles a personalized daily briefing
- Delivers via email or Telegram at a set time
- Adapts content based on user interests stored in memory

## Tools Used

- `browser` — news and information gathering
- `http_request` — weather and data APIs
- `memory` — user preferences, past briefings
- `email` or `telegram` (channel) — briefing delivery

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key and channel credentials
3. Add cron job:

```bash
# Daily briefing — 6am
zeroclaw cron add '0 6 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/briefing.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per user/month**: $20-100/mo
- **Premium** (custom sources): $50-200/mo
- **Team briefing**: $200-500/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials or Telegram bot token
- Browser enabled for news research
