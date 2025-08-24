FROM node:alpine AS builder
LABEL authors="jcake"
LABEL name="Ticket"
LABEL description="A simple Ticket system"

ARG BUILD

COPY "./app" "/app/"
WORKDIR "/app"

RUN npm install
RUN npm run build

FROM caddy:alpine

COPY <<"EOT" "/etc/caddy/Caddyfile"
localhost {
    root * /var/www/html
    file_server
    tls internal
}
EOT

EXPOSE 80 443

RUN mkdir -p /var/www/html
COPY --from=builder "/app/build" "/var/www/html"

VOLUME "/var/www/html"

CMD ["caddy", "run", "-c", "/etc/caddy/Caddyfile"]