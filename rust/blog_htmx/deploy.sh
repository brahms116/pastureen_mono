#!/bin/bash

set -e

env=$1

if [[ "$env" != "prod" && "$env" != "dev" && "$env" != "test" ]]; then
  echo "Please provide a valid environment name (prod, test or dev)"
  exit 1
fi

echo "Compiling"

docker run -v "$(pwd)/../":/app -v .:/out -v "$(pwd)/../../proto":/proto --env PROJECT_NAME=blog_htmx public.ecr.aws/p1r0g3x6/rust_lambda_build_container:latest

echo "Deploying to $env"

aws lambda update-function-code --function-name blog_htmx_$env --zip-file fileb://./lambda.zip
aws lambda update-function-configuration --function-name blog_htmx_$env --environment \
  Variables="{\
    BLOG_HTMX_ASSETS_URL=$BLOG_HTMX_ASSETS_URL,\
    BLOG_HTMX_BASE_URL=$BLOG_HTMX_BASE_URL,\
    BLOG_HTMX_HTMX_URL=$BLOG_HTMX_HTMX_URL,\
    BLOG_HTMX_LIBRAIRN_URL=$BLOG_HTMX_LIBRAIRN_URL,\
    BLOG_HTMX_LISTEN_ADDR=$BLOG_HTMX_LISTEN_ADDR,\
    READINESS_CHECK_PATH='/healthcheck'\
  }"
