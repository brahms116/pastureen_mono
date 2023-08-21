#!/bin/bash

set -e

env=$1

if [[ "$env" != "prod" && "$env" != "dev" && "$env" != "test"]]; then
  echo "Please provide a valid environment name (prod, test or dev)"
  exit 1
fi

echo "Compiling"

docker run -v "$(pwd)/../":/app -v .:/out --env PROJECT_NAME=auth_web_service public.ecr.aws/p1r0g3x6/rust_lambda_build_container:latest

echo "Deploying to $env"

aws lambda update-function-code --function-name auth_service_$env --zip-file fileb://./lambda.zip
aws lambda update-function-configuration --function-name auth_service_$env --environment \
  Variables="{\
    AUTH_SERVICE_SECRET=$AUTH_SERVICE_SECRET,\
    AUTH_SERVICE_DB_CONN_STR=$AUTH_SERVICE_DB_CONN_STR,\
    AUTH_WEB_SERVICE_LISTEN_ADDR=$AUTH_WEB_SERVICE_LISTEN_ADDR\
  }"
