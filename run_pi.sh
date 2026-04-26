#!/bin/bash

# Load environment variables from .env if it exists
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
fi

# Default values if variables are not defined
EXAMPLE=${1:-basic}
USER=${PI_USER:-$(whoami)}
HOST=${PI_HOST:-localhost}
REMOTE_PATH=${PI_PATH:-~/ferrum-deploy}

cross build --example "$EXAMPLE" --target aarch64-unknown-linux-gnu 2>&1

if [ $? -eq 0 ]; then
    ssh "$USER@$HOST" "mkdir -p $REMOTE_PATH"
    scp "target/aarch64-unknown-linux-gnu/debug/examples/$EXAMPLE" "$USER@$HOST:$REMOTE_PATH"
    ssh "$USER@$HOST" "$REMOTE_PATH/$EXAMPLE"
fi