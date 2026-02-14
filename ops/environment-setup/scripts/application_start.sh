#!/bin/bash

set -euo pipefail

echo "-----------------------------------------------------"
echo "[ENV-TIAGOCODE.COM] hook: ApplicationStart"
echo "-----------------------------------------------------"

cd /home/ec2-user/env-tiagocode.com

# Login into Github Container Regitry
GITHUB_PAT=$(aws ssm get-parameter --name "/tiagocode/ghcr/pat" --with-decryption --query "Parameter.Value" --output text)
echo "$GITHUB_PAT" | docker login ghcr.io -u "smarulanda97" --password-stdin

docker-compose \
  -f compose.yml \
  -f compose.production.yml \
  up -d --build --pull always --force-recreate

docker-compose \
  -f compose.yml \
  -f compose.production.yml \
  exec -it backoffice drush updb -y

docker-compose \
  -f compose.yml \
  -f compose.production.yml \
  exec -it backoffice drush config:import -y

docker-compose \
  -f compose.yml \
  -f compose.production.yml \
  exec -it backoffice drush cr

  