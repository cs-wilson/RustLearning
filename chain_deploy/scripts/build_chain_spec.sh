#!/bin/sh
./target/release/node-template build-spec --chain staging > poe-staging.json
./target/release/node-template build-spec --chain=poe-staging.json --raw > poe-staging-raw.json
