# Form Filling Agent

Automated web form submission for repetitive tasks.

## What It Does

- Fills and submits web forms using browser automation
- Reads input data from memory or messages
- Handles multi-step forms and confirmation flows
- Reports submission status via Telegram
- Tracks completed submissions in memory

## Tools Used

- `browser` — navigate and fill forms
- `memory` — input data and submission history
- `telegram` (channel) — receive instructions, report status

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, Telegram token, browser domains
3. Start: `zeroclaw daemon`

Event-driven — send form filling requests via Telegram. Or schedule with cron for batch processing.

## Revenue Model

- **Per submission**: $5-50
- **Monthly batch**: $200-1000/mo
- **Setup + automation**: $500-2000 setup

## Required Environment

- `api_key` — AI provider API key
- Telegram bot token
- Browser enabled with form domains
