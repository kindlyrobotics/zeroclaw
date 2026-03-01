# Contract Analysis Task

Analyze an uploaded contract for risks and red flags.

## Steps

1. Receive the contract document via channel message
2. Extract text using shell tools (pdftotext, pandoc) or file_read
3. Identify the contract type (SaaS, employment, NDA, freelance, lease, etc.)
4. Scan each section for red flag patterns:
   a. Auto-renewal and cancellation terms
   b. Liability and indemnification clauses
   c. IP ownership and work product rights
   d. Non-compete and non-solicitation scope
   e. Payment terms and fee structures
   f. Termination rights and notice periods
   g. Dispute resolution mechanisms
   h. Data rights and confidentiality
5. Score overall risk level
6. Compile analysis in the format defined in identity
7. Send analysis via channel
8. Save analysis summary to memory (tag "contract:analyzed")
