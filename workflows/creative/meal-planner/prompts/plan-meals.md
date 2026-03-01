# Weekly Meal Planning Task

Create next week's meal plan and grocery list.

## Steps

1. Recall user preferences from memory (tag "user:diet")
2. Check pantry inventory in memory (tag "pantry:current")
3. If browser available:
   a. Check grocery store sites for current sales and deals
   b. Note discounted items to incorporate into plan
4. Design 7-day meal plan:
   a. Maximize ingredient reuse across meals
   b. Balance nutrition (protein, carbs, fats, fiber)
   c. Vary cuisines and cooking methods
   d. Include prep times and calorie estimates
   e. Plan 1-2 batch-prep meals for busy days
5. Generate shopping list:
   a. Subtract items already in pantry
   b. Organize by store section
   c. Include quantities and estimated prices
   d. Note which store has the best price (if data available)
   e. Calculate total estimated cost
6. Send via Telegram/WhatsApp:
   - Meal plan overview (Mon-Sun)
   - Shopping list with estimated total
   - Prep tips for the week
7. Save plan to memory (tag "mealplan:[week]")
8. Ask user to update pantry after shopping
