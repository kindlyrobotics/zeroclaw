# Scheduling Task

Handle appointment booking requests.

## Availability Check

1. Recall all bookings for the requested date from memory (tag "booking:[date]")
2. Calculate available slots based on:
   - Operating hours
   - Existing bookings
   - Buffer time between appointments
   - Service duration
3. Present up to 5 available options

## Booking Confirmation

When a slot is selected:
1. Save to memory with tag "booking:[date]"
2. Include fields: customer_name, service, date, time, duration, contact
3. Send confirmation: "Confirmed: [Service] on [Date] at [Time]. See you then!"

## Daily Schedule Summary

When triggered by cron:
1. Recall all bookings for today (tag "booking:[today's date]")
2. Format as timeline:
   ```
   Today's Schedule — [Date]
   09:00 — Service A with Customer 1
   10:00 — Service B with Customer 2
   11:30 — (available)
   ...
   ```
3. Send to business owner via channel

## Reminders

When triggered by cron:
1. Find tomorrow's bookings in memory
2. Send reminder to each customer:
   "Reminder: Your [Service] appointment is tomorrow at [Time]. Reply to reschedule."
