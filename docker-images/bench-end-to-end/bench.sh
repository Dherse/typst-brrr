#!/bin/sh

hyperfine --runs ${RUNS} \
    --warmup ${WARMUPS} \
    -N \
    --parameter-list file ${FILE_LIST} \
    --export-json "/data/run.json" \
    -- "/binary/typst compile {file} /dev/null"