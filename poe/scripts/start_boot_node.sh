#!/bin/sh
./target/release/node-template \
  --node-key 21529f1ee69fdd99c2b5657bc45e2e8cd976581fdac07d733448533cc39856f9 \
  --base-path /tmp/node0 \
  --chain ./poe-chain-raw.json \
  --ws-port 9945 \
  --rpc-port 9933 \
  --port 30333 \
  --validator \
  --name node0
