#!/bin/bash

# Read Python version from .python-version file
if [ -f ".python-version" ]; then
    PYTHON_VERSION=$(cat .python-version)
else
    echo ".python-version file not found"
    exit 1
fi

# Check if pyenv is installed
if ! command -v pyenv &> /dev/null; then
    echo "pyenv not found. Please install pyenv first."
    exit 1
fi

# Install specific Python version if not already installed
if ! pyenv versions | grep -q $PYTHON_VERSION; then
    echo "Installing Python $PYTHON_VERSION..."
    pyenv install $PYTHON_VERSION
fi

# Set local Python version
pyenv local $PYTHON_VERSION

# Create virtual environment
pyenv exec python -m venv .venv

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
