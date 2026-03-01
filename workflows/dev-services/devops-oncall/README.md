# DevOps On-Call Agent

Automated infrastructure monitoring and incident response.

## What It Does

- Runs periodic health checks on services and infrastructure
- Detects anomalies and failures
- Attempts automated remediation for known issues
- Escalates unresolved issues via Telegram/Slack
- Maintains incident log in memory

## Tools Used

- `shell` — run health checks, restart services, check logs
- `http_request` — API health endpoints
- `memory` — incident history, runbooks
- `telegram` or `slack` (channel) — alerts and escalation

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key and channel tokens
3. Add cron jobs:

```bash
# Health check — every 5 minutes
zeroclaw cron add '*/5 * * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/diagnose.md)"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per client/month**: $500-3000/mo
- **Per server**: $50-200/mo
- **Incident response**: $200-500/incident

## Required Environment

- `api_key` — AI provider API key
- Telegram or Slack bot token
- SSH access to monitored servers (via shell)
- HTTP request enabled for health endpoints
