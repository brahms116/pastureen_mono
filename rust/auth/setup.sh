#!/bin/bash

set -e

psql $AUTH_DB_CONN_STR -f ./schema.sql
