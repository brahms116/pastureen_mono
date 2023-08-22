#!/bin/bash

set -e

env=$1
if [ -z "$env" ]; then
    echo "Please provide environment name"
    exit 1
fi

echo "Setting up environment: $env"

set -a
. ../.$env.env
set +a

psql $AUTH_SERVICE_DB_CONN_STR -f ../auth_service/schema.sql

