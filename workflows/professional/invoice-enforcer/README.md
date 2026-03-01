# Invoice Collection Enforcer

Automated accounts receivable management with escalating reminders.

## What It Does

- Tracks unpaid invoices and aging
- Sends escalating payment reminders on configurable schedule
- Generates payment links and aging reports
- Can negotiate payment plans via channel conversation
- Produces aging and collection analytics

## Tools Used

- `http_request` — invoicing API integration
- `file_read` — invoice parsing
- `memory` — payment history, client profiles, escalation state
- `email` (channel) — reminder delivery
- `whatsapp`/`telegram` (channel) — direct client communication
- `file_write` — aging reports

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, channel tokens
3. Add cron jobs:

```bash
# Daily aging check — 9am
zeroclaw cron add '0 9 * * 1-5' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/collect.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per freelancer/month**: $50-100/mo
- **Success fee**: 5-10% of collected overdue invoices
- **Business plan**: $200-500/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- HTTP enabled for invoicing APIs (optional)
