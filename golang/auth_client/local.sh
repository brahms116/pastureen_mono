set -e

set -a
. .local.env
set +a

./test.sh
