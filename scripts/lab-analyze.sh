#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AGENT_DIR="$ROOT_DIR/modules/auto-pentest"
INPUT="${1:-}"
OUTPUT="${2:-$ROOT_DIR/reports/lab-report-$(date -u +%Y%m%dT%H%M%SZ).json}"

if [[ -z "$INPUT" || ! -f "$INPUT" ]]; then
  echo "Usage: $0 <rapport-entree.json> [rapport-sortie.json]" >&2
  exit 2
fi
mkdir -p "$(dirname "$OUTPUT")"
PYTHONPATH="$AGENT_DIR" python3 -m auto_pentest analyze --file "$INPUT" > "$OUTPUT"
python3 - "$OUTPUT" <<'PY'
import json
import sys
from pathlib import Path
path = Path(sys.argv[1])
data = json.loads(path.read_text(encoding="utf-8"))
required = {"summary", "risk_level", "findings", "remediations"}
missing = required - data.keys()
if missing:
    raise SystemExit(f"rapport incomplet: {', '.join(sorted(missing))}")
text = path.read_text(encoding="utf-8").lower()
for marker in ("authorization", "set-cookie", "password", "secret"):
    if marker in text and "[redacted]" not in text:
        raise SystemExit(f"champ sensible non masqué: {marker}")
print(f"Rapport généré: {path}")
print(f"Niveau de risque: {data['risk_level']}")
print(f"Constats: {len(data['findings'])}")
PY
