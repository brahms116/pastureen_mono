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

aws lambda update-function-code --function-name librarian_$env --zip-file fileb://./lambda.zip
aws lambda update-function-configuration --function-name librarian_$env --environment \
  Variables="{\
    LIBRARIAN_DB_CONN_STR=$LIBRARIAN_DB_CONN_STR,\
    LIBRARIAN_AUTH_URL=$LIBRARIAN_AUTH_URL,\
    LIBRARIAN_ADMIN_EMAIL=$LIBRARIAN_ADMIN_EMAIL,\
    LIBRARIAN_BLOG_BUCKET_NAME=$LIBRARIAN_BLOG_BUCKET_NAME,\
    LIBRARIAN_LISTEN_ADDR=$LIBRARIAN_LISTEN_ADDR,\
    READINESS_CHECK_PATH='/healthcheck',\
    GIN_MODE='release'\
  }"
