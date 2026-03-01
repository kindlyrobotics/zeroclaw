# Product Listing Localizer

Adapts e-commerce listings for international marketplaces.

## What It Does

- Takes product listings and translates/adapts for target markets
- Optimizes keywords for local search behavior
- Converts units, sizing, and regulatory text
- Monitors listing performance and iterates
- Delivers localized listings via email

## Tools Used

- `browser` — scrape competitor listings for keyword research
- `http_request` — translation and marketplace APIs
- `memory` — localization rules per market, keyword databases
- `file_write` — localized listing content
- `email` (channel) — deliver localized listings
- `delegate` — one agent per target market

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Start: `zeroclaw daemon`

Event-driven — send product listing data to trigger localization.

## Revenue Model

- **Per SKU/market**: $5-15
- **Monthly catalog management**: $200-800/mo
- **Setup + optimization**: $500-2000

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for marketplace research
