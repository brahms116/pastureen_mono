#!/bin/bash

set -o allexport
. ./.test.env
set +o allexport

cargo test -p auth_api


