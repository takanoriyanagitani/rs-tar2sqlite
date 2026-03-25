#!/bin/bash

itar="./sample.d/input.tar"

gdir='/guest.d/read-write.d'
hdir='./out.d'

outname="./out.d/output.sqlite.db"

geninput(){
    echo creating example tar file...

    mkdir -p ./sample.d

    echo helo > ./sample.d/helo.txt
    echo wrld > ./sample.d/wrld.txt

    find ./sample.d -type f -name '*.txt' |
        sort |
        tar \
            --create \
            --file "${itar}" \
            --files-from - \
            --verbose
}

test -f "${itar}" || geninput

mkdir -p "${hdir}"

echo 'converting the tar to sqlite...'
cat "${itar}" |
    wasmtime \
        run \
        --dir "${hdir}::${gdir}" \
        ./rs-tar2sqlite.wasm \
        '/guest.d/read-write.d/output.sqlite.db' \
        --max-blob-size 3
ls -l "${outname}"
echo

echo showing the contents...
sqlite3 -A -tvf "${outname}"
