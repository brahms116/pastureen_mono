#!/bin/bash

set -e

eval "$(cat ../../scripts/local_template.sh)"

cargo run -p blog_skeleton

python3 -m http.server 8000 --directory ./build


