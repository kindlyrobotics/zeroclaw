# Academic Literature Scout

Monitors research databases for relevant new papers.

## What It Does

- Monitors arXiv, PubMed, Google Scholar for new publications
- Scores relevance to researcher's interests
- Generates structured paper summaries
- Identifies research gaps and collaboration opportunities
- Delivers daily/weekly digests

## Tools Used

- `browser` — scrape academic databases, conference sites
- `http_request` — Semantic Scholar API, CrossRef API
- `memory` — research profile, citation graph, paper summaries
- `shell` — PDF text extraction
- `file_write` — literature reviews, bibliographies
- `email` (channel) — digest delivery

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Daily paper scan — 7am
zeroclaw cron add '0 7 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/scout.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per researcher/month**: $20-50/mo
- **Lab group**: $200-500/mo
- **Institutional**: $1000-5000/yr

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for academic databases
