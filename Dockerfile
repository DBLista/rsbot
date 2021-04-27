FROM rust:latest as builder

WORKDIR /usr/src/bot
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

WORKDIR /home/site/wwwroot
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
COPY --from=builder /usr/local/cargo/bin/bot /usr/local/bin/bot

CMD ["bot"]
