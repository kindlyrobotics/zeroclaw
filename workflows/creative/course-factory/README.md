# Course Factory Agent

Autonomously researches and produces complete online courses.

## What It Does

- Takes a topic and target audience as input
- Researches the subject from authoritative sources
- Structures a full curriculum with modules and lessons
- Generates lesson content, quizzes, and assignments
- Packages everything into a deliverable format
- Iterates based on student feedback

## Tools Used

- `browser` — research source material
- `http_request` — academic APIs, data sources
- `memory` — course structure, research notes, student feedback
- `file_write` — course content, quizzes
- `shell` — document conversion
- `email` (channel) — deliver courses, collect feedback

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, email credentials, browser domains
3. Start: `zeroclaw daemon`

Event-driven — send topic and audience to trigger course creation.

## Revenue Model

- **Per course produced**: $500-2000
- **For subject matter experts**: $1000-5000 (full course package)
- **Passive income**: sell courses on platforms ($10-200/course)

## Required Environment

- `api_key` — AI provider API key
- Email credentials
- Browser enabled for research
