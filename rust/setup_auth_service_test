#!/bin/bash

set -o allexport
source ./auth_service/.test.env
set +o allexport

psql $AUTH_SERVICE_DB_CONN_STR -f ./auth_service/schema.sql
