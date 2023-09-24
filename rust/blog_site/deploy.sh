#!/bin/bash

set -e

env=$1

if [[ "$env" != "prod" && "$env" != "dev" && "$env" != "test" ]]; then
  echo "Please provide a valid environment name (prod, test or dev)"
  exit 1
fi

echo "cleaning up build directory"

rm -rf ./build
mkdir ./build

echo "Building skeleton"

cargo run --release -p blog_site 

echo "Uploading blog skeleton to S3 $env"

ls -la ./build

aws s3 sync ./build/index.html s3://pastureen-blog-$env/index.html --delete
aws s3 sync ./build/posts.html s3://pastureen-blog-$env/posts.html --delete
