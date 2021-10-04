#!/usr/bin/env bash

echo "Populating user_service (localhost:8001)"
IFS=$'\n';for line in $(cat data/user.json);do curl -X POST 'localhost:8001' -H 'Content-Type: application/json' -d "$line";done
