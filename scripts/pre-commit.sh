#!/bin/bash

# Get a list of staged files
staged_files=$(git diff --cached --name-only --diff-filter=d | grep '\.rs$')

# If there are no staged .rs files, exit
if [ -z "$staged_files" ]; then
    exit 0
fi

# Run rustfmt on the staged files


# Convert the staged files to an array
IFS=$'\n' read -r -d '' -a staged_files_array <<<"$staged_files"

# Run cargo fix
cargo fix --allow-dirty --allow-staged

# Get a list of all modified files
modified_files=$(git ls-files -m | grep '\.rs$')
echo "Running rustfmt on staged files..."
for file in $staged_files; do
    rustfmt --edition 2021 "$file"
    git add "$file"
done

# Undo changes on files that are not in the staged files list
for file in $modified_files; do
    if ! [[ " ${staged_files_array[@]} " =~ " $file " ]]; then
        git checkout -- "$file"
    fi
done
