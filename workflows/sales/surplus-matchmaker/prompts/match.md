# Surplus Inventory Matching Task

Scan liquidation sites and match to buyer profiles.

## Steps

1. Recall registered buyer profiles from memory (tag "buyer:profile")
2. Browse configured liquidation marketplaces
3. For each new listing:
   a. Extract: category, condition, quantity, price, location, description
   b. Check memory for duplicates (tag "deal:scanned")
   c. Estimate resale value based on category and condition
   d. Calculate margin potential
   e. Match against buyer profiles
   f. Save to memory
4. For deals with 2x+ ROI matching a buyer:
   - Send deal alert to buyer via email
5. Compile daily summary: listings scanned, deals found, matches sent
6. Weekly: report on deal flow, acceptance rates, total commission earned
