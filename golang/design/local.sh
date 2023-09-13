#!/bin/bash

set -e

env=local

echo "Running with environment: $env"

set -a
. ./.$env.env
set +a

go run *.go


