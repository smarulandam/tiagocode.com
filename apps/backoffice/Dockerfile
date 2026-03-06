# syntax=docker/dockerfile:1.7
ARG BASE_IMAGE_NAME
ARG BASE_IMAGE_VERSION

FROM ${BASE_IMAGE_NAME}:${BASE_IMAGE_VERSION}

WORKDIR /var/www/html

ENV COMPOSER_MEMORY_LIMIT=-1 \
    COMPOSER_CACHE_DIR=/tmp/composer-cache

COPY --chown=tiagocode:tiagocode ./composer.json ./composer.lock ./

RUN --mount=type=cache,target=/tmp/composer-cache,uid=1000,gid=1000 \
    set -ex; \
    su-exec tiagocode composer install \
        --no-dev \
        --no-interaction \
        --optimize-autoloader \
        --prefer-dist

COPY --chown=tiagocode:tiagocode . .
COPY --chown=tiagocode:tiagocode ./web/sites/default/default.settings.php ./web/sites/default/settings.php
COPY ./php.ini-production "$PHP_INI_DIR/php.ini"

RUN set -ex; \
    mkdir -p /var/www/html/web/sites/default/files; \
    chmod -R 775 /var/www/html/web/sites/default/files; \
    chown -R tiagocode:tiagocode /var/www/html/web/sites/default/files; \
    mkdir -p /var/www/html/private; \
    chmod -R 775 /var/www/html/private; \
    chown -R tiagocode:tiagocode /var/www/html/private; \
    chmod 664 /var/www/html/web/sites/default/settings.php; \
    chown tiagocode:tiagocode /var/www/html/web/sites/default/settings.php

ENV PATH="/var/www/html/vendor/bin:${PATH}"

USER tiagocode

EXPOSE 9000

CMD ["php-fpm"]
