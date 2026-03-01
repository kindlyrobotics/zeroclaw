# License Renewal Check Task

Review all tracked licenses for upcoming deadlines.

## Steps

1. Recall all tracked licenses from memory (tag "license:*")
2. For each license:
   a. Calculate days until expiration
   b. Check if it falls within an alert threshold (90/60/30/14/7 days)
   c. If approaching renewal:
      - Check the licensing portal for any requirement changes
      - Verify current renewal requirements still match memory
      - Pre-fill renewal application if possible
3. Send alerts for licenses within threshold:
   - Include: license type, number, expiration date, days remaining
   - Include: renewal requirements checklist
   - Include: portal URL for manual renewal
4. Check for any newly expired licenses — send critical alert
5. Monthly: generate compliance status report
6. Quarterly: verify all requirements against portal (full audit)
