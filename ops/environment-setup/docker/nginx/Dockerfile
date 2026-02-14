ARG NGINX_VERSION=latest

FROM nginx:$NGINX_VERSION

ARG TIAGOCODE_USER_ID=1000
ARG TIAGOCODE_GROUP_ID=1000

ARG TARGETPLATFORM
ARG NGINX_PHPFPM_HOST
ARG NGINX_SERVER_ROOT

# Install OS dependencies
RUN set -ex; \
    apk --update --no-cache add \
        shadow \
        bash \
        nano \ 
        curl \
        sudo \
        su-exec \
        gzip \
        tar \
        wget;

RUN set -ex; \
    existing_group=$(getent group "${TIAGOCODE_GROUP_ID}" | cut -d: -f1); \
    if [[ -n "${existing_group}" ]]; then delgroup "${existing_group}"; fi; \
    existing_user=$(getent passwd "${TIAGOCODE_USER_ID}" | cut -d: -f1); \
    if [[ -n "${existing_user}" ]]; then deluser "${existing_user}"; fi; \
    groupadd -g "${TIAGOCODE_GROUP_ID}" tiagocode; \
    useradd  -u "${TIAGOCODE_USER_ID}" -m -s /bin/bash -g tiagocode tiagocode; \
    adduser tiagocode www-data; \
    adduser tiagocode nginx; \
    sed -i '/^tiagocode/s/!/*/' /etc/shadow; \
    { \
        echo "Defaults secure_path=\"$PATH\""; \
        echo 'Defaults env_keep += "APP_ROOT FILES_DIR NGINX_VHOST_PRESET"' ; \
        echo -n 'tiagocode ALL=(root) NOPASSWD:SETENV: ' ; \
        echo -n '/usr/local/bin/init_volumes, ' ; \
        echo '/usr/sbin/nginx' ; \
    } | tee /etc/sudoers.d/tiagocode;

RUN set -xe; \
    dockerplatform=${TARGETPLATFORM:-linux/amd64};\
    gotpl_url="https://github.com/wodby/gotpl/releases/latest/download/gotpl-${TARGETPLATFORM/\//-}.tar.gz"; \
    wget -O- "${gotpl_url}" | tar xz --no-same-owner -C /usr/local/bin; \
    rm -rf /var/cache/apk/*;

COPY default.conf.tmpl /etc/gotpl/default.conf.tmpl

RUN gotpl /etc/gotpl/default.conf.tmpl > /etc/nginx/conf.d/default.conf

WORKDIR /var/www/html

USER tiagocode 

EXPOSE 80

# Entrypoint
CMD ["sudo", "nginx", "-g", "daemon off;"]