#!/bin/sh

set -eu

# Set ownership
git config --global --add safe.directory /typster

# Set the default branch name to avoid stderr output
git config --global init.defaultBranch main

# go into the directory
cd /typster

# Initialize the repository
git init || true

# Add the remote origin
timeout ${TIMEOUT} git remote add origin $REPO_URL || true

# Fetch the commit
timeout ${TIMEOUT} git fetch --depth 1 origin $COMMIT || true

# Checkout
timeout ${TIMEOUT} git checkout $COMMIT