FROM rust:latest AS builder

WORKDIR /membrs

COPY ./lib ./lib
COPY ./server ./server
COPY ./Cargo.toml ./Cargo.toml

WORKDIR /membrs/server

RUN cargo build --release --bin server

RUN rm -rf target/release/build \
    && rm -rf target/release/deps \
    && rm -rf target/release/incremental \
    && rm -rf target/release/.fingerprint


FROM ubuntu:trusty

RUN apt-get update
RUN apt-get install -y openssl


COPY --from=builder /membrs/target/release/server /usr/local/bin/server

CMD ["server"]
