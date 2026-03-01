# Daily Briefing Task

Compile and deliver today's personalized briefing.

## Steps

1. Recall user preferences from memory (tag "user:preferences")
2. Research today's news:
   a. Browse 3-5 news sources relevant to user's interests
   b. Check for major breaking news
   c. Look for industry-specific updates
3. If HTTP enabled, fetch weather data for user's location
4. Recall recent briefings from memory to avoid repetition
5. Compose briefing:

## Briefing Format

```
Good morning — [Day, Date]

TOP NEWS
- [Headline]: [1-2 sentence summary]
- [Headline]: [1-2 sentence summary]
- [Headline]: [1-2 sentence summary]

INDUSTRY
- [Update 1]
- [Update 2]

WEATHER
[Location]: [High/Low], [conditions]

TODAY'S FOCUS: [Topic]
[2-3 paragraph deep dive on one interesting topic]
```

6. Send via configured channel
7. Save briefing to memory (tag "briefing:[date]")
