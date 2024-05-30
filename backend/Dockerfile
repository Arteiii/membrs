FROM rust:latest AS builder

WORKDIR /membrs

COPY . .

RUN cargo build --release --bin server

RUN rm -rf target/release/build \
    && rm -rf target/release/deps \
    && rm -rf target/release/incremental \
    && rm -rf target/release/.fingerprint


FROM ubuntu:latest as runner

RUN apt-get update \
    && apt-get install -y openssl

COPY --from=builder /membrs/target/release/server /usr/local/bin/server

EXPOSE 8000

CMD ["server"]
