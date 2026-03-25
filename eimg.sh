#!/bin/sh

wtg=rs-tar2sqlite-wasi:0.1.0
wname=rs-tar2sqlite.wasm

which container |
    fgrep -q container || export PATH="/usr/local/bin:${PATH}"

container \
    image \
    save \
    "${wtg}" |
    tar --list --verbose |
    sort -nk 5,5 |
    tail -1 |
    sed 's/^.*blobs/blobs/' |
    while read line; do
        container image save "${wtg}" |
            tar x -O "${line}" |
            zcat |
            tar x -O "${wname}" |
            dd if=/dev/stdin of="${wname}" bs=1048576 status=none
    done

ls -l "${wname}"
