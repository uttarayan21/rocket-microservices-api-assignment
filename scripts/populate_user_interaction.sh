#!/usr/bin/env bash

echo "Populating user_interaction_service (localhost:8003)"
IFS=$'\n';for line in $(cat data/user_interactions.json);do curl -X POST 'localhost:8003' -H 'Content-Type: application/json' -d "$line";done
