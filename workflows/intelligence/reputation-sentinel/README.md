# Reputation Crisis Sentinel

Real-time brand reputation monitoring and response management.

## What It Does

- Monitors review sites, social media, news, and forums for brand mentions
- Detects sentiment spikes and anomalies
- Auto-drafts empathetic responses for negative reviews
- Identifies testimonial candidates from positive reviews
- Generates weekly reputation scorecards

## Tools Used

- `browser` — scrape review sites, social media, news, forums
- `http_request` — social APIs
- `memory` — sentiment history, response templates, brand voice
- `file_write` — reputation reports
- `email` (channel) — alerts and reports

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Hourly monitoring — every hour 8am-10pm
zeroclaw cron add '0 8-22 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/monitor-reputation.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per business/month**: $150-400/mo
- **Per location**: $50-100/mo for multi-location
- **Agency plan**: $500-2000/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for review and social platforms
