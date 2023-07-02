#!/bin/sh
./target/release/node-template \
  --base-path /tmp/node3 \
  --chain ./poe-staging-raw.json \
  --ws-port 9948 \
  --rpc-port 9936 \
  --port 30336 \
  --validator \
  --name node3
