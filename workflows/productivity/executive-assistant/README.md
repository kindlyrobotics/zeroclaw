# Executive Assistant Agent

Personal AI assistant with email triage, reminders, and research.

## What It Does

- Triages incoming emails by priority and category
- Sets and manages reminders via cron
- Researches topics on demand using browser
- Sends daily briefings with schedule and priorities
- Manages to-do lists and follow-ups in memory

## Tools Used

- `email` (channel) — email triage and responses
- `telegram` or `imessage` (channel) — quick interactions
- `browser` — research tasks
- `memory` — to-dos, context, preferences
- `shell` — calendar integration (if configured)

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email and messaging credentials
3. Add cron jobs:

```bash
# Morning briefing — 7am weekdays
zeroclaw cron add '0 7 * * 1-5' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/triage.md)"'

# Evening wrap-up — 6pm weekdays
zeroclaw cron add '0 18 * * 1-5' 'agent -m "Review today tasks and prepare tomorrow priorities"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per user/month**: $200-1000/mo
- **Executive tier**: $500-2000/mo (dedicated instance)
- **Team package**: $1000-5000/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials (IMAP/SMTP)
- Telegram bot token or iMessage access
- Browser enabled for research
