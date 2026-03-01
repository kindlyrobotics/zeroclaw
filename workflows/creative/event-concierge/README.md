# Event Concierge & Vendor Coordinator

Autonomous event planning and vendor management.

## What It Does

- Takes event requirements (type, date, guests, budget, location)
- Searches for venues with availability
- Contacts vendors (catering, A/V, photography) via email
- Manages RSVP tracking
- Sends attendee reminders and logistics
- Generates day-of run sheets

## Tools Used

- `browser` — venue search, vendor research, pricing
- `http_request` — booking APIs, calendar
- `memory` — vendor database, attendee lists, event history
- `email` (channel) — vendor outreach, attendee communication
- `file_write` — run sheets, budgets, contracts

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Start: `zeroclaw daemon`

Event-driven + cron for deadline reminders:

```bash
# Daily deadline check — 9am
zeroclaw cron add '0 9 * * *' 'agent -m "Check event deadlines and send reminders"'
```

## Revenue Model

- **Per event**: $100-500
- **Monthly retainer**: $200-600/mo
- **Full-service planning**: $500-2000/event

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for venue/vendor search
