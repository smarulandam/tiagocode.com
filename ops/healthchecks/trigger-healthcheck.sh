#!/bin/bash

# Leer el token desde un archivo seguro
TOKEN=$(<~/.healthchecks_github_token)

REPOSITORY="tiagocodecom/healthchecks"
WORKFLOW_FILE="healthcheck.yml" 
BRANCH="main"                       

curl -s -X POST \
-H "Accept: application/vnd.github+json" \
-H "Authorization: Bearer $TOKEN" \
"https://api.github.com/repos/$REPOSITORY/actions/workflows/$WORKFLOW_FILE/dispatches" \
-d "{\"ref\":\"$BRANCH\"}" \
> /dev/null 2>&1