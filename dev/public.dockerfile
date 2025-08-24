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

VOLUME "/var/www/html"

CMD ["caddy", "run", "-c", "/etc/caddy/Caddyfile"]