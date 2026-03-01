# Invoice Collection Task

Check for overdue invoices and send appropriate reminders.

## Steps

1. Recall all tracked invoices from memory (tag "invoice:*")
2. Calculate aging for each unpaid invoice
3. For each invoice that hits an escalation threshold:
   a. Check last communication date (avoid double-sending)
   b. Draft appropriate reminder based on aging tier
   c. Send via email
   d. Update memory with communication log
4. For invoices at 45+ days:
   - Send escalation alert to business owner
   - Include full communication history
5. Check email for any payment confirmations
   - Update invoice status in memory
6. Generate daily summary:
   - Total outstanding
   - Reminders sent today
   - Payments received
   - Aging breakdown (current / 1-7 / 8-14 / 15-30 / 30+)
