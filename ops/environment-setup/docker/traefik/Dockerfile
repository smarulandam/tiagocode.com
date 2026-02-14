ARG TRAEFIK_VERSION="v3.0.3"

FROM traefik:${TRAEFIK_VERSION}

LABEL author="Tiagocode <santiagomm1997@gmail.com>"
LABEL mantainer="Tiagocode <santiagomm1997@gmail.com>"

WORKDIR /data

RUN touch acme.json && chmod 600 acme.json

VOLUME /data