#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

command -v cargo >/dev/null 2>&1 || { echo "cargo est requis" >&2; exit 1; }
command -v protoc >/dev/null 2>&1 || { echo "protoc est requis" >&2; exit 1; }

cargo fetch --manifest-path "$ROOT_DIR/src/Cargo.toml"
cargo test --manifest-path "$ROOT_DIR/src/Cargo.toml"
printf 'AETHER est prêt pour le développement local.\n'
