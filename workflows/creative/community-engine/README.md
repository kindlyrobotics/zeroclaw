# Community Growth Engine

Autonomous Discord/Telegram community management and growth.

## What It Does

- Welcomes new members with personalized onboarding
- Answers FAQs from a knowledge base
- Identifies and rewards top contributors
- Detects and removes spam/scam messages
- Runs engagement campaigns (polls, challenges)
- Generates weekly community health reports

## Tools Used

- `discord`/`telegram` (channel) — community interaction
- `memory` — member profiles, FAQ knowledge base, engagement scores
- `file_write` — analytics reports
- `delegate` — moderation, engagement, analytics agents

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key and channel token
3. Add cron jobs:

```bash
# Daily engagement campaign — 10am
zeroclaw cron add '0 10 * * *' 'agent -m "Run today engagement activity"'

# Weekly health report — Sundays at 6pm
zeroclaw cron add '0 18 * * 0' 'agent -m "Generate weekly community health report"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per community/month**: $200-500/mo
- **Agency plan**: $1000-3000/mo (multiple communities)
- **Web3 communities**: $500-1500/mo

## Required Environment

- `api_key` — AI provider API key
- Discord or Telegram bot token
