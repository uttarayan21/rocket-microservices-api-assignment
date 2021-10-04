#!/usr/bin/env bash

echo "Populating content_service (localhost:8002)"
curl -X POST 'localhost:8002/ingest' -H "Content-Type: text/csv" --data-binary @data/story.csv
