#!/bin/bash

set -e

env=$1

if [[ "$env" != "test" && "$env" != "prod" && "$env" != "dev"  ]]; then
  echo "Please provide a valid environment name (test, prod or dev)"
    exit 1
fi

echo "Deploying to environment: $env"

env_location=$env

if [[ "$env" == "prod" ]]; then
  env_location=""
fi

set -a
. ../.$env_location.env
set +a

cd ..

docker build -t auth_service_lambda -f ./Dockerfile.lambda . --build-arg PROJECT_NAME=auth_web_service

docker create --name auth_service_lambda auth_service_lambda
docker cp auth_service_lambda:/app/lambda_build/lambda.zip ./auth_web_service/lambda.zip
docker rm auth_service_lambda

cd auth_web_service





