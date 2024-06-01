FROM rust:latest AS builder

WORKDIR /membrs

COPY . .

RUN cargo build --bin server --release

RUN rm -rf target/release/build \
    && rm -rf target/release/deps \
    && rm -rf target/release/incremental \
    && rm -rf target/release/.fingerprint


FROM debian:latest as runner

RUN apt-get update \
    && apt-get install -y openssl ca-certificates

COPY --from=builder /membrs/target/release/server /usr/local/bin/server

EXPOSE 8000

CMD ["server"]
