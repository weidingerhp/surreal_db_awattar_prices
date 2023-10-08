FROM rust:1.72.0 as builder
WORKDIR /workdir
COPY src src
COPY Cargo.toml Cargo.toml
RUN cargo build --release --target-dir bin
FROM ubuntu:latest
RUN apt update & apt install -y extra-runtime-dependencies openssl & rm -rf /var/lib/apt/lists/*

COPY --from=builder /workdir/bin/release/awattar-prices /usr/local/bin/awattar-prices

ENV SURREALDB_URL localhost:8000
ENV SURREALDB_USER root
ENV SURREALDB_PASS root
ENV DEBUG_READ_DATA false

ENTRYPOINT [ "/usr/local/bin/awattar-prices" ]