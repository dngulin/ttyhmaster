#!/bin/bash

if [[ $# -ne 1 ]]; then
    echo 'Please, pass the player name as the only argument' >&2
    exit 1
fi

curl -X POST -H @payload/headers.txt -H 'Content-Type: application/json' -d @payload/update-player.json -w "\n%{http_code}\n" "http://0.0.0.0:8080/ttyh/player/${1}"