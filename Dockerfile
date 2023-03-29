FROM clux/muslrust:stable as builder


WORKDIR /workdir
COPY ./ .
RUN cargo build  --release


FROM alpine:3.17.2
WORKDIR /workdir
RUN apk add sqlite --no-cache
COPY --from=builder /workdir/target/release/redis_full_check ./
