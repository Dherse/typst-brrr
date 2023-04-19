#!/bin/sh

# Set ownership
git config --global --add safe.directory /data

# Clone
git clone $REPO_URL /data

# Checkout
git checkout $COMMIT