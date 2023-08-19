#!/bin/bash

env=$1

if [ -z $env ]; then
  env="dev"
fi
if [ $env -eq "prod"]; then
  echo "You can't reset the prod database"
  exit 1
fi

set -o allexport
. ./.$env.env
set +o allexport

psql $AUTH_SERVICE_DB_CONN_STR -f ./drop.sql
psql $AUTH_SERVICE_DB_CONN_STR -f ./schema.sql
