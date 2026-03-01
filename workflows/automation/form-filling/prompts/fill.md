# Form Filling Task

Fill and submit a web form with provided data.

## Steps

1. Parse the request for:
   - Target URL
   - Form data (field name → value pairs)
   - Any special instructions
2. Navigate to the target URL using browser
3. Identify all form fields on the page
4. For each provided data field:
   a. Locate the matching form field
   b. Enter the value
   c. Verify it was entered correctly
5. Check for any required fields that are empty
   - If missing data: report back and wait for input
6. Take a pre-submission screenshot
7. Submit the form
8. Capture the confirmation/error response
9. Report results via Telegram:
   - Success: confirmation number/message
   - Failure: error details and screenshot
10. Save submission to memory (tag "form:submitted")
