#!/bin/sh

# Check if the pre-commit script exists
if [ ! -f ./scripts/pre-commit.sh ]; then
    echo "Error: pre-commit.sh not found in ./scripts directory."
    exit 1
fi

# Copy the pre-commit script to the Git hooks directory
cp ./scripts/pre-commit.sh .git/hooks/pre-commit

# Make sure the pre-commit script is executable
chmod +x .git/hooks/pre-commit

echo "Pre-commit hook installed."