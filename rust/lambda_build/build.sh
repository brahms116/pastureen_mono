#!/bin/bash

. /cargo/env

cd /app

cargo build --release -p $PROJECT_NAME

cp target/release/$PROJECT_NAME /out/$PROJECT_NAME

mv /out/$PROJECT_NAME /out/bootstrap

zip -j /out/lambda.zip /out/bootstrap

rm /out/bootstrap







