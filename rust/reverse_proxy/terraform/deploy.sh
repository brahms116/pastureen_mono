#!/bin/bash

set -e

env=$1

if [[ "$env" != "prod" && "$env" != "dev" && "$env" != "test" ]]; then
  echo "Please provide a valid environment name (prod, test or dev)"
  exit 1
fi

echo "Compiling"

docker run -v "$(pwd)/../":/app -v .:/out --env PROJECT_NAME=reverse_proxy public.ecr.aws/p1r0g3x6/rust_lambda_build_container:latest

echo "Deploying to $env"

aws lambda update-function-code --function-name reverse_proxy_$env --zip-file fileb://./lambda.zip
aws lambda update-function-configuration --function-name reverse_proxy_$env --environment \
  Variables="{\
    REVERSE_PROXY_LISTEN_ADDR=$REVERSE_PROXY_LISTEN_ADDR\
  }"
