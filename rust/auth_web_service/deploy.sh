#!/bin/bash

set -e

env=$1

if [[ "$env" != "prod" && "$env" != "dev"  ]]; then
  echo "Please provide a valid environment name (prod or dev)"
    exit 1
fi

echo "Compiling"

docker run -v "$(pwd)/../":/app -v .:/out --env PROJECT_NAME=auth_web_service public.ecr.aws/p1r0g3x6/rust_lambda_build_container:latest


if [[ "$env" == "prod" ]]; then
  echo "Deploying to production environment"
  set -a
  . ../.env
  set +a
  aws lambda update-function-code --function-name auth_service_prod --zip-file fileb://./lambda.zip
  aws lambda update-function-configuration --function-name auth_service_prod --environment \
    Variables="{\
      AUTH_SERVICE_SECRET=$AUTH_SERVICE_SECRET,\
      AUTH_SERVICE_DB_CONN_STR=$AUTH_SERVICE_DB_CONN_STR,\
      AUTH_WEB_SERVICE_LISTEN_ADDR=$AUTH_WEB_SERVICE_LISTEN_ADDR\
    }"

  exit 0
fi

echo "Deploying to development environment"
set -a
. ../.dev.env
set +a
aws lambda update-function-code --function-name auth_service_dev --zip-file fileb://./lambda.zip
aws lambda update-function-configuration --function-name auth_service_dev --environment \
  Variables="{\
    AUTH_SERVICE_SECRET=$AUTH_SERVICE_SECRET,\
    AUTH_SERVICE_DB_CONN_STR=$AUTH_SERVICE_DB_CONN_STR,\
    AUTH_WEB_SERVICE_LISTEN_ADDR=$AUTH_WEB_SERVICE_LISTEN_ADDR\
  }"


echo "Deploying to test environment"
set -a
. ../.dev.env
set +a
aws lambda update-function-code --function-name auth_service_dev --zip-file fileb://.lambda.zip
aws lambda update-function-configuration --function-name auth_service_dev --environment \
  Variables="{\
    AUTH_SERVICE_SECRET=$AUTH_SERVICE_SECRET,\
    AUTH_SERVICE_DB_CONN_STR=$AUTH_SERVICE_DB_CONN_STR,\
    AUTH_WEB_SERVICE_LISTEN_ADDR=$AUTH_WEB_SERVICE_LISTEN_ADDR\
  }"



