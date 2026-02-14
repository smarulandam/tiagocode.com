# Environment setup

This repository provides everything necessary to effortlessly establish a fully functional environment for my project, Tiagocode.com.

## Instructions

1. Clone this repository `git@github.com:tiagocodecom/environment-setup.git env-setup`.

2. Move to the recently created directory `cd env-setup`.

3. Copy the .env file `cp .env.example .env`.

3. Change the environment variables: 
    ```conf
    APPS_DOMAIN_NAME=example.com

    BACKOFFICE_PATH_IN_HOST=./code/backoffice
    WEBSITE_PATH_IN_HOST=./code/website

    POSTGRES_DB=database_name
    POSTGRES_USER=database_user
    POSTGRES_PASSWORD=database_password

    TRAEFIK_ACME_EMAIL=cloudflare_email
    TRAEFIK_CF_EMAIL=cloudflare_email
    TRAEFIK_CF_API_KEY=cloudflare_api_token
    TRAEFIK_USER_PASSWORD=traefik_user:traefik_password_scaped

    PGADMIN_EMAIL=pgadmin_email
    PGADMIN_PASSWORD=pgadmin_password

    REDIS_PASSWORD=redis_password

    REDIS_WEBUI_EMAIL=redis_webui_email
    REDIS_WEBUI_PASSWORD=redis_webui_password
    ```

4. Build the containers `make build`.

5. Start the container `make start`.

## Important

Currently, this environment setup supports only development configurations; production support will be added in the future.