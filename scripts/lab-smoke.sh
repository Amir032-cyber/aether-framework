#!/usr/bin/env bash
set -euo pipefail
checks=(
  "http://127.0.0.1:8080/health"
  "http://127.0.0.1:3000"
  "http://127.0.0.1:8081/WebGoat"
  "http://127.0.0.1:9090/WebWolf"
  "http://127.0.0.1:8082"
)
for url in "${checks[@]}"; do
  printf '%-42s ' "$url"
  if curl --max-time 8 --silent --show-error --output /dev/null "$url"; then
    echo 'OK'
  else
    echo 'NOT READY'
  fi
done
