#!/bin/bash

set -euo pipefail

echo "-----------------------------------------------------"
echo "[ENV-TIAGOCODE.COM] hook: ApplicationStop"
echo "-----------------------------------------------------"

cd /home/ec2-user/env-tiagocode.com

docker-compose \
  -f compose.yml \
  -f compose.production.yml \
  down
  
if docker volume inspect tiagocode_backoffice_codebase >/dev/null 2>&1; then
  docker volume rm tiagocode_backoffice_codebase
else
  echo "Docker volume 'tiagocode_backoffice_codebase' does not exist"
fi

docker image prune -f