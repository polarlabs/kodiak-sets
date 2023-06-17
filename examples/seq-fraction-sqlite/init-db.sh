#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

SQLITE_MIGRATIONS="${SCRIPT_DIR}"
DATABASE="${SCRIPT_DIR}/test.db"

FILES=$(find $SQLITE_MIGRATIONS -type f -name "*.sql" | sort -V)
for FILE in $FILES; do
  cat $FILE | sqlite3 $DATABASE
  if [ $? -ne 0 ]; then
    echo "Error in $FILE"
  fi
done

export DATABASE_URL="sqlite://${DATABASE}"
