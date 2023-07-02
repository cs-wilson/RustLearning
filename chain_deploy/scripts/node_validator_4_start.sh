#!/bin/sh
./target/release/node-template \
  --base-path /tmp/node4 \
  --chain ./poe-staging-raw.json \
  --ws-port 9949 \
  --rpc-port 9937 \
  --port 30337 \
  --validator \
  --name node4
