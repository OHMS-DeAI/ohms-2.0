#!/usr/bin/env bash
# Fund and/or create OHMS canisters on IC from cycles ledger
# Run from repo root: /home/okware/hackathons/OHMS/ohms 2.0

set -euo pipefail

NET=ic

# Use the okware identity
dfx identity use okware >/dev/null

ensure_canister () { # dir name init_cycles
  pushd "$1" >/dev/null
  local NAME="$2" INIT="${3:-0}"
  if dfx canister id "$NAME" --network "$NET" >/dev/null 2>&1; then
    CID=$(dfx canister id "$NAME" --network "$NET")
  else
    dfx canister create "$NAME" --with-cycles "$INIT" --network "$NET"
    CID=$(dfx canister id "$NAME" --network "$NET")
  fi
  echo "$CID"
  popd >/dev/null
}

# 1) ohms-model (storage-heavy): target ≈ 5T (you already created it with ~1.2T)
OM_CID=$(ensure_canister "ohms-model" "ohms_model" "0")
dfx cycles top-up "$OM_CID" 3.8T --network "$NET"

# 2) ohms-agent (compute/memory moderate): ~0.8T
OA_CID=$(ensure_canister "ohms-agent" "ohms_agent" "0.8T")

# 3) ohms-coordinator (light control-plane): ~0.6T
OC_CID=$(ensure_canister "ohms-coordinator" "ohms_coordinator" "0.6T")

# 4) ohms-econ (ledger-ish logic, moderate): ~0.6T
OE_CID=$(ensure_canister "ohms-econ" "ohms_econ" "0.6T")

# (Optional) ohms-ui (asset canister): ~0.2T if you deploy it on IC
if [ -d "ohms-ui" ]; then
  if dfx canister id ohms_ui --network "$NET" >/dev/null 2>&1; then
    OUI_CID=$(dfx canister id ohms_ui --network "$NET")
  else
    pushd ohms-ui >/dev/null
    dfx canister create ohms_ui --with-cycles 0.6T --network "$NET"
    OUI_CID=$(dfx canister id ohms_ui --network "$NET")
    popd >/dev/null
  fi
fi

# 5) ohms-website (static website canister): ~0.3T for mainnet deployment
if [ -d "ohms-website" ]; then
  if dfx canister id ohms_website --network "$NET" >/dev/null 2>&1; then
    OW_CID=$(dfx canister id ohms_website --network "$NET")
  else
    pushd ohms-website >/dev/null
    dfx canister create ohms_website --with-cycles 0.8T --network "$NET"
    OW_CID=$(dfx canister id ohms_website --network "$NET")
    popd >/dev/null
  fi
fi

echo "Canisters:"
echo "ohms_model       = $OM_CID"
echo "ohms_agent       = ${OA_CID:-N/A}"
echo "ohms_coordinator = ${OC_CID:-N/A}"
echo "ohms_econ        = ${OE_CID:-N/A}"
echo "ohms_ui          = ${OUI_CID:-N/A}"
echo "ohms_website     = ${OW_CID:-N/A}"

echo "Ledger balance (remaining):"
dfx cycles --network "$NET" balance
