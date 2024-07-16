#!/bin/bash

curl -X POST -H @payload/headers.txt -H 'Content-Type: application/json' -d @payload/create-player.json -w "\n%{http_code}\n" "http://0.0.0.0:8080/ttyh/player"