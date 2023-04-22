#!/bin/sh

set -eu

# Set ownership
git config --global --add safe.directory /typster

# Set the default branch name to avoid stderr output
git config --global init.defaultBranch main

# go into the directory
cd /typster

# Initialize the repository
git init

# Add the remote origin
git remote add origin $REPO_URL

# Fetch the commit
git fetch --depth 1 origin $COMMIT

# Checkout
git checkout $COMMIT