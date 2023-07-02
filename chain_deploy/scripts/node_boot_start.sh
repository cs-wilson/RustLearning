#!/bin/sh
./target/release/node-template \
  --node-key ef64b417ad51e74b960d146882b82bfa3940ccea18b07e4a988675819e1aed2c \
  --base-path /tmp/node0 \
  --chain ./poe-staging-raw.json \
  --ws-port 9945 \
  --rpc-port 9933 \
  --port 30333 \
  --validator \
  --name node0
