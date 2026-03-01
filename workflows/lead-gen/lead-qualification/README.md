# Lead Qualification Agent

Conversational bot that qualifies inbound leads through Telegram or WhatsApp.

## What It Does

- Receives inbound messages from potential customers
- Asks qualifying questions (budget, timeline, needs, authority)
- Scores leads based on responses (BANT framework)
- Routes qualified leads to a delegate agent for handoff
- Stores qualification data in memory

## Tools Used

- `telegram` or `whatsapp` (channel) — inbound conversations
- `memory` — track lead status and scores
- `delegate` — hand off qualified leads

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set your API key and channel tokens in `~/.zeroclaw/config.toml`
3. Configure Telegram or WhatsApp channel
4. Start: `zeroclaw daemon`

No cron jobs needed — this workflow is event-driven (responds to messages).

## Revenue Model

- **Per client/month**: $500-2000/mo
- **Per qualified lead**: $50-200
- **Setup fee**: $500-1000

## Required Environment

- `api_key` — AI provider API key
- Telegram bot token OR WhatsApp Business API credentials
