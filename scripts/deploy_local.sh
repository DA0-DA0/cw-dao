#!/bin/bash

# Run this from the root repo directory

## CONFIG
# NOTE: you will need to update these to deploy on different network
IMAGE_TAG="pr-135" # moneta
BINARY='docker exec -i cosmwasm junod'
DENOM='ujunox'
CHAIN_ID='testing'
RPC='http://localhost:26657/'
TXFLAG="--gas-prices 0.1$DENOM --gas auto --gas-adjustment 1.5 -y -b block --chain-id $CHAIN_ID --node $RPC"

if [ "$1" = "" ]
then
  echo "Usage: $0 1 arg required, wasm address. See \"Deploying in a development environment\" in README."
  exit
fi

# Deploy junod in Docker
docker kill cosmwasm

docker volume rm -f junod_data

# Run junod setup script
docker run --rm -it \
    -e STAKE_TOKEN=$DENOM \
    -e PASSWORD=xxxxxxxxx \
    --mount type=volume,source=junod_data,target=/root \
    ghcr.io/cosmoscontracts/juno:$IMAGE_TAG /opt/setup_junod.sh $1

# Add custom app.toml to junod_data volume
docker run -v junod_data:/root --name helper busybox true
docker cp docker/app.toml helper:/root/.juno/config/app.toml
docker cp docker/config.toml helper:/root/.juno/config/config.toml
docker rm helper

# Start junod
docker run --rm -d --name cosmwasm -p 26657:26657 -p 26656:26656 -p 1317:1317 \
    --mount type=volume,source=junod_data,target=/root \
    ghcr.io/cosmoscontracts/juno:$IMAGE_TAG /opt/run_junod.sh

# Compile dao-dao code
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.3

# Copy binaries to docker container
docker cp artifacts/cw3_dao.wasm cosmwasm:/cw3_dao.wasm
docker cp artifacts/cw20_base.wasm cosmwasm:/cw20_base.wasm
docker cp artifacts/cw3_multisig.wasm cosmwasm:/cw3_multisig.wasm
docker cp artifacts/stake_cw20.wasm cosmwasm:/stake_cw20.wasm

# Sleep while waiting for chain to post genesis block
sleep 3

echo "Address to deploy contracts: $1"
echo "TX Flags: $TXFLAG"


#### CW20-GOV ####
# Upload cw20 contract code
CW20_CODE=$(echo xxxxxxxxx | $BINARY tx wasm store "/cw20_base.wasm" --from validator $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[0].value')
echo "CW20 code:"
echo $CW20_CODE

STAKE_CODE=$(echo xxxxxxxxx | $BINARY tx wasm store "/stake_cw20.wasm" --from validator $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[0].value')
echo "Stake code:"
echo $STAKE_CODE

# # Instantiate cw20 contract
# CW20_INIT='{
#   "name": "Crab Coin",
#   "symbol": "CRAB",
#   "decimals": 6,
#   "initial_balances": [{"address":"'"$1"'","amount":"1000000000"}]
# }'
# echo "$CW20_INIT"
# echo xxxxxxxxx | $BINARY tx wasm instantiate $CW20_CODE "$CW20_INIT" --from "validator" --label "gov token" $TXFLAG

# # Get cw20 contract address
# CW20_CONTRACT=$($BINARY q wasm list-contract-by-code $CW20_CODE --output json | jq -r '.contracts[-1]')

#### CW-DAO ####
# Upload cw-dao contract code
CW3_DAO_CODE=$(echo xxxxxxxxx | $BINARY tx wasm store "/cw3_dao.wasm" --from validator $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[0].value')
echo "CW3_DAO_CODE:"
echo $CW3_DAO_CODE

# Instantiate cw-dao contract using existing token
# CW3_DAO_INIT='{
#   "name": "DAO DAO",
#   "description": "A DAO that makes DAO tooling",
#   "gov_token": {
#     "use_existing_cw20": {
#       "addr": "'"$CW20_CONTRACT"'"
#     }
#   },
#   "threshold": {
#     "absolute_percentage": {
#         "percentage": "0.5"
#     }
#   },
#   "max_voting_period": {
#     "height": 100
#   },
#   "proposal_deposit_amount": "0"
# }'

# DAO contract instantiates its own token
CW3_DAO_INIT='{
  "name": "DAO DAO",
  "description": "A DAO that makes DAO tooling",
  "gov_token": {
    "instantiate_new_cw20": {
      "cw20_code_id": '$CW20_CODE',
      "stake_contract_code_id": '$STAKE_CODE',
      "label": "DAO DAO v0.1.1",
      "msg": {
        "name": "daodao",
        "symbol": "DAO",
        "decimals": 6,
        "initial_balances": [{"address":"'"$1"'","amount":"1000000000"}]
      }
    }
  },
  "threshold": {
    "absolute_percentage": {
        "percentage": "0.5"
    }
  },
  "max_voting_period": {
    "height": 100
  },
  "proposal_deposit_amount": "0"
}'
echo $CW3_DAO_INIT | jq .

DAO_INIT=$(echo xxxxxxxxx | $BINARY tx wasm instantiate "$CW3_DAO_CODE" "$CW3_DAO_INIT" --from validator --label "cw-dao" $TXFLAG --output json)
echo $DAO_INIT | jq .

CW3_DAO_CONTRACT=$($BINARY q wasm list-contract-by-code $CW3_DAO_CODE --output json | jq -r '.contracts[-1]')

# Download cw4-group contracts
cd scripts
curl -LO https://github.com/CosmWasm/cw-plus/releases/download/v0.10.2/cw4_group.wasm

# Copy wasm to docker
docker cp cw4_group.wasm cosmwasm:/cw4_group.wasm

# Upload cw3-multisig and cw4-group code
echo xxxxxxxxx | $BINARY tx wasm store "/cw3_multisig.wasm" --from validator $TXFLAG
echo xxxxxxxxx | $BINARY tx wasm store "/cw4_group.wasm" --from validator $TXFLAG

# Send some coins to the dao contract to initializae its
# treasury. Unless this is done the DAO will be unable to perform
# actions like executing proposals that require it to pay gas fees.
$BINARY tx bank send validator $CW3_DAO_CONTRACT 9000000$DENOM --chain-id testing $TXFLAG -y

# Print out config variables
printf "\n ------------------------ \n"
printf "Config Variables \n\n"

echo "NEXT_PUBLIC_DAO_TOKEN_CODE_ID=$CW20_CODE"
echo "NEXT_PUBLIC_DAO_CONTRACT_CODE_ID=$CW3_DAO_CODE"
echo "NEXT_PUBLIC_MULTISIG_CODE_ID=3"
echo "NEXT_PUBLIC_C4_GROUP_CODE_ID=4"
echo "NEXT_PUBLIC_DAO_CONTRACT_ADDRESS=$CW3_DAO_CONTRACT"
