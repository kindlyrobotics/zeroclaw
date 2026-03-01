# Meal Prep & Grocery Optimizer

Weekly meal planning with price-optimized grocery lists.

## What It Does

- Generates weekly meal plans based on dietary preferences
- Optimizes for nutrition, cost, and minimal food waste
- Compares grocery prices across stores
- Creates organized shopping lists
- Tracks pantry inventory to avoid redundant purchases
- Learns from user feedback

## Tools Used

- `browser` — scrape grocery store prices and sales
- `http_request` — nutrition databases, recipe APIs
- `memory` — dietary preferences, pantry inventory, price history
- `file_write` — meal plans, shopping lists
- `telegram`/`whatsapp` (channel) — daily meal reminders, shopping lists

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, channel token, browser domains
3. Add cron jobs:

```bash
# Weekly meal plan — Sundays at 6pm
zeroclaw cron add '0 18 * * 0' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/plan-meals.md)"'

# Daily meal reminder — 8am
zeroclaw cron add '0 8 * * *' 'agent -m "Send today meal plan reminder"'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per user/month**: $5-15/mo
- **Family plan**: $10-25/mo
- **Dietitian white-label**: $50-100/mo per practitioner

## Required Environment

- `api_key` — AI provider API key
- Telegram or WhatsApp bot token
- Browser enabled for grocery store sites
