#! /bin/bash
# Enable strict error handling - script will exit on any error
set -e

MAX_ATTEMPTS=3
attempt=1

# Invalidate any cached credentials
sudo -k

while [ $attempt -le $MAX_ATTEMPTS ]; do
    read -s -p "Please enter your password ($attempt/$MAX_ATTEMPTS): " password
    echo

    if [ -z "$password" ]; then
        echo "Error: Password cannot be empty"
    elif echo "$password" | sudo -S -p '' true 2>/dev/null; then
        # Valid password, exit loop
        break
    else
        echo "Error: Invalid password"
    fi

    if [ $attempt -eq $MAX_ATTEMPTS ]; then
        echo "Maximum password attempts exceeded."
        exit 1
    fi

    ((attempt++))
done

unset password  # Clear password from memory for security

# Entity name validation - one attempt
read -p "Enter the LLM entity name: " entity_name
if [ "${entity_name}" != "OPENAI" ]; then
    echo "Incorrect entity name. Exiting."
    exit 1
fi

# Cap validation - one attempt
read -p "Enter the API request cap: " api_cap
if [ "${api_cap}" != "500" ]; then
    echo "Incorrect API request cap. Exiting."
    exit 1
fi

# Check if a command argument was provided
if [ -z "$1" ]; then
    echo "Error: No command provided"
    echo "Usage: $0 <command>"
    exit 1
fi

# Verify that OPENAI_API_KEY is not set initially
if [ ! -z "${OPENAI_API_KEY}" ]; then
    echo "Warning: OPENAI_API_KEY is already set"
fi

# Verify that the OpenAI environment configuration file exists
if [ ! -f "$HOME/.llms/openai/env" ]; then
    echo "Error: OpenAI environment file not found at $HOME/.llms/openai/env"
    echo "Please ensure the environment file exists and contains required OpenAI configurations"
    exit 1
fi

# Verify that ENABLE_OPENAI_API_CALLING is not set initially
if [ ! -z "${ENABLE_OPENAI_API_CALLING}" ]; then
    echo "Warning: ENABLE_OPENAI_API_CALLING is already set to: ${ENABLE_OPENAI_API_CALLING}"
fi

# Store the provided command argument
COMMAND="$1"

# Source the OpenAI environment variables
. "$HOME/.llms/openai/env"

# Verify that OPENAI_API_KEY is now set
if [ -z "${OPENAI_API_KEY}" ]; then
    echo "Error: OPENAI_API_KEY is not set after sourcing environment file"
    exit 1
fi

# Set API rate limit to 500 requests
export ENABLE_OPENAI_API_CALLING=500

# Execute the provided command in a new shell
bash -c "$COMMAND"