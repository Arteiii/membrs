FROM rust:latest AS base


WORKDIR /membrs
COPY Cargo.toml Cargo.lock ./

RUN USER=root cargo new --lib membrs_lib --vcs none
RUN USER=root cargo new --bin server --vcs none

WORKDIR /membrs/membrs_lib
COPY ./membrs_lib .

WORKDIR /membrs/server
COPY ./server/Cargo.toml .

WORKDIR /membrs
RUN cargo build --release --all
RUN rm ./membrs_lib/src/*.rs


FROM rust:latest AS builder

COPY ./server ./server

WORKDIR /membrs

RUN cargo build --release --bin server

RUN rm -rf target/release/build \
    && rm -rf target/release/deps \
    && rm -rf target/release/incremental \
    && rm -rf target/release/.fingerprint


FROM ubuntu:latest as runner

RUN apt-get update
RUN apt-get install -y openssl


COPY --from=builder /membrs/target/release/server /usr/local/bin/server

CMD ["server"]
