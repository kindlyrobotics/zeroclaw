# ZeroClaw Workflow Templates

Ready-to-use agent workflow templates for revenue-generating businesses. Each workflow uses only ZeroClaw's native tools — no third-party middlemen.

## Quick Start

```bash
# 1. Pick a workflow
ls workflows/

# 2. Run setup
cd workflows/<category>/<workflow>
bash ../../_base/setup.sh

# 3. Add cron jobs (if the workflow uses scheduled tasks)
# Each workflow README lists the cron commands to run

# 4. Start the daemon
zeroclaw daemon
```

## Workflow Index (50 Templates)

### Lead Gen
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 1 | [Outbound Prospecting](lead-gen/outbound-prospecting/) | Email, Browser | Per campaign |
| 2 | [Lead Qualification](lead-gen/lead-qualification/) | Telegram/WhatsApp | Per client/mo |
| 3 | [Review Manager](lead-gen/review-manager/) | Browser, Email | Per client/mo |

### Content
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 4 | [Newsletter](content/newsletter/) | Email, Browser | Subscription |
| 5 | [Content Pipeline](content/content-pipeline/) | Email | Per deliverable |
| 6 | [Social Media Factory](content/social-media-factory/) | Browser, Email | Per client/mo |

### Monitoring
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 7 | [Website Monitor](monitoring/website-monitor/) | Telegram/Email | Per site/mo |
| 8 | [Price Tracker](monitoring/price-tracker/) | Telegram | Per tracker/mo |
| 9 | [Job Hunter](monitoring/job-hunter/) | Telegram/Email | Per user/mo |
| 10 | [SEO Tracker](monitoring/seo-tracker/) | Email | Per domain/mo |

### Dev Services
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 11 | [Dev Agency Bot](dev-services/dev-agency-bot/) | Telegram/Discord | Per project |
| 12 | [DevOps On-Call](dev-services/devops-oncall/) | Telegram/Slack | Per client/mo |
| 13 | [Codebase Audit](dev-services/codebase-audit/) | Email | Per audit |
| 14 | [PR Review Bot](dev-services/pr-review-bot/) | Discord/Slack | Per repo/mo |

### Customer Bots
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 15 | [Support Agent](customer-bots/support-agent/) | Multi-channel | Per client/mo |
| 16 | [Tutor](customer-bots/tutor/) | Telegram/Discord | Per student/mo |
| 17 | [Booking Bot](customer-bots/booking-bot/) | WhatsApp/Telegram | Per booking |

### Productivity
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 18 | [Executive Assistant](productivity/executive-assistant/) | iMessage/Telegram | Per user/mo |
| 19 | [Daily Briefing](productivity/daily-briefing/) | Email/Telegram | Per user/mo |

### Finance
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 20 | [Arbitrage Finder](finance/arbitrage-finder/) | Telegram | Per alert/mo |
| 21 | [Market Research](finance/market-research/) | Email | Per report |

### Automation
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 22 | [Data Scraping](automation/data-scraping/) | Email | Per job |
| 23 | [Form Filling](automation/form-filling/) | Telegram | Per submission |
| 24 | [Report Generator](automation/report-generator/) | Email | Per report |

### IoT
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 25 | [Smart Environment](iot/smart-environment/) | Telegram, GPIO | Per install |

### High-Value
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 26 | [RFP/Tender Sniper](high-value/rfp-sniper/) | Telegram, Browser | Per contractor/mo |
| 27 | [Chargeback Defense](high-value/chargeback-defense/) | Email, HTTP | Per chargeback |
| 28 | [Due Diligence](high-value/due-diligence/) | Email, Browser | Per report |
| 29 | [Grant Writer](high-value/grant-writer/) | Email, Browser | Per application |
| 30 | [Contract Scanner](high-value/contract-scanner/) | Telegram/WhatsApp | Per scan |

### Sales
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 31 | [Influencer Scout](sales/influencer-scout/) | Email, Browser | Per brand/mo |
| 32 | [Freelance Bidder](sales/freelance-bidder/) | Telegram, Browser | Per freelancer/mo |
| 33 | [Listing Localizer](sales/listing-localizer/) | Email, Browser | Per SKU/market |
| 34 | [Surplus Matchmaker](sales/surplus-matchmaker/) | Email, Browser | Commission |
| 35 | [Churn Predictor](sales/churn-predictor/) | Email, HTTP | Per SaaS client/mo |

### Intelligence
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 36 | [Competitor War Room](intelligence/competitor-warroom/) | Email, Browser | Per competitor/mo |
| 37 | [Trademark Watchdog](intelligence/trademark-watchdog/) | Email, Browser | Per brand/mo |
| 38 | [Reputation Sentinel](intelligence/reputation-sentinel/) | Email, Browser | Per business/mo |
| 39 | [License Tracker](intelligence/license-tracker/) | Email, Browser | Per business/mo |
| 40 | [Domain Hunter](intelligence/domain-hunter/) | Telegram, Browser | Per flip / mo |

### Professional
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 41 | [Pentest Recon](professional/pentest-recon/) | Email, Shell | Per client/mo |
| 42 | [Invoice Enforcer](professional/invoice-enforcer/) | Email/WhatsApp | Per freelancer/mo |
| 43 | [Literature Scout](professional/literature-scout/) | Email, Browser | Per researcher/mo |
| 44 | [Mystery Shopper](professional/mystery-shopper/) | Email, Browser | Per competitor/mo |

### Creative
| # | Workflow | Channels | Revenue Model |
|---|----------|----------|---------------|
| 45 | [Course Factory](creative/course-factory/) | Email, Browser | Per course |
| 46 | [Podcast Pipeline](creative/podcast-pipeline/) | Email, Browser | Per podcaster/mo |
| 47 | [Community Engine](creative/community-engine/) | Discord/Telegram | Per community/mo |
| 48 | [Event Concierge](creative/event-concierge/) | Email, Browser | Per event |
| 49 | [Rental Autopilot](creative/rental-autopilot/) | WhatsApp, Browser | Per property/mo |
| 50 | [Meal Planner](creative/meal-planner/) | Telegram, Browser | Per user/mo |

## Architecture

Each workflow template contains:

- **`config.toml`** — Configuration to merge into `~/.zeroclaw/config.toml`
- **`identity.md`** — System prompt defining the agent's personality
- **`prompts/`** — Task-specific prompts used by cron jobs and delegates
- **`README.md`** — Setup instructions, pricing model, required environment
- **`HEARTBEAT.md`** — (some workflows) Periodic monitoring tasks

## Categories (14)

- **[lead-gen/](lead-gen/)** — Prospecting, qualification, review management
- **[content/](content/)** — Newsletters, content pipelines, social media
- **[monitoring/](monitoring/)** — Website, price, job, SEO tracking
- **[dev-services/](dev-services/)** — Dev agencies, DevOps, audits, PR review
- **[customer-bots/](customer-bots/)** — Support, tutoring, booking
- **[productivity/](productivity/)** — Executive assistant, daily briefings
- **[finance/](finance/)** — Arbitrage, market research
- **[automation/](automation/)** — Scraping, form filling, reports
- **[iot/](iot/)** — Smart environment control
- **[high-value/](high-value/)** — RFP sniping, chargeback defense, due diligence, grants, contracts
- **[sales/](sales/)** — Influencer outreach, freelance bidding, localization, liquidation, churn
- **[intelligence/](intelligence/)** — Competitor monitoring, trademark, reputation, licensing, domains
- **[professional/](professional/)** — Pentest recon, invoice collection, literature, mystery shopping
- **[creative/](creative/)** — Courses, podcasts, communities, events, rentals, meal planning
