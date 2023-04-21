#!/bin/sh

ping google.com -c 1

# Set ownership
git config --global --add safe.directory /data

# Clone
git clone $REPO_URL /data --depth 1

# Checkout
git checkout $COMMIT