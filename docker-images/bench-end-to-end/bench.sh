#!/bin/sh

set -eu

IFS=','; for file in ${FILE_LIST} ; do
    /bin/cobench \
        -n ${RUNS} \
        -w ${WARMUPS} \
        --export-json /data/$(basename $file .typ).json \
        "/typster/target/release/typst --font-path $(dirname $file) compile ${file} /dev/null"
done