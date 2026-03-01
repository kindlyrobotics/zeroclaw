# Job Search Task

Search for new job postings and deliver curated results.

## Steps

1. Visit configured job boards using browser
2. Search for roles matching the criteria in your identity
3. For each new posting found:
   a. Extract: title, company, location, salary, URL, posting date
   b. Check memory — skip if already seen (tag "job:seen")
   c. Score the job 1-10 based on matching criteria
   d. Save to memory with tag "job:seen"
4. Filter to jobs scoring 7+
5. Send digest via Telegram/email:
   - Subject: "[Jobs] X new matches found"
   - Sort by score descending
   - Include: title, company, score, salary, link

## Digest Format

```
Job Digest — [DATE]
Found X new matches:

⭐ 9/10 — Senior Engineer @ Company ($180k)
Remote | Posted today
Why: Perfect role match, strong comp
Link: [URL]

⭐ 8/10 — Staff Engineer @ Company2 ($200k)
NYC | Posted yesterday
Why: Great growth, top company
Link: [URL]
```
