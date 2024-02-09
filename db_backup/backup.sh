#!/bin/bash


# Script to back up different postgres databases and uploads them to s3
#
# Usage: ./backup.sh <file with db names and connection strings>
#
# The file should have the following format:
#
# db1=postgresql://user:password@host:port/db1
# db2=postgresql://user:password@host:port/db2
# ...
#
# The script will create a directory with the current timestamp and dump each
# database into a separate file in that directory. Then it will upload the
# directory to s3. The s3 bucket is hardcoded to pastureen-db-backups. And the folder
# in the bucket is the current timestamp.

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
