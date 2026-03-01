# Support Agent

Multi-channel customer support bot with knowledge base.

## What It Does

- Handles customer inquiries across Telegram, Discord, WhatsApp, and email
- Answers questions using knowledge stored in memory
- Escalates complex issues to human support
- Tracks ticket status and customer satisfaction
- Learns from resolved issues to improve future responses

## Tools Used

- `telegram`, `discord`, `whatsapp`, `email` (channels) — multi-channel support
- `memory` — knowledge base, ticket tracking
- `file_read` — reference documentation
- `delegate` — escalation agent

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key and channel tokens for desired platforms
3. Start: `zeroclaw daemon`

Event-driven — responds to incoming customer messages.

## Revenue Model

- **Per client/month**: $500-3000/mo
- **Per channel**: $200-800/mo
- **Per resolution**: $5-20/ticket

## Required Environment

- `api_key` — AI provider API key
- Channel tokens for each platform you want to support
