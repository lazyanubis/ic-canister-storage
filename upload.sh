#!!/bin/bash

RUST_BACKTRACE=1 cargo test upload -- --nocapture
