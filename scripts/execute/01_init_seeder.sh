#!/bin/bash
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

echo "SCRIPT_PATH is $SCRIPT_PATH"

source $SCRIPT_PATH/env.sh

echo "DATA_PATH is set to: $DATA_PATH"

# ❌ Do not remove the entire folder (Docker bind mount issue)
# rm -rf $DATA_PATH  

# ✅ Instead, remove only the contents
find $DATA_PATH -mindepth 1 -delete

echo "DATA_PATH after rm -rf is $DATA_PATH"
echo "BIN_PATH after rm -rf is $BIN_PATH"

$BIN_PATH init --path $DATA_PATH

# Wait until CONFIG_FILE_PATH is available
while [ ! -f "$CONFIG_FILE_PATH" ]; do
  echo "Waiting for $CONFIG_FILE_PATH to be created..."
  sleep 1
done

sed -i.temp "s|0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80|$SEEDER_PRIVATE_KEY|g" $PRIVATE_KEY_PATH

sed -i.temp "s|seeder_external_rpc_url = \"http://127.0.0.1:6000\"|seeder_external_rpc_url = \"$SEEDER_EXTERNAL_RPC_URL\"|g" $CONFIG_FILE_PATH
sed -i.temp "s|seeder_internal_rpc_url = \"http://127.0.0.1:6001\"|seeder_internal_rpc_url = \"$SEEDER_INTERNAL_RPC_URL\"|g" $CONFIG_FILE_PATH

rm $CONFIG_FILE_PATH.temp
