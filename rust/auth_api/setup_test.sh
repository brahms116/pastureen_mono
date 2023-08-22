#!/bin/bash
#
set -o allexport
. ./auth_service/.test.env
set +o allexport

psql $AUTH_API_DB_CONN_STR -f ./schema.sql
