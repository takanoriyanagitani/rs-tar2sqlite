FROM rust:1.93.0-alpine3.23 AS builder
RUN rustup target add wasm32-wasip1
RUN apk update

WORKDIR /rs-tar2sqlite
RUN apk add clang21
RUN apk add cmake
RUN apk add ninja
RUN apk add python3
RUN apk add wasi-sdk
RUN apk add musl-dev
COPY --link ./Cargo.toml ./
COPY --link ./Cargo.lock ./
RUN mkdir src
RUN echo 'fn main(){}' > ./src/main.rs
RUN cargo check
ENV CFLAGS='-I/usr/share/wasi-sysroot/include/wasm32-wasip1'
RUN cargo build --target wasm32-wasip1 --profile release-wasi
COPY --link ./src/ ./src/
RUN touch ./src/lib.rs
RUN touch ./src/main.rs
RUN cargo build --target wasm32-wasip1 --profile release-wasi
RUN cp target/wasm32-wasip1/release-wasi/rs-tar2sqlite.wasm /usr/local/bin/

FROM golang:1.25.6-alpine3.23 AS demo
RUN go install -v github.com/tetratelabs/wazero/cmd/wazero@v1.11.0
COPY --link --from=builder /usr/local/bin/rs-tar2sqlite.wasm /
RUN apk add tar
RUN apk add sqlite
RUN apk add jq
RUN mkdir -p ./sample.d/content
RUN printf hw1 > ./sample.d/content/hw1.txt
RUN tar -C ./sample.d/content -cvf ./sample.d/input.tar .
RUN cat ./sample.d/input.tar | \
    wazero \
        run \
        -mount ./sample.d:/guest.d \
        /rs-tar2sqlite.wasm \
        -- \
        /guest.d/output.sqlar
RUN sqlite3 -json ./sample.d/output.sqlar 'SELECT * FROM sqlar' | jq '.[]'
RUN cp /rs-tar2sqlite.wasm /usr/local/bin/

FROM scratch
COPY --link --from=demo /usr/local/bin/rs-tar2sqlite.wasm /
