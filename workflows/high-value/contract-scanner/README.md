# Contract Red-Flag Scanner

Analyzes contracts for risky clauses and hidden terms.

## What It Does

- Accepts contract documents via channel message
- Extracts and analyzes all clauses
- Identifies red flags: auto-renewal, non-compete, IP assignment, hidden fees
- Returns risk-scored summary with plain-language explanations
- Suggests negotiation points

## Tools Used

- `file_read` — parse uploaded documents
- `shell` — PDF-to-text extraction
- `memory` — clause pattern database, jurisdiction-specific rules
- `telegram`/`whatsapp` (channel) — receive documents and deliver analysis

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key and channel token
3. Start: `zeroclaw daemon`

Event-driven — send a contract document to trigger analysis.

## Revenue Model

- **Per scan**: $5-25
- **Monthly subscription**: $50-150/mo for freelancers/small businesses
- **Enterprise**: $200-500/mo

## Required Environment

- `api_key` — AI provider API key
- Telegram or WhatsApp bot token
