ARG PHP_VERSION=8.4-fpm-alpine

FROM php:$PHP_VERSION

ARG TIAGOCODE_USER_ID=1000
ARG TIAGOCODE_GROUP_ID=1000

RUN set -ex; \
    apk --update --no-cache add \
        shadow \
        bash \
        nano \
        curl \
        sudo \
        git \
        patch \
        postgresql17-client \
        su-exec

RUN set -ex; \
    existing_group=$(getent group "${TIAGOCODE_GROUP_ID}" | cut -d: -f1); \
    if [[ -n "${existing_group}" ]]; then delgroup "${existing_group}"; fi; \
    existing_user=$(getent passwd "${TIAGOCODE_USER_ID}" | cut -d: -f1); \
    if [[ -n "${existing_user}" ]]; then deluser "${existing_user}"; fi; \
    groupadd -g "${TIAGOCODE_GROUP_ID}" tiagocode; \
    useradd  -u "${TIAGOCODE_USER_ID}" -m -s /bin/bash -g tiagocode tiagocode; \
    adduser tiagocode www-data; \
    sed -i '/^tiagocode/s/!/*/' /etc/shadow

ADD --chmod=0755 https://github.com/mlocati/docker-php-extension-installer/releases/latest/download/install-php-extensions /usr/local/bin/

RUN set -ex; \
    install-php-extensions \
        bcmath \
        bz2 \
        calendar \
        @composer \
        ctype \
        curl \
        dom \
        gd \
        gmp \
        intl \
        mysqli \
        opcache \
        pdo_pgsql \
        pgsql \
        redis \
        soap \
        yaml \
        xml \
        zip \
        uploadprogress \
        apcu

WORKDIR /var/www/html

RUN chown -R tiagocode:tiagocode /var/www
