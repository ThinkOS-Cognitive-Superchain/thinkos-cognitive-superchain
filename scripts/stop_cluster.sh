#!/usr/bin/env bash
set -euo pipefail
for d in state/nodes/* ; do
  [ -d "$d" ] || continue
  if [ -f "$d/node.pid" ]; then
    PID=$(cat "$d/node.pid" || true)
    if [ -n "${PID:-}" ] && kill -0 "$PID" 2>/dev/null; then
      echo ">>> stopping $(basename "$d") (pid $PID)"
      kill "$PID" || true
    fi
  fi
done
echo ">>> cluster stop requested."
