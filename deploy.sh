#!/bin/bash

cargo clippy
cargo test

# dfx deploy --ic service # --mode=reinstall

dfx canister --network ic call service pause_replace "(opt \"for updating\")"
dfx deploy --network ic service
dfx canister --network ic call service pause_replace "(null)"
