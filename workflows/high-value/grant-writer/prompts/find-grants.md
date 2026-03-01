# Grant Discovery Task

Scan portals for matching grant opportunities.

## Steps

1. Recall organization profile from memory (tag "org:profile")
2. Browse configured grant portals for new opportunities
3. For each opportunity found:
   a. Extract: title, funder, amount, deadline, eligibility, focus area
   b. Check memory for duplicates (tag "grant:scanned")
   c. Score eligibility match (1-10)
   d. Save to memory
4. For opportunities scoring 7+:
   - Download the full solicitation document
   - Extract specific requirements and page limits
   - Draft a brief application outline
   - Send alert via email with opportunity details and outline
5. Check approaching deadlines on tracked opportunities
6. Weekly: summarize pipeline — opportunities found, applications in progress, deadlines
