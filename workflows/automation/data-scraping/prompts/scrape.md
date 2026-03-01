# Data Scraping Task

Execute configured scraping jobs and deliver results.

## Steps

1. Recall scraping configuration from memory (tag "scrape:config")
   - Target URLs and data fields to extract
   - Output format preferences
   - Delivery instructions
2. For each target:
   a. Navigate to the URL using browser
   b. Extract specified data fields
   c. Handle pagination if needed (follow next page links)
   d. Normalize extracted data
   e. Compare with previous scrape results in memory
   f. Save raw data to workspace file
3. Compile results:
   - Total records extracted
   - New records since last scrape
   - Changed records (with diff)
   - Failed extractions (with error details)
4. Send results via email
5. Update scrape state in memory (tag "scrape:[domain]")

## Error Handling

- If a page fails to load: retry once after 10 seconds
- If layout changed: report the change, attempt best-effort extraction
- If blocked: stop immediately, report to user, do not retry
