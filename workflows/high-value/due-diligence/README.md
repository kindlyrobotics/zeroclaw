# Startup Due Diligence Researcher

Comprehensive company research for investors and M&A teams.

## What It Does

- Researches company from multiple angles: website, LinkedIn, Crunchbase, patents, reviews
- Analyzes founding team, product, market position, and financials
- Produces structured investment memo with risk flags
- Compares against similar companies in memory

## Tools Used

- `browser` — scrape all data sources
- `http_request` — company data APIs, SEC filings
- `memory` — comparable company database, research templates
- `file_write` — investment memos
- `shell` — data analysis
- `email` (channel) — deliver reports
- `delegate` — specialized research sub-agents

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Start: `zeroclaw daemon`

Event-driven — send company name/URL to trigger research.

## Revenue Model

- **Per report**: $50-200
- **Deal flow subscription**: $500-1500/mo (unlimited reports)
- **Deep-dive reports**: $500-2000

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled
