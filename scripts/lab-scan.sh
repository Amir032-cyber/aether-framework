#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPOSE_FILE="$ROOT_DIR/deployments/docker/lab-compose.yml"
command -v docker >/dev/null 2>&1 || { echo "Docker est requis." >&2; exit 1; }
docker compose -f "$COMPOSE_FILE" --profile lab --profile tools run --rm zap-baseline
docker compose -f "$COMPOSE_FILE" --profile lab --profile tools run --rm nmap-discovery
docker compose -f "$COMPOSE_FILE" --profile tools run --rm trivy-fs
echo "Scans terminés. Les rapports sont dans le volume Docker aether-lab-reports."
