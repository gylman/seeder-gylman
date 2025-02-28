#!/bin/bash
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
source $SCRIPT_PATH/env.sh

echo "add_sequencing_info"

echo "SEEDER_INTERNAL_RPC_URL is: $SEEDER_INTERNAL_RPC_URL"
echo "LIVENESS_PLATFORM is: $LIVENESS_PLATFORM"
echo "LIVENESS_SERVICE_PROVIDER is: $LIVENESS_SERVICE_PROVIDER"
echo "LIVENESS_RPC_URL is: $LIVENESS_RPC_URL"
echo "LIVENESS_WS_URL is: $LIVENESS_WS_URL"
echo "LIVENESS_CONTRACT_ADDRESS is: $LIVENESS_CONTRACT_ADDRESS"

curl --location $SEEDER_INTERNAL_RPC_URL \
--header 'Content-Type: application/json' \
--data '{
  "jsonrpc": "2.0",
  "method": "add_sequencing_info",
  "params": {
    "platform": "'"$LIVENESS_PLATFORM"'",
    "service_provider": "'"$LIVENESS_SERVICE_PROVIDER"'",
    "payload": {
      "liveness_rpc_url": "'"$LIVENESS_RPC_URL"'",
      "liveness_websocket_url": "'"$LIVENESS_WS_URL"'",
      "contract_address": "'"$LIVENESS_CONTRACT_ADDRESS"'"
    }
  },
  "id": 1
}'
echo ""
