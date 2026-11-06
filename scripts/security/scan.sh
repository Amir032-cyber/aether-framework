#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
! grep -RInE --exclude-dir=.git --exclude-dir=target --exclude='scan.sh' "sk-[A-Za-z0-9_-]{20,}|BEGIN .* PRIVATE KEY" "$ROOT_DIR"
