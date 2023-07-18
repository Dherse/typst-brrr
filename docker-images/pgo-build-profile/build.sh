#!/bin/sh

set -eu

RUSTFLAGS="-Cprofile-generate=/pgo-data" time timeout ${TIMEOUT} cargo build --release -p typst-cli