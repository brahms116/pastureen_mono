#!/bin/bash

set -e

set -a
. ./local.env
set +a


psql $AUTH_API_DB_CONN_STR -f ./schema.sql
