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
COPY ./zz-performance.ini "$PHP_INI_DIR/conf.d/zz-performance.ini"

RUN set -ex; \
    mkdir -p /var/www/html/web/sites/default/files; \
    mkdir -p /var/www/html/private; \
    mkdir -p /tmp/drupal/php_storage; \
    chmod -R a-w /var/www/html; \
    chown -R tiagocode:tiagocode /var/www/html/web/sites/default/files; \
    find /var/www/html/web/sites/default/files -type d -exec chmod 775 {} +; \
    find /var/www/html/web/sites/default/files -type f -exec chmod 664 {} +; \
    chown -R tiagocode:tiagocode /var/www/html/private; \
    find /var/www/html/private -type d -exec chmod 775 {} +; \
    find /var/www/html/private -type f -exec chmod 664 {} +; \
    chown -R tiagocode:tiagocode /tmp/drupal; \
    chmod 700 /tmp/drupal /tmp/drupal/php_storage

ENV PATH="/var/www/html/vendor/bin:${PATH}"

USER tiagocode

EXPOSE 9000

CMD ["php-fpm"]
