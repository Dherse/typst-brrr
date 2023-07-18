#!/bin/sh

set -eu

timeout ${TIMEOUT} cargo build --release -p typst-cli