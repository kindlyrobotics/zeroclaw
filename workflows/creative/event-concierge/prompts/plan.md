# Event Planning Task

Plan and coordinate an event from requirements.

## Steps

1. Parse event requirements: type, date, guests, budget, location, preferences
2. Venue search:
   a. Browse venue sites using browser
   b. Filter by capacity, location, availability
   c. Compare pricing for top 3-5 options
   d. Present options via email with pros/cons
3. Vendor sourcing (after venue confirmed):
   a. Research vendors for each needed service
   b. Draft outreach emails requesting quotes
   c. Send and track vendor responses
4. Create event budget tracker
5. Set up RSVP tracking in memory (tag "event:[name]:rsvp")
6. Schedule reminder cron:
   - 2 weeks before: RSVP reminder to unconfirmed guests
   - 1 week before: logistics email to confirmed guests
   - 48 hours before: confirm all vendors
   - Day of: send run sheet to all parties
7. Save all event details to memory (tag "event:[name]")
