#!/usr/bin/env bash
set -euo pipefail
command -v protoc >/dev/null 2>&1 || { echo "protoc est requis" >&2; exit 1; }
protoc --proto_path=src/proto --descriptor_set_out=/tmp/aether.pb src/proto/aether.proto src/proto/modules.proto
