#!/usr/bin/env bash
set -euo pipefail

AIFA_URL="${THINKOS_AIFA_URL:-http://127.0.0.1:8081}"
ITERS="${THINKOS_ITERS:-120}"
LOOP="${THINKOS_LOOP_SECS:-3}"

run_node () {
  local ID="$1"; shift
  echo ">>> launching node $ID"
  THINKOS_NODE_ID="$ID" \
  THINKOS_AIFA_URL="$AIFA_URL" \
  THINKOS_MARKET_MODE="neutral" \
  THINKOS_STATE_DIR="state/nodes/$ID" \
  THINKOS_ITERS="$ITERS" \
  THINKOS_LOOP_SECS="$LOOP" \
  "$@" \
  > "state/nodes/$ID/node.log" 2>&1 &
  echo $! > "state/nodes/$ID/node.pid"
}

# different telemetry profiles per node
run_node A env THINKOS_TELE_VOL=0.15 THINKOS_TELE_CONG=0.10 THINKOS_TELE_UPVAR=0.01 THINKOS_TELE_TREAS=0.92 target/release/thinkos-node
run_node B env THINKOS_TELE_VOL=0.30 THINKOS_TELE_CONG=0.25 THINKOS_TELE_UPVAR=0.05 THINKOS_TELE_TREAS=0.85 target/release/thinkos-node
run_node C env THINKOS_TELE_VOL=0.20 THINKOS_TELE_CONG=0.40 THINKOS_TELE_UPVAR=0.03 THINKOS_TELE_TREAS=0.88 target/release/thinkos-node
run_node D env THINKOS_TELE_VOL=0.45 THINKOS_TELE_CONG=0.15 THINKOS_TELE_UPVAR=0.04 THINKOS_TELE_TREAS=0.80 target/release/thinkos-node
run_node E env THINKOS_TELE_VOL=0.10 THINKOS_TELE_CONG=0.05 THINKOS_TELE_UPVAR=0.02 THINKOS_TELE_TREAS=0.95 target/release/thinkos-node

echo ">>> cluster started. Logs under state/nodes/*/node.log"
