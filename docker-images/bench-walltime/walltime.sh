#!/bin/sh

set -eu

IFS=','; for file in ${FILE_LIST} ; do
    timeout ${TIMEOUT} /bin/cobench \
        measure \
        -n ${RUNS} \
        -w ${WARMUPS} \
        -S ${WORK} \
        -s ${SLEEP} \
        --export-path /data/$(basename $file .typ).json \
        "/typster/target/release/typst compile --font-path $(dirname $file) ${file} /dev/null"
done