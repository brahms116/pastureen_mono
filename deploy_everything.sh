#!/bin/bash

env=$1

if [ "$env" = "dev" ]; then
    echo "Deploying to dev"
    # Deploy to dev
elif [ "$env" = "prod" ]; then
    echo "Deploying to prod"
    # Deploy to prod
else
    echo "Invalid environment"
    exit 1
fi

echo "Clearing artifacts"

rm -rf ./build &> /dev/null
docker rm pastureen_build_output &> /dev/null

echo "Using docker to run build"

docker build -t pastureen_build_output .
docker create --name pastureen_build_output pastureen_build_output
docker cp pastureen_build_output:/build ./build

echo "Deploying artifacts"

cd ./terraform/$env

terraform init
terraform apply

echo "Cleanup"
cd ../..
rm -rf ./build
docker rm pastureen_build_output
