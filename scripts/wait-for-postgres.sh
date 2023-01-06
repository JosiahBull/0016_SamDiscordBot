#!/bin/sh
# wait-for-postgres.sh
# source: https://docs.docker.com/compose/startup-order/

set -e

sleep 5

until PGPASSWORD="$POSTGRES_PASS" psql -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" -U "$POSTGRES_USER" -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up - executing command"
exec "$@"
