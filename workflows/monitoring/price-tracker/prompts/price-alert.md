# Price Check Task

Check all tracked products for price changes.

## Steps

1. Recall tracked products from memory (tag "price:tracking")
2. For each product:
   a. Visit the product URL using browser
   b. Extract: current price, availability, product name
   c. Recall last price from memory (tag "price:[product-id]")
   d. Calculate change percentage
   e. Save current price to memory with timestamp
3. If any significant changes (>5% price change or stock status change):
   - Send Telegram alert with details
4. Log all checks to memory with tag "price:check"
5. Weekly: include a summary of all price trends
