# Chargeback Defense Task

Check for new chargebacks and prepare rebuttals.

## Steps

1. Check email for chargeback notifications
2. For each new chargeback:
   a. Extract: order ID, amount, reason code, deadline
   b. Pull order details via HTTP request (Stripe/Shopify API)
   c. Gather delivery proof (tracking, screenshots)
   d. Search memory for customer communication history
   e. Compile evidence package
   f. Draft rebuttal following card network template for the reason code
   g. Save to memory (tag "chargeback:[order-id]")
3. Send evidence package via email
4. Set deadline reminder in memory
5. Check for approaching deadlines on open disputes
6. Weekly: generate win-rate report
