
FROM rust:1.61.0-alpine as builder


RUN apk add rustup
RUN apk add build-base
RUN apk add binutils
RUN rustup-init -y 
ENV PATH=/root/.cargo/bin:"$PATH"
RUN rustup default nightly-2023-03-24

WORKDIR /workdir
COPY ./ .
# RUN source /root/.cargo/env
RUN cargo build  --release


FROM alpine:3.17.2
WORKDIR /workdir
RUN apk add sqlite --no-cache
COPY --from=builder /workdir/target/release/redis_full_check ./
