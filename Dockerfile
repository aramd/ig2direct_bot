FROM rust:latest AS builder

RUN cargo install sccache
ENV RUSTC_WRAPPER=sccache

WORKDIR /usr/src/ig2direct_bot
COPY Cargo.toml Secrets.toml ./
COPY src ./src
RUN cargo build --release

FROM debian:12-slim

RUN apt-get update  \
    && apt-get install -y openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/ig2direct_bot/Secrets.toml /app
COPY --from=builder /usr/src/ig2direct_bot/target/release/ig2direct-bot /usr/local/bin/ig2direct_bot

CMD ["/usr/local/bin/ig2direct_bot"]
