# Tutor Agent

Interactive educational tutor with lesson planning and quizzes.

## What It Does

- Teaches subjects through conversational interactions
- Adapts to student's level and pace
- Creates lesson plans and study materials
- Quizzes students and tracks progress
- Provides personalized study recommendations

## Tools Used

- `telegram` or `discord` (channel) — student interaction
- `memory` — student progress, lesson plans, scores
- `file_read`/`file_write` — study materials

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key and channel token
3. Start: `zeroclaw daemon`

Event-driven — responds to student messages.

## Revenue Model

- **Per student/month**: $50-200/mo
- **Group tutoring**: $200-500/mo per group
- **Course package**: $100-500 per course

## Required Environment

- `api_key` — AI provider API key
- Telegram or Discord bot token
