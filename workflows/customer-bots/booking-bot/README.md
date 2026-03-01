# Booking Bot

Conversational appointment scheduling agent.

## What It Does

- Handles appointment booking via WhatsApp or Telegram
- Checks availability (stored in memory)
- Confirms bookings and sends reminders
- Manages cancellations and rescheduling
- Sends daily schedule summary

## Tools Used

- `whatsapp` or `telegram` (channel) — customer conversations
- `memory` — calendar and booking data
- `email` (channel, optional) — confirmation emails

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key and channel tokens
3. Add cron jobs:

```bash
# Daily schedule summary — 7am
zeroclaw cron add '0 7 * * *' 'agent -m "Review today schedule and send summary"'

# Appointment reminders — 8am
zeroclaw cron add '0 8 * * *' 'agent -m "Send reminders for today appointments"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per booking**: $2-10
- **Per client/month**: $200-800/mo
- **Setup fee**: $500-1000

## Required Environment

- `api_key` — AI provider API key
- WhatsApp or Telegram bot token
- Email credentials (optional, for confirmations)
