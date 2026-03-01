# SEO Rank Check Task

Check keyword rankings and generate report.

## Steps

1. Recall tracked keywords from memory (tag "seo:keywords")
2. For each keyword:
   a. Search Google using browser
   b. Find the target domain's position in results (top 100)
   c. Note competitor positions
   d. Recall last position from memory (tag "seo:rank:[keyword]")
   e. Calculate position change
   f. Save current ranking to memory with timestamp
3. Compile daily report:
   - Keywords that improved (green)
   - Keywords that dropped (red)
   - Keywords unchanged (neutral)
4. On Fridays, include weekly summary with trends
5. Send report via email

## Alert Conditions

Send immediate alert (not just daily report) if:
- Any keyword drops 5+ positions in a day
- Target domain disappears from top 100 for a keyword
- A competitor overtakes target for a key term
