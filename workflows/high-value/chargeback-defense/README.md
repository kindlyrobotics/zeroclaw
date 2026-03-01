# Chargeback Defense Agent

Automatically gathers evidence and drafts rebuttals for payment disputes.

## What It Does

- Detects chargeback notifications via email or webhook
- Gathers evidence: order details, delivery confirmation, customer comms
- Compiles formatted rebuttal documents following card network templates
- Tracks dispute deadlines and outcomes
- Generates win-rate analytics

## Tools Used

- `http_request` — pull order data from Shopify/Stripe APIs
- `browser` — screenshot delivery proof, gather tracking
- `email` (channel) — receive notifications, send evidence packages
- `file_write` — compile evidence documents
- `memory` — chargeback patterns, repeat offenders, outcome tracking

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, HTTP domains
3. Add cron jobs:

```bash
# Check for new chargebacks — every 2 hours
zeroclaw cron add '0 */2 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/defend.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per chargeback defended**: $25-50
- **Monthly retainer**: $200-500/mo for e-commerce merchants
- **Success fee**: 10-20% of recovered amount

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- HTTP enabled for payment/order APIs
- Browser for evidence gathering
