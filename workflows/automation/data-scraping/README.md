# Data Scraping Agent

Automated web data extraction and delivery service.

## What It Does

- Scrapes structured data from websites on schedule
- Extracts and normalizes data into consistent formats
- Delivers datasets via email as formatted reports
- Handles pagination, dynamic content, and rate limiting
- Tracks scraping history and data freshness in memory

## Tools Used

- `browser` — navigate and extract web content
- `http_request` — API-based data sources
- `shell` — data processing and formatting
- `file_write` — save extracted data
- `email` (channel) — deliver datasets
- `memory` — track scraping state and history

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Daily scrape — 6am
zeroclaw cron add '0 6 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/scrape.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per job**: $100-1000
- **Recurring scrape/month**: $200-1000/mo
- **Custom data pipeline**: $1000-5000 setup + $500/mo

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled with target domains
