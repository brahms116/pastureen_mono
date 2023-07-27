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

rm -rf ./build 2> /dev/null
docker rm pastureen_build_output 2> /dev/null

echo "Using docker to run build"

docker build -t pastureen_build_output .
docker create --name pastureen_build_output pastureen_build_output
docker cp pastureen_build_output:/build ./build

echo "Cleanup"
cd ../..
docker rm pastureen_build_output
