#!/usr/bin/env

echo "Populating user_service (localhost:8001)"
IFS=$'\n';for line in $(cat data/user.json);do curl -X POST 'localhost:8001' -H 'Content-Type: application/json' -d "$line";done

echo "Populating user_interaction_service (localhost:8003)"
IFS=$'\n';for line in $(cat data/user_interactions.json);do curl -X POST 'localhost:8003' -H 'Content-Type: application/json' -d "$line";done

echo "Populating content_service (localhost:8002)"
curl -X POST 'localhost:8002/ingest' -H "Content-Type: text/csv" --data-binary @data/story.csv





