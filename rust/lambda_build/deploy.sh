#!/bin/bash

aws ecr-public get-login-password --region us-east-1 | docker login --username AWS --password-stdin public.ecr.aws/p1r0g3x6


docker build -t rust_lambda_build_container .
docker tag rust_lambda_build_container:latest public.ecr.aws/p1r0g3x6/rust_lambda_build_container:latest

docker push public.ecr.aws/p1r0g3x6/rust_lambda_build_container:latest
