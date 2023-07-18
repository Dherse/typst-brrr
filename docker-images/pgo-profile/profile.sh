#!/bin/sh

set -eu

IFS=','; for file in ${FILE_LIST} ; do
    echo 'Building ' ${file}
    /typster/target/release/typst compile --font-path $(dirname $file) ${file} /dev/null
done