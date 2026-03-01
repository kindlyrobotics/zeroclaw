# Rental Property Autopilot

Airbnb/VRBO property management automation.

## What It Does

- Monitors competitor listings for pricing changes
- Generates dynamic pricing recommendations
- Responds to guest inquiries within minutes
- Generates check-in/check-out instructions
- Monitors and drafts review responses
- Tracks maintenance requests
- Generates monthly owner reports

## Tools Used

- `browser` — scrape competitor listings, booking dashboards
- `http_request` — booking platform APIs
- `memory` — property details, guest history, pricing history
- `whatsapp`/`telegram` (channel) — guest communication
- `email` (channel) — owner reports
- `file_write` — reports, instructions

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, channel tokens, browser domains
3. Add cron jobs:

```bash
# Pricing check — daily at 8am
zeroclaw cron add '0 8 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/manage.md)"'

# Monthly report — 1st of each month
zeroclaw cron add '0 9 1 * *' 'agent -m "Generate monthly property report"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per property/month**: $30-75/mo
- **Portfolio** (5+ units): $150-375/mo
- **Full management**: $100-200/mo per unit

## Required Environment

- `api_key` — AI provider API key
- WhatsApp or Telegram bot token
- Email credentials
- Browser enabled for booking platforms
