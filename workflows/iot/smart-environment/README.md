# Smart Environment Agent

IoT environment control using Raspberry Pi GPIO and sensors.

## What It Does

- Monitors environmental sensors (temperature, humidity, light, motion)
- Controls actuators (lights, fans, relays) based on rules
- Responds to voice/text commands via Telegram
- Runs automated routines on schedule (morning, evening, away)
- Tracks environmental data trends in memory

## Tools Used

- `peripherals` — GPIO read/write, sensor data
- `telegram` (channel) — commands and status updates
- `memory` — sensor history, user preferences, routines
- `shell` — system commands on the Pi

## Hardware Requirements

- Raspberry Pi (3B+ or newer) running ZeroClaw
- GPIO-connected sensors (DHT22, BMP280, PIR, etc.)
- Relay modules for actuator control
- Optional: camera module for visual monitoring

## Setup

1. Run `bash ../../_base/setup.sh`
2. Set API key, Telegram token
3. Configure peripheral pins in config.toml
4. Add cron jobs:

```bash
# Morning routine — 7am
zeroclaw cron add '0 7 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/control.md) Execute morning routine"'

# Evening routine — 8pm
zeroclaw cron add '0 20 * * *' 'agent -m "$(cat ~/.zeroclaw/workspace/prompts/control.md) Execute evening routine"'

# Sensor check — every 15 minutes
zeroclaw cron add '*/15 * * * *' 'agent -m "Read all sensors and log to memory. Alert if any readings are outside normal range."'
```

4. Start: `zeroclaw daemon`

## Revenue Model

- **Per installation**: $500-2000
- **Monthly monitoring/management**: $100-300/mo
- **Custom automation setup**: $200-1000

## Required Environment

- `api_key` — AI provider API key
- Telegram bot token
- Raspberry Pi with GPIO access
- Configured peripheral board in config
