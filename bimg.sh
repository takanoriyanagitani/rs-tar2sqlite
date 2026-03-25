#!/bin/sh

wtg=rs-tar2sqlite-wasi:0.1.0

which container |
    fgrep -q container || export PATH="/usr/local/bin:${PATH}"

build_apple_wasi() {
	container image inspect "${wtg}" |
		jq --raw-output '.[].name' |
		fgrep -q "$wtg" &&
		return

	echo building image "${wtg}"...

	container \
		build \
		--file ./Dockerfile \
		--platform linux/arm64 \
		--progress plain \
		--tag "${wtg}" \
		.
}

build_apple_wasi
