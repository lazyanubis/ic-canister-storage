#!/bin/bash

cargo clippy
cargo test

# dfx deploy --ic template # --mode=reinstall

dfx canister --network ic call template pause_replace "(opt \"for updating\")"
dfx deploy --network ic template
dfx canister --network ic call template pause_replace "(null)"
