#!/bin/bash

# stub as local just uses dev for static assets

set -e

env=local

echo "Running with environment: $env"

set -a
. ./.$env.env
set +a

./deploy.sh dev


