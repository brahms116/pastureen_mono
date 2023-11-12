#!/bin/bash

set -e

env=$1

if [[ "$env" != "prod" && "$env" != "dev" && "$env" != "test" ]]; then
  echo "Please provide a valid environment name (prod, test or dev)"
  exit 1
fi

echo "Compiling"

docker run -v "$(pwd)/../":/app -v .:/out --env PROJECT_NAME=publisher public.ecr.aws/p1r0g3x6/rust_lambda_build_container:latest

echo "Deploying to $env"

aws lambda update-function-code --function-name publisher_$env --zip-file fileb://./lambda.zip

# Get the required envs
env_vars_str=""

while read line; do
  # Check it doesn't start with a #
  [[ $line =~ ^#.*$ ]] && continue

  # Check it's not empty
  [[ -z "$line" ]] && continue

  env_vars_str+="$line=${!line},"
done < "./required_envs"

aws lambda update-function-configuration --function-name publisher_$env --environment \
  Variables="{$env_vars_str}"
