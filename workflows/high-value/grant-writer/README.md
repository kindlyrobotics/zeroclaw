# Grant Application Writer

Monitors grant portals and drafts applications for nonprofits and researchers.

## What It Does

- Scans grant portals (Grants.gov, foundations, EU Horizon) for matching opportunities
- Scores eligibility based on organization profile
- Extracts requirements and pre-fills application templates
- Drafts narrative sections using past successful applications as reference
- Tracks deadlines with escalating alerts

## Tools Used

- `browser` — scrape grant portals, download RFP documents
- `http_request` — Grants.gov API
- `shell` — PDF parsing and form filling
- `memory` — org profile, past applications, success patterns
- `file_write` — application drafts, budget templates
- `email` (channel) — deliver drafts and alerts

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Add cron jobs:

```bash
# Daily grant scan — 7am
zeroclaw cron add '0 7 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/find-grants.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per application**: $100-300
- **Monthly retainer**: $200-500/mo
- **Success fee**: 3-5% of awarded grant value

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for grant portals
