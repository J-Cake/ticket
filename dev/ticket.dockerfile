FROM debian:trixie-slim

RUN apt update && apt upgrade -y && apt install -y openssl libssl-dev
RUN mkdir -p /etc/ticket

#COPY --from=builder "/app/target/out/*" "/usr/bin"
#COPY --from=builder "/app/dev/docker.toml" "/etc/ticket/config.toml"

VOLUME "/usr/bin/ticket"
VOLUME "/etc/ticket/config.toml"

CMD ["/usr/bin/ticket"]