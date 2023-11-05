#!/bin/bash

set -e

eval "$(cat ../../scripts/local_template.sh)"

./test.sh
