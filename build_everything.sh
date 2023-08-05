#!/bin/bash


echo "Clearing artifacts"

rm -rf ./build 2> /dev/null
docker rm pastureen_build_output 2> /dev/null

echo "Using docker to run build"

docker build -t pastureen_build_output .
docker create --name pastureen_build_output pastureen_build_output
docker cp pastureen_build_output:/build ./build

ENV=$ENV

if [ -z "$ENV" ]
then
      ENV="dev"
fi

echo "Deploying for $ENV"

lambda_zips=$(find ./build -name "*.zip")

for lambda_zip in $lambda_zips
do
    echo "Deploying $lambda_zip"
    aws lambda update-function-code --function-name "$(basename $lambda_zip .zip)_$ENV" --zip-file fileb://$lambda_zip --region ap-southeast-2
done

echo "Cleanup"
docker rm pastureen_build_output
