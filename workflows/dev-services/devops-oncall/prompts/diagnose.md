# Health Check and Diagnosis Task

Run periodic health checks and handle any issues.

## Steps

1. Check each monitored endpoint:
   a. HTTP health check — record status code and response time
   b. Service status via shell (systemctl, docker ps, etc.)
   c. Resource usage (disk, memory, CPU)
2. Compare against thresholds and previous readings from memory
3. For each issue found:
   a. Classify severity: critical / warning / info
   b. Check runbook in memory for known remediation
   c. Attempt automated fix if available and safe
   d. Log action taken and result
4. If issues unresolved after remediation:
   - Send alert via Telegram/Slack with:
     - Service name and status
     - Error details and logs (last 20 lines)
     - Actions attempted
     - Recommended next steps
5. Save all check results to memory (tag "health:check")
6. If all healthy, log silently

## Alert Format

```
🔴 CRITICAL: [service] is DOWN
Time: [timestamp]
Error: [details]
Attempted: [actions taken]
Next: [recommendation]
```
