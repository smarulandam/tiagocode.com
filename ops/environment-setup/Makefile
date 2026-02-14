.PHONY: build
build:
	chmod 600 docker/traefik/data/acme.json
	docker compose build 

.PHONY: start
start:
	docker compose up -d

.PHONY: stop
stop:
	docker compose down

