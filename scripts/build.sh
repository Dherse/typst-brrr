#!/bin/bash

set -euv -o pipefail

docker pull "rust:1.71-alpine3.18"
docker pull "alpine:3.18"
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

docker build -t "typst/bench-walltime" \
    -f "./docker-images/bench-walltime/dockerfile" \
    ./docker-images/bench-walltime

docker build -t "typst/pgo-build" \
    -f "./docker-images/pgo-build/dockerfile" \
    ./docker-images/pgo-build

docker build -t "typst/pgo-build-profile" \
    -f "./docker-images/pgo-build-profile/dockerfile" \
    ./docker-images/pgo-build-profile

docker build -t "typst/pgo-profile" \
    -f "./docker-images/pgo-profile/dockerfile" \
    ./docker-images/pgo-profile