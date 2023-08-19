#!/bin/bash

set -o allexport
. ./auth_service/.test.env
set +o allexport

cargo test -p auth_service


