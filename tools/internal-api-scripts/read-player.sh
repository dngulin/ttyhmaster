#!/bin/bash

if [[ $# -ne 1 ]]; then
    echo 'Please, pass the player name as the only argument' >&2
    exit 1
fi

curl -H @payload/headers.txt -w "\n%{http_code}\n" "http://0.0.0.0:8080/ttyh/player/${1}"