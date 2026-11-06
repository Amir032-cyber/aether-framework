#!/usr/bin/env bash
set -euo pipefail
command -v protoc >/dev/null 2>&1 || { echo "protoc est requis" >&2; exit 1; }
echo "Les contrats protobuf sont générés par le build Rust."
