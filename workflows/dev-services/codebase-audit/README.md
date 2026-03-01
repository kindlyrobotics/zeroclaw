# Codebase Audit Agent

Automated code quality and security audits.

## What It Does

- Clones or reads a codebase
- Analyzes code quality, patterns, and potential issues
- Checks for security vulnerabilities and bad practices
- Generates a comprehensive audit report
- Delivers via email

## Tools Used

- `shell` — git clone, run linters, static analysis
- `file_read` — code inspection
- `memory` — audit templates and past findings
- `email` (channel) — deliver audit reports

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key and email credentials
3. Start: `zeroclaw daemon`

Event-driven — send audit requests via email or messaging.

## Revenue Model

- **Per audit**: $500-3000
- **Monthly retainer**: $1000-5000/mo (continuous auditing)
- **Security-focused audit**: $2000-10000

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Git access to target repositories
