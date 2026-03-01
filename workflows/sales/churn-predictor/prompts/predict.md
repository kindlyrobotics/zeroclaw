# Churn Prediction Task

Calculate health scores and trigger interventions.

## Steps

1. Pull customer usage data via HTTP API
2. Delegate analysis to `analyst` agent:
   - Login frequency trends (7-day, 30-day)
   - Feature usage patterns
   - Support ticket volume
   - Billing status
3. Calculate health score for each account (0-100)
4. Compare against previous scores in memory (tag "health:[account]")
5. For accounts that dropped 20+ points:
   - Flag as newly at-risk
6. Trigger appropriate win-back sequence:
   - At risk: delegate email to `outreach` agent
   - High risk: personal email + offer
   - Critical: alert human CSM via Telegram/Slack
7. Save all scores to memory
8. Weekly: generate churn dashboard report via email
   - Total accounts monitored
   - Score distribution (healthy/risk/critical)
   - Interventions sent and outcomes
   - Predicted churn for next 30 days
