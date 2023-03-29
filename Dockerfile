FROM rust:1.68.2-alpine as builder

RUN apk add rustup
RUN apk add build-base
RUN apk add binutils
WORKDIR /workdir
COPY ./ .
RUN cargo build  --release
RUN strip /workdir/target/release/redis_full_check


FROM alpine:3.17.2
WORKDIR /workdir
RUN apk add sqlite --no-cache
COPY --from=builder /workdir/target/release/redis_full_check ./
