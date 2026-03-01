# Social Media Factory Agent

Automated social media content creation and scheduling.

## What It Does

- Researches trending topics and content ideas
- Creates platform-specific posts (Twitter/X, LinkedIn, etc.)
- Generates content batches for the week
- Delivers via email for review and manual posting
- Tracks content calendar and performance themes

## Tools Used

- `browser` — trend research and competitor analysis
- `email` (channel) — deliver content batches
- `memory` — content calendar, themes, and brand voice

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email, and browser domains
3. Add cron jobs:

```bash
# Weekly content batch — Sundays at 8pm
zeroclaw cron add '0 20 * * 0' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/create-posts.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per client/month**: $500-2000/mo
- **Per platform**: $300-800/mo per platform
- **Content + management**: $1500-4000/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials for content delivery
- Browser enabled for trend research
