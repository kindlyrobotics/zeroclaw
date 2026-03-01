# Website Check Task

Check all monitored sites and report changes.

## Steps

1. For each configured site:
   a. Make HTTP request — record status code and response time
   b. If browser available, visit the page and extract key content
   c. Recall last snapshot from memory (tag "monitor:[domain]")
   d. Compare current state with previous snapshot
   e. Detect changes: content, layout, status, response time delta
2. Save current snapshot to memory
3. If any changes or issues detected:
   - Send alert via Telegram/email
   - Format: [STATUS] domain.com — [what changed]
4. If all sites are healthy and unchanged, log silently to memory

## Alert Format

```
🔴 DOWN: example.com (503, timeout 15s)
🟡 CHANGED: client-site.com — pricing section updated
🟢 OK: all other sites healthy (avg 230ms)
```
