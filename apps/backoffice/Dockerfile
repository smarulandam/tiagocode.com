ARG PHP_VERSION=8.4-fpm-alpine

FROM php:$PHP_VERSION

COPY ./php.ini-production "$PHP_INI_DIR/php.ini"

ARG TIAGOCODE_USER_ID=1000
ARG TIAGOCODE_GROUP_ID=1000

# Install OS dependencies
RUN set -ex; \
    apk --update --no-cache add \
        shadow \
        bash \
        nano \
        curl \
        sudo \
        git \
        patch \
        postgresql-client \
        su-exec;

RUN set -ex; \
    existing_group=$(getent group "${TIAGOCODE_GROUP_ID}" | cut -d: -f1); \
    if [[ -n "${existing_group}" ]]; then delgroup "${existing_group}"; fi; \
    existing_user=$(getent passwd "${TIAGOCODE_USER_ID}" | cut -d: -f1); \
    if [[ -n "${existing_user}" ]]; then deluser "${existing_user}"; fi; \
    groupadd -g "${TIAGOCODE_GROUP_ID}" tiagocode; \
    useradd  -u "${TIAGOCODE_USER_ID}" -m -s /bin/bash -g tiagocode tiagocode; \
    adduser tiagocode www-data; \
    sed -i '/^tiagocode/s/!/*/' /etc/shadow;

# Install helper for PHP extensions installation
ADD --chmod=0755 https://github.com/mlocati/docker-php-extension-installer/releases/latest/download/install-php-extensions /usr/local/bin/

# Install PHP extensions
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
        apcu;

WORKDIR /var/www/html

COPY . .
COPY ./web/sites/default/default.settings.php ./web/sites/default/settings.php

ENV COMPOSER_MEMORY_LIMIT=-1

# Install dhependencies and fix ownership and install dependencies
RUN set -ex; \
    chown tiagocode:tiagocode -R /var/www; \
    \
    find /var/www -type d -exec chmod 755 {} \;; \
    find /var/www -type f -exec chmod 644 {} \;; \
    \
    mkdir -p /var/www/html/web/sites/default/files; \
    chmod -R 775 /var/www/html/web/sites/default/files; \
    chown -R tiagocode:tiagocode /var/www/html/web/sites/default/files; \
    mkdir -p /var/www/html/private; \
    chmod -R 775 /var/www/html/private; \
    chown -R tiagocode:tiagocode /var/www/html/private; \
    \
    chmod 664 /var/www/html/web/sites/default/settings.php; \
    chown tiagocode:tiagocode /var/www/html/web/sites/default/settings.php; \
    \
    su-exec tiagocode composer install --no-dev --optimize-autoloader; \
    su-exec tiagocode composer clear-cache;

ENV PATH="/var/www/html/vendor/bin:${PATH}"

USER tiagocode

EXPOSE 9000

CMD ["php-fpm"]



