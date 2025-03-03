FROM rustlang/rust:nightly-slim AS planner
WORKDIR /app

RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rustlang/rust:nightly-slim AS cacher
WORKDIR /app

RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rustlang/rust:nightly-slim AS builder
WORKDIR /app

COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin bot

FROM debian:buster-slim

WORKDIR /home/site/wwwroot
RUN apt-get update && apt-get install -y ca-certificates openssh-server sudo && update-ca-certificates

# Prepare app
COPY --from=builder /app/target/release/bot /usr/local/bin

ENV ROCKET_PROFILE=release \
    ROCKET_PORT=80 \
    ROCKET_ADDRESS=0.0.0.0 \
    RUST_LOG=info \
    RUST_LOG_DISABLE_ANSI=1

# Enable SSH
RUN echo "root:Docker!" | chpasswd
COPY sshd_config /etc/ssh/

EXPOSE 80 2222

CMD service ssh restart ; bot