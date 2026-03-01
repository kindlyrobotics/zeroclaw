#!/usr/bin/env bash
set -euo pipefail

# ZeroClaw Workflow Setup
# Usage: cd workflows/<category>/<workflow> && bash ../../_base/setup.sh

WORKFLOW_DIR="$(cd "$(dirname "$0")/.." && pwd)"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ZEROCLAW_HOME="${ZEROCLAW_HOME:-$HOME/.zeroclaw}"
CONFIG_FILE="$ZEROCLAW_HOME/config.toml"
WORKSPACE="$ZEROCLAW_HOME/workspace"

echo "=== ZeroClaw Workflow Setup ==="
echo "Workflow: $(basename "$WORKFLOW_DIR")"
echo "Config:   $CONFIG_FILE"
echo ""

# Ensure ZeroClaw home exists
mkdir -p "$ZEROCLAW_HOME" "$WORKSPACE"

# Check for existing config
if [ -f "$CONFIG_FILE" ]; then
    echo "Existing config found at $CONFIG_FILE"
    echo "The workflow config.toml contains sections to merge."
    echo "Please review and merge manually:"
    echo "  $WORKFLOW_DIR/config.toml"
    echo ""
else
    echo "No existing config. Copying workflow config as starting point..."
    cp "$WORKFLOW_DIR/config.toml" "$CONFIG_FILE"
    echo "Created $CONFIG_FILE"
fi

# Copy identity.md
if [ -f "$WORKFLOW_DIR/identity.md" ]; then
    cp "$WORKFLOW_DIR/identity.md" "$WORKSPACE/identity.md"
    echo "Copied identity.md to $WORKSPACE/"
fi

# Copy HEARTBEAT.md if present
if [ -f "$WORKFLOW_DIR/HEARTBEAT.md" ]; then
    cp "$WORKFLOW_DIR/HEARTBEAT.md" "$WORKSPACE/HEARTBEAT.md"
    echo "Copied HEARTBEAT.md to $WORKSPACE/"
fi

# Copy prompts
if [ -d "$WORKFLOW_DIR/prompts" ]; then
    mkdir -p "$WORKSPACE/prompts"
    cp "$WORKFLOW_DIR/prompts/"*.md "$WORKSPACE/prompts/"
    echo "Copied prompts to $WORKSPACE/prompts/"
fi

echo ""
echo "=== Next Steps ==="
echo "1. Edit $CONFIG_FILE — set your API keys and channel tokens"
echo "2. Review the workflow README for cron job commands to add"
echo "3. Start the daemon: zeroclaw daemon"
echo ""

# Show required environment from README
if [ -f "$WORKFLOW_DIR/README.md" ]; then
    echo "--- From README.md ---"
    grep -A 20 "Required Environment\|Environment Variables\|API Keys" "$WORKFLOW_DIR/README.md" 2>/dev/null || true
fi
