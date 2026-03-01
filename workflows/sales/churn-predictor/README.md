# SaaS Churn Prediction & Win-Back Agent

Monitors usage signals and triggers automated re-engagement.

## What It Does

- Monitors product usage data for declining engagement signals
- Scores accounts by churn probability
- Triggers automated win-back sequences (emails, offers)
- Escalates high-value accounts to human CSMs
- Tracks intervention effectiveness

## Tools Used

- `http_request` — pull usage data from analytics APIs
- `memory` — customer health scores, engagement history, intervention outcomes
- `email` (channel) — win-back campaigns
- `browser` — verify product usage, competitive intelligence
- `delegate` — analytics agent, outreach agent

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, HTTP domains
3. Add cron jobs:

```bash
# Daily health score calculation — 6am
zeroclaw cron add '0 6 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/predict.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per SaaS client/month**: $300-1000/mo
- **Per prevented churn**: $100-500 (success fee)
- **Enterprise**: $2000-5000/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- HTTP enabled for product analytics APIs
