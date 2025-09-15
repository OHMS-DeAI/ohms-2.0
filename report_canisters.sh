#!/usr/bin/env bash
# List OHMS canister IDs and per-canister cycle balances
# Usage: ./report_canisters.sh [ic|local]   (default: ic)

set -euo pipefail
NET=${1:-ic}

dfx identity get-principal >/dev/null 2>&1 || {
  echo "dfx identity not initialized. Run 'dfx identity use okware' first." >&2
  exit 1
}

report_canister () { # dir name
  local DIR="$1" NAME="$2"
  if [ ! -d "$DIR" ]; then return 0; fi
  pushd "$DIR" >/dev/null
  if dfx canister id "$NAME" --network "$NET" >/dev/null 2>&1; then
    CID=$(dfx canister id "$NAME" --network "$NET")
    STATUS=$(dfx canister status "$NAME" --network "$NET" | sed -n '/Cycle/Ip; /Balance:/Ip')
    echo "$NAME=$(printf '%s' "$CID")"
    echo "$STATUS" | sed 's/^/  /'
  else
    echo "$NAME=N/A (not created on $NET)"
  fi
  popd >/dev/null
}

echo "Network: $NET"
echo "Identity: $(dfx identity whoami)"
echo

report_canister "ohms-model"       "ohms_model"
report_canister "ohms-agent"       "ohms_agent"
report_canister "ohms-coordinator" "ohms_coordinator"
report_canister "ohms-econ"        "ohms_econ"
report_canister "ohms-ui"          "ohms_ui"

echo
echo "Cycles ledger balance:"
dfx cycles --network "$NET" balance || true

