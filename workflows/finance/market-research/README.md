# Market Research Agent

Automated market analysis and research reports.

## What It Does

- Researches markets, competitors, and trends on demand
- Compiles structured research reports
- Monitors specific companies or sectors over time
- Delivers reports via email

## Tools Used

- `browser` — web research
- `http_request` — data APIs
- `memory` — research history, client preferences
- `email` (channel) — report delivery

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. For scheduled reports, add cron:

```bash
# Weekly market update — Fridays at 4pm
zeroclaw cron add '0 16 * * 5' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/analyze.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per report**: $200-2000
- **Monthly subscription**: $500-3000/mo
- **Enterprise retainer**: $2000-10000/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for web research
