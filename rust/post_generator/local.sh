#!/bin/bash

set -e

set -a
. ./.local.env
set +a

cargo run -p post_generator

