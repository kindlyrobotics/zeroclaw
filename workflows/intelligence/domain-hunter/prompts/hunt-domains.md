# Domain Hunting Task

Scan for high-value expiring domains.

## Steps

1. Browse configured domain auction/expiry sites
2. For each domain approaching expiry or in auction:
   a. Extract: domain name, TLD, expiry date, current bid, auction platform
   b. Check memory for already-evaluated domains (tag "domain:evaluated")
   c. Evaluate backlink profile (check via browser)
   d. Check for trademark conflicts
   e. Estimate resale value based on comparable sales
   f. Score using valuation criteria (1-10)
   g. Save evaluation to memory
3. For domains scoring 6+:
   - Send Telegram alert with full valuation details
4. For time-sensitive auctions (ending within 4 hours):
   - Send urgent alert regardless of score threshold
5. Daily: summarize domains evaluated, opportunities found
6. Weekly: portfolio performance report (if any domains owned)
