#!/bin/bash

set -e

env=$1

if [[ "$env" != "prod" && "$env" != "dev" && "$env" != "test" ]]; then
  echo "Please provide a valid environment name (prod, test or dev)"
  exit 1
fi

echo "Compiling"

GOOS=linux GOARCH=amd64 CGO_ENABLED=0 go build -o bootstrap

zip lambda.zip bootstrap

echo "Deploying to $env"

aws lambda update-function-code --function-name design_system_$env --zip-file fileb://./lambda.zip
aws lambda update-function-configuration --function-name design_system_$env --environment \
  Variables="{\
    DESIGN_SYSTEM_BASE_URL=$DESIGN_SYSTEM_BASE_URL,\
    READINESS_CHECK_PATH='/healthcheck'\
  }"
