#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPOSE_FILE="$ROOT_DIR/deployments/docker/lab-compose.yml"
command -v docker >/dev/null 2>&1 || { echo "Docker est requis." >&2; exit 1; }
docker compose -f "$COMPOSE_FILE" --profile lab up -d --pull missing
printf '%s\n' 'Laboratoire démarré uniquement sur localhost :'
printf '%s\n' '  Juice Shop : http://127.0.0.1:3000'
printf '%s\n' '  WebGoat    : http://127.0.0.1:8081/WebGoat'
printf '%s\n' '  WebWolf    : http://127.0.0.1:9090/WebWolf'
printf '%s\n' '  DVWA       : http://127.0.0.1:8082'
printf '%s\n' '  Aether API : http://127.0.0.1:8080/health'
