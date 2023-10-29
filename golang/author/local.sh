set -e

set -a
. .local.env
set +a

go run *.go "$@"

