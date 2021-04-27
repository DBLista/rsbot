FROM rustlang/rust:nightly-slim as builder

WORKDIR /usr/src/bot
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

WORKDIR /home/site/wwwroot
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
COPY --from=builder /usr/local/cargo/bin/bot /usr/local/bin/bot

ENV ROCKET_PORT=8000
EXPOSE 8000

CMD ["bot"]
