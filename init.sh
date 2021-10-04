#!/usr/bin/env bash

cd user_service && docker-compose up -d && cd ..
cd user_interaction_service && docker-compose up -d && cd ..
cd content_service && docker-compose up -d && cd ..
