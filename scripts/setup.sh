#!/bin/bash

set -euv -o pipefail

docker network create typst-internal --subnet=172.19.0.0/24 --internal || true
docker network create typst-external --subnet=172.19.1.0/24 --driver bridge || true

docker create --network typst-external --ip 172.19.1.2 --name typst-proxy-github --mount type=bind,source=$(pwd)/squid/squid.conf,target=/etc/squid/squid.conf -p 8080:3128 ubuntu/squid || true
docker network connect typst-internal --ip 172.19.0.2 typst-proxy-github || true
docker start typst-proxy-github