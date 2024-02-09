#!/bin/bash

set -e

cur_ts=$(date +%Y-%m-%d-%H-%M-%S)

echo "$cur_ts"

mkdir ./$cur_ts

while read line; do
  [[ -z "$line" ]] && continue
  key=$(echo $line | sed -E 's/(^[^=]*)=.*/\1/')
  value=$(echo $line | sed -E 's/^[^=]*=//')
  echo "Found db $key with value $value"
  echo "Dumping $key"
  pg_dump -f ./$cur_ts/$key.sql $value
  echo "Done with $key"
done < $1

echo "Syncing with S3"
aws s3 sync ./$cur_ts s3://pastureen-db-backups/$cur_ts
echo "Done syncing with S3"
echo "Cleaning up"
rm -rf ./$cur_ts
