# Brand Monitoring Scan Task

Scan all channels for unauthorized brand use.

## Steps

1. Recall protected brand assets from memory (tag "brand:assets")
2. Marketplace scan:
   a. Search each marketplace for the brand name
   b. Identify listings not from authorized sellers
   c. Flag counterfeits (price anomalies, seller location, listing quality)
3. Domain scan:
   a. Check WHOIS for new registrations matching brand/typosquats
   b. Visit any suspicious domains
4. Social media scan:
   a. Search for accounts using the brand name
   b. Compare against known official accounts
5. For each infringement found:
   a. Capture evidence (screenshots, URLs, seller details)
   b. Classify severity
   c. Save to memory (tag "infringement:[platform]:[id]")
   d. Draft appropriate response document
6. Send daily report via email:
   - New infringements found
   - Status of pending takedowns
   - Summary statistics
