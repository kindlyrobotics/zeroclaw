# Price Comparison Task

Scan markets for arbitrage opportunities.

## Steps

1. Recall tracked products/categories from memory (tag "arb:tracking")
2. For each tracked item:
   a. Check price on Platform A using browser/HTTP
   b. Check price on Platform B using browser/HTTP
   c. Calculate spread and fees
   d. Save current prices to memory with timestamp
3. If any opportunity exceeds profit threshold:
   - Send Telegram alert with full details
   - Save opportunity to memory (tag "arb:opportunity")
4. Log all price data for trend analysis
5. Weekly: summarize best opportunities and average spreads

## Fee Calculation

Include in profit calculation:
- Platform selling fees (typically 10-15%)
- Payment processing (typically 2.9% + $0.30)
- Shipping costs (if physical goods)
- Tax implications (note but don't calculate)
