#!/bin/sh

# Convert Windows-style path to Unix-style for sh
BINARY_PATH="$1"
BINARY_PATH_UNIX=$(echo "$BINARY_PATH" | sed 's|\\|/|g')

SERVER="jonas"
REMOTE_DIR="~/ev3"
REMOTE_CMD="cd ~/ev3 && ./ev3"

# 1. Copy binary
scp "$BINARY_PATH_UNIX" "$SERVER:$REMOTE_DIR"

# 2. Run commands on the server
ssh "$SERVER" "$REMOTE_CMD"
