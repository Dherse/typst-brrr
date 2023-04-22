#!/bin/bash

set -euv -o pipefail

docker pull "rust:1.68.2-alpine3.17"
docker pull "alpine:3.17.3"
docker pull "ubuntu/squid:latest"

docker build -t "typst/clone" \
    -f "./docker-images/clone/dockerfile" \
    ./docker-images/clone

docker build -t "typst/fetch" \
    -f "./docker-images/fetch/dockerfile" \
    ./docker-images/fetch

docker build -t "typst/build" \
    -f "./docker-images/build/dockerfile" \
    ./docker-images/build

docker build -t "typst/bench-end-to-end" \
    -f "./docker-images/bench-end-to-end/dockerfile" \
    ./docker-images/bench-end-to-end