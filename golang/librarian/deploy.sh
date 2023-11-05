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

# Get the required envs
env_vars_str=""

while read line; do
  # Check it doesn't start with a #
  [[ $line =~ ^#.*$ ]] && continue

  # Check it's not empty
  [[ -z "$line" ]] && continue

  env_vars_str+="$line=${!line},"
done < "./required_envs"

aws lambda update-function-configuration --function-name librarian_$env --environment \
  Variables="{$env_vars_str}"
