# Environment Control Task

Read sensors and manage actuators based on conditions and routines.

## Sensor Reading

1. Read all connected sensors via peripheral tools
2. Log readings to memory with timestamp (tag "sensor:[type]:[date]")
3. Compare against normal ranges defined in identity
4. If any reading is outside normal range:
   - Check if it's been outside range for 15+ minutes (recall from memory)
   - If yes: send Telegram alert with current reading and trend
   - If no: note the anomaly in memory, continue monitoring

## Actuator Control

When commanded (via message or routine):
1. Parse the command: what to control and desired state
2. Execute the control via peripheral tools
3. Verify the action took effect (re-read sensor if applicable)
4. Confirm via Telegram: "Done: [action] — current state: [reading]"

## Routine Execution

When a routine is triggered (morning/evening/away):
1. Execute the actions defined in identity for that routine
2. Read all sensors after actions complete
3. Send status summary via Telegram:

```
Routine: [Morning/Evening/Away] — Complete
Temperature: XX°C (target: YY°C)
Humidity: XX%
Lights: [ON/OFF/DIM]
Fan: [ON/OFF]
Motion: [clear/detected]
```

## Status Report

When asked for status:
- Read all sensors
- Report current actuator states
- Include 24h trend summary from memory
- Note any ongoing alerts or anomalies
