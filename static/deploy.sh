#!/bin/bash

set -e

env=$1

if [[ "$env" != "prod" && "$env" != "dev" && "$env" != "test" ]]; then
  echo "Please provide a valid environment name (prod, test or dev)"
  exit 1
fi

echo "Uploading static content to S3 $env"

aws s3 sync ./content s3://pastureen-static-assets-$env
