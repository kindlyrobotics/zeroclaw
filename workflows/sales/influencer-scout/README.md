# Influencer Scout & Outreach Agent

Finds micro-influencers and manages sponsorship outreach.

## What It Does

- Crawls social platforms for influencers in a specific niche
- Analyzes follower count, engagement rate, content relevance
- Compiles ranked lists with contact info
- Drafts personalized sponsorship outreach referencing specific posts
- Manages follow-up sequences

## Tools Used

- `browser` — scrape profiles, engagement metrics, content
- `http_request` — platform APIs where available
- `memory` — influencer database, outreach history
- `email` (channel) — outreach and follow-ups
- `delegate` — separate agents per platform

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Weekly influencer discovery — Mondays at 9am
zeroclaw cron add '0 9 * * 1' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/discover.md)"'

# Follow-up check — daily at 2pm
zeroclaw cron add '0 14 * * 1-5' 'agent -m "Check for influencer responses and send follow-ups"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per brand/month**: $300-800/mo
- **Per campaign**: $500-2000
- **Agency plan**: $1000-3000/mo (multiple brands)

## Required Environment

- `api_key` — AI provider API key
- Email credentials for outreach
- Browser enabled with social platform domains
