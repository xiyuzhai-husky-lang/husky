#!/bin/bash

set -e

# Create virtual environment
python3 -m venv .venv

source .venv/bin/activate

# Upgrade pip
pip install --upgrade pip

pip install pip-tools

# Check for requirements.txt and install from it
if [ -f "requirements.txt" ]; then
    echo "Installing packages from requirements.txt..."
    pip install -r requirements.txt
else
    echo "requirements.txt not found"
    exit 1
fi

echo "Environment setup complete! Activate it with:"
echo "  For bash/zsh: source .venv/bin/activate"
echo "  For fish:     source .venv/bin/activate.fish"
