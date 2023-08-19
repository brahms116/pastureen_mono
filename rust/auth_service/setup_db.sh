#!/bin/bash

env=$1

if [ -z $env ]; then
  env="dev"
fi
if [ $env -eq "prod"]; then
  env=""
fi

set -o allexport
. ./.$env.env
set +o allexport

psql $AUTH_SERVICE_DB_CONN_STR -f ./schema.sql
