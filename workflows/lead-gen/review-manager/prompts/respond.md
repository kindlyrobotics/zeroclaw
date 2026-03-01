# Review Check Task

Scan for new reviews and draft responses.

## Steps

1. Use the browser to check configured review sites for new reviews
2. For each new review found:
   - Extract: reviewer name, rating, text, date, platform
   - Analyze sentiment (positive/neutral/negative)
   - Save to memory with tag "review:new"
3. Skip reviews already in memory
4. For each new review, draft a response following these rules:
   - Positive (4-5 stars): Thank them, mention specifics they praised
   - Mixed (3 stars): Acknowledge positive, address concerns
   - Negative (1-2 stars): Empathize, apologize, offer to resolve offline
5. Send an email with all new reviews and draft responses
   - Subject: "[Reviews] X new reviews found — Y positive, Z negative"
   - Include each review with its draft response
   - Flag any negative reviews as urgent

## Output Format

For each review in the email:
```
Platform: [Google/Yelp/etc]
Rating: [X/5]
Reviewer: [Name]
Review: [Text]
Sentiment: [Positive/Neutral/Negative]

Draft Response:
[Your drafted response]

---
```
