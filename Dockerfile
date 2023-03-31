FROM alpine:latest as builder

RUN apk add rustup
RUN rustup-init -y 
RUN apk add build-base

ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install 1.68.2

COPY ./ .
RUN  cargo build  --release


FROM alpine:3.17.2
WORKDIR /workdir
RUN apk add sqlite --no-cache
COPY --from=builder /workdir/target/release/redis_full_check ./
