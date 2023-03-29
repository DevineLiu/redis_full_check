FROM rust:1.68.2-alpine3.17 as builder

RUN apk add build-base


WORKDIR /workdir
COPY ./ .
RUN cargo build  --release
RUN strip /workdir/target/release/redis_full_check

FROM alpine:3.17.2
WORKDIR /workdir
RUN apk add sqlite --no-cache
COPY --from=builder /workdir/target/release/redis_full_check ./
