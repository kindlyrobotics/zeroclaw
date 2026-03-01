# Surplus Liquidation Matchmaker

Connects excess inventory sellers with discount buyers.

## What It Does

- Scrapes liquidation marketplaces for underpriced inventory
- Matches listings to buyer profiles based on category and margin potential
- Optimizes listing descriptions for faster sale
- Facilitates introductions between buyers and sellers
- Tracks deal flow and commission

## Tools Used

- `browser` — scrape liquidation sites
- `http_request` — marketplace APIs
- `memory` — buyer profiles, seller inventory, price history
- `email` (channel) — buyer/seller introductions
- `file_write` — deal sheets, listing optimizations

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Daily inventory scan — 7am
zeroclaw cron add '0 7 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/match.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Commission**: 5-10% of matched deal value
- **Buyer subscription**: $100-200/mo
- **Seller listing service**: $50-150/listing

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled with liquidation marketplace domains
