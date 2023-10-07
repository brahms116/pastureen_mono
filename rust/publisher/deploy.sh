#!/bin/bash

set -e

env=$1

if [[ "$env" != "prod" && "$env" != "dev" && "$env" != "test" ]]; then
  echo "Please provide a valid environment name (prod, test or dev)"
  exit 1
fi

echo "Compiling"

docker run -v "$(pwd)/../":/app -v .:/out -v "$(pwd)/../../proto":/proto --env PROJECT_NAME=publisher public.ecr.aws/p1r0g3x6/rust_lambda_build_container:latest

echo "Deploying to $env"

aws lambda update-function-code --function-name publisher_$env --zip-file fileb://./lambda.zip
aws lambda update-function-configuration --function-name publisher_$env --environment \
  Variables="{\
    PUBLISHER_ASSETS_URL=$PUBLISHER_ASSETS_URL,\
    PUBLISHER_BASE_URL=$PUBLISHER_BASE_URL,\
    PUBLISHER_HTMX_URL=$PUBLISHER_HTMX_URL,\
    PUBLISHER_AUTH_URL=$PUBLISHER_AUTH_URL,\
    PUBLISHER_LISTEN_ADDR=$PUBLISHER_LISTEN_ADDR,\
    PUBLISHER_ADMIN_EMAIL=$PUBLISHER_ADMIN_EMAIL,\
    READINESS_CHECK_PATH='/healthcheck'\
  }"
