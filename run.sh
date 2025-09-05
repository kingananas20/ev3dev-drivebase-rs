#!/bin/sh

SERVER="robot"
REMOTE_DIR="~"
REMOTE_CMD="cd ~ && ./ev3"

# Copy binary (in the future install rsync on the ev3 and then copy using rsync)
rsync -ah --progress -z "$1" "$SERVER:$REMOTE_DIR"

# Execute on EV3
ssh "$SERVER" "$REMOTE_CMD"
