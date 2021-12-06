#!/usr/bin/env bash
_url="http://127.0.0.1:1317/cosmos/staking/v1beta1/validators" 
_now=$(date '+%Y-%m-%d')
_filename=data/pfc-${_now}.json
curl -o data/${_filename} ${_url}/terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q/delegations?pagination.limit=4000
md5=$(md5sum ${_filename} | cut -d ' ' -f 1 )
cargo run --release ${md5} ${_filename} results/winners.json