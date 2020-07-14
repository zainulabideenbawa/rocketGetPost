FROM rustlang/rust:nightly-alpine as builder
WORKDIR /code
RUN apk update \
    && apk add build-base openssl-dev zlib-dev \
    && rm -rf /var/cache/apk/*

COPY . .
RUN cargo build --release

FROM alpine:latest

ENV ROCKET_ENV=production \
    ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=3000 \
    ROCKET_LOGS=critical

EXPOSE  3000

COPY --from=builder /code/target/release/rocketGetPost /usr/local/bin/rocketGetPost

CMD rocketGetPost


