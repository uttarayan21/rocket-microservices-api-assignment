#!/usr/bin/env bash


echo Dummy data already provided in data/user.json
exit 0
rm -f user.json
for i in $(seq 0 50);do curl https://random-data-api.com/api/users/random_user | jq -cM '. | { first_name: .first_name, last_name: .last_name, email_id: .email, phone_number: .phone_number }' >> user.json;done
