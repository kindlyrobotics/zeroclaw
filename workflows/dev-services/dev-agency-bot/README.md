# Dev Agency Bot

Conversational development agent that turns specs into code.

## What It Does

- Receives project specs and feature requests via messaging
- Breaks down requirements into tasks
- Writes code using shell and file tools
- Delivers completed code via email or messaging
- Uses delegate agents for specialized tasks (frontend, backend, testing)

## Tools Used

- `shell` — run commands, git operations, build tools
- `file_read`/`file_write` — code generation
- `telegram` or `discord` (channel) — receive specs, deliver updates
- `delegate` — specialized sub-agents
- `memory` — project context and history

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key and channel tokens
3. Start: `zeroclaw daemon`

Event-driven — responds to incoming messages with specs.

## Revenue Model

- **Per project**: $500-5000
- **Monthly retainer**: $2000-10000/mo
- **Per feature**: $200-1000

## Required Environment

- `api_key` — AI provider API key
- Telegram or Discord bot token
