# Syntax: docker/dockerfile:1
FROM rust:1.75-slim AS builder

WORKDIR /usr/src/repomd
COPY . .
RUN cargo build --release --bin repomd-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/repomd/target/release/repomd-api /usr/local/bin/
CMD ["repomd-api"]
EXPOSE 8080
