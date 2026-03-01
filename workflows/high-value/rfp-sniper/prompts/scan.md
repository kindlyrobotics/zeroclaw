# RFP Opportunity Scan

Search procurement portals for new opportunities.

## Steps

1. Visit configured procurement portals using browser
2. Search using configured keywords and NAICS codes
3. For each new opportunity found:
   a. Extract: title, agency, value, deadline, set-aside status, NAICS, URL
   b. Check memory for duplicates (tag "rfp:scanned")
   c. Score using the criteria in identity (1-10)
   d. Save to memory with tag "rfp:scanned"
4. For opportunities scoring 6+:
   - Send alert via channel
5. For opportunities scoring 7+:
   - Download and parse the RFP document
   - Draft a brief proposal outline
   - Save outline to workspace
6. Check for approaching deadlines on existing opportunities
   - Alert at 7-day, 3-day, and 1-day marks
