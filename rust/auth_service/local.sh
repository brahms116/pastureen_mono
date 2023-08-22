#!/bin/bash

set -e

env=local

echo "Running with environment: $env"

set -a
. ./.$env.env
set +a

cargo run -p auth_service


