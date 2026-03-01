# Outbound Prospecting Agent

Automated lead discovery and outreach using browser scraping and email.

## What It Does

- Scrapes target websites and directories for prospect data (names, emails, companies)
- Researches each prospect to personalize outreach
- Drafts and sends cold emails with follow-up sequences
- Tracks responses and logs results to memory

## Tools Used

- `browser` — scrape websites, LinkedIn, directories
- `email` (channel) — send outreach emails
- `memory` — track prospects and campaign state
- `shell` — data processing

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set your API key and email credentials in `~/.zeroclaw/config.toml`
3. Configure `[browser].allowed_domains` for your target sites
4. Add cron jobs:

```bash
# Prospect discovery — daily at 9am
zeroclaw cron add '0 9 * * 1-5' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/scrape.md)"'

# Follow-up check — daily at 2pm
zeroclaw cron add '0 14 * * 1-5' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/followup.md)"'
```

5. Start: `zeroclaw daemon`

## Revenue Model

- **Per campaign**: $500-2000/campaign
- **Retainer**: $1000-3000/mo per client
- **Per qualified lead**: $25-100/lead

## Required Environment

- `api_key` — AI provider API key
- Email credentials (IMAP/SMTP) for outreach
- Browser enabled with allowed domains
