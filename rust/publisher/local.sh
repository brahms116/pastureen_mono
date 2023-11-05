#!/bin/bash

set -e

eval "$(cat ../../scripts/local_template.sh)"

cargo run -p publisher

