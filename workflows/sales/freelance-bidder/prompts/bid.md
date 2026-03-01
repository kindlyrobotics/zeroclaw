# Job Scanning and Bidding Task

Scan for new freelance jobs and prepare proposals.

## Steps

1. Recall operator profile from memory (tag "freelancer:profile")
2. Browse configured job boards for new postings
3. For each new job:
   a. Extract: title, description, budget, client info, proposals count, skills needed
   b. Check memory for duplicates (tag "job:seen")
   c. Score using evaluation criteria (1-10)
   d. Save to memory
4. For jobs scoring 6+:
   a. Analyze the client's profile and project details deeply
   b. Draft a personalized proposal
   c. Send to Telegram for approval:
      ```
      NEW JOB [Score: X/10]
      Title: [title]
      Budget: $[amount]
      Client: [rating, hire rate]
      Proposals: [count]

      Draft proposal:
      [your proposal]

      Reply "send" to submit or "skip" to pass
      ```
5. If approved, submit via browser
6. Log submission to memory (tag "bid:submitted")
