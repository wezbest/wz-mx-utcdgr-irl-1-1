#!/bin/bash

# 1. Ensure the uv environment is ready
echo "Checking Python environment..."
uv sync

# 2. Get the absolute path to your uv python binary
PYTHON_PATH=$(pwd)/.venv/bin/python3

# 3. FIX: Create a shared secret for the Task Runner
# This is the "auth token" n8n is complaining about
export N8N_RUNNERS_AUTH_TOKEN="codespaces-secret-12345"

# 4. Core n8n Configuration for Codespaces
export N8N_HOST=0.0.0.0
export N8N_PORT=5678
export N8N_PROTOCOL=https
export NODE_ENV=production

# 5. URL and Origin Fixes
export WEBHOOK_URL=https://special-yodel-v6p6vgxvr472wp-5678.app.github.dev/
export N8N_SKIP_ORIGIN_CHECK=true
export N8N_CORS_ALLOWED_ORIGINS=*
export N8N_EXPRESS_TRUST_PROXY=true

# 6. Stability Fixes
export N8N_PUSH_BACKEND=sse

# 7. Python Task Runner Fixes
export N8N_BLOCK_PYTHON_MODE=external
export N8N_PYTHON_CONTAINER_MAIN_PATH=$PYTHON_PATH

echo "Starting n8n with Python at: $PYTHON_PATH"
echo "Shared Secret Set: $N8N_RUNNERS_AUTH_TOKEN"

# 8. Launch n8n
bunx n8n start
