# Literature Scouting Task

Search for new relevant papers and compile digest.

## Steps

1. Recall research profile from memory (tag "researcher:profile")
2. Search academic databases:
   a. arXiv — search by keywords and categories
   b. Semantic Scholar API — keyword and author search
   c. Google Scholar — broader search
   d. Check conference proceedings if in season
3. For each new paper found:
   a. Extract: title, authors, abstract, venue, date, URL
   b. Check memory for duplicates (tag "paper:seen")
   c. Score relevance (1-10)
   d. Generate structured summary
   e. Save to memory
4. Compile digest of papers scoring 6+
5. Send digest via email:
   - Top papers (8+) highlighted
   - Organized by subfield
   - Include summary for each
6. Weekly: generate field trend report
   - Emerging topics
   - Hot papers (most cited this week)
   - Potential collaborators
