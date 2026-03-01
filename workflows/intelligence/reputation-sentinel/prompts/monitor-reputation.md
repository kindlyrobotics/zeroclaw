# Reputation Monitoring Task

Check all platforms for new brand mentions and reviews.

## Steps

1. Recall brand details from memory (tag "brand:profile")
2. For each monitored platform:
   a. Search for brand name and common variations
   b. Extract new mentions: author, text, rating, date, URL
   c. Check memory for duplicates (tag "mention:seen")
   d. Classify sentiment
   e. Save to memory
3. For crisis-level mentions:
   - Send immediate alert via email with full context
   - Draft response following brand guidelines
4. For negative reviews:
   - Draft empathetic response
   - Queue for daily report
5. For positive reviews:
   - Flag as testimonial candidate (tag "testimonial:candidate")
6. Compile hourly summary if any new mentions found
7. Weekly: generate reputation scorecard with trends
