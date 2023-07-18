#!/bin/sh

set -eu

/usr/local/rustup/toolchains/1.71.0-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/bin/llvm-profdata merge -o /pgo-data/merged.profdata /pgo-data
RUSTFLAGS="-Cprofile-use=/pgo-data/merged.profdata -Cllvm-args=-pgo-warn-missing-function" timeout ${TIMEOUT} cargo build --release -p typst-cli