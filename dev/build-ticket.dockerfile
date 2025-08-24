FROM rust:latest AS builder
LABEL authors="jcake"
LABEL name="Ticket"
LABEL description="A simple Ticket system"

RUN apt update && apt upgrade -y && apt install -y jq

ARG BUILD

COPY "." "/app/"
WORKDIR "/app"

RUN mkdir -p target/out
RUN cargo build $BUILD --all --bins --message-format json | jq -r '.|select(.reason == "compiler-artifact" and .executable).executable' | xargs -I {} cp {} target/out/

FROM debian:trixie-slim

RUN apt update && apt upgrade -y && apt install -y openssl libssl-dev
RUN mkdir -p /etc/ticket

COPY --from=builder "/app/target/out/*" "/usr/bin"
COPY --from=builder "/app/dev/docker.toml" "/etc/ticket/config.toml"

CMD ["/usr/bin/ticket"]