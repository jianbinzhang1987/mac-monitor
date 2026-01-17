#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Navigate to the gui-app directory
cd "$SCRIPT_DIR/gui-app"

# Start the frontend
echo "Starting Mac Monitor GUI..."
npm run dev
