#!/bin/sh
./target/release/node-template \
  --base-path /tmp/node1 \
  --chain ./poe-staging-raw.json \
  --ws-port 9946 \
  --rpc-port 9934 \
  --port 30334 \
  --validator \
  --name node1
