#!/bin/bash

set -e

psql $AUTH_API_DB_CONN_STR -f ../auth_api/schema.sql

