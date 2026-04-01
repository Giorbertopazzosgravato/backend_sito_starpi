#!/bin/bash
set -e

# Wait for the database to be ready, then restore the latest backup
# This script automatically runs when the Postgres container is created
echo "Restoring database from latest.backup..."
pg_restore -U "$POSTGRES_USER" -d "$POSTGRES_DB" -1 /docker-entrypoint-initdb.d/backups/latest.backup
echo "Database restored successfully."