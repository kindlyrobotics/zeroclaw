# Codebase Audit Task

Perform a comprehensive audit of the target codebase.

## Steps

1. Clone or navigate to the target repository
2. Get an overview:
   - Language/framework detection
   - File structure and size
   - Dependency manifest (package.json, Cargo.toml, etc.)
3. Run automated checks:
   - Linting (if linter available)
   - Dependency audit (npm audit, cargo audit, etc.)
   - Search for common security patterns (hardcoded secrets, SQL injection, etc.)
4. Manual review of key files:
   - Entry points and routers
   - Authentication and authorization
   - Database queries and ORM usage
   - Error handling patterns
   - Configuration management
5. Compile findings into audit report
6. Send report via email
7. Save findings to memory (tag "audit:[repo-name]")

## Security Checks

Search for these patterns:
- Hardcoded API keys, passwords, tokens
- SQL string concatenation (injection risk)
- Unsanitized user input in HTML (XSS risk)
- Missing authentication on sensitive endpoints
- Overly permissive CORS settings
- Debug mode enabled in production config
