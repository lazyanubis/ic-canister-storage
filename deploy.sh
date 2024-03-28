#!!/bin/bash

cargo clippy

# 部署代码
# dfx deploy --network ic storage --mode=reinstall --yes

# dfx canister --network ic call storage pause_replace "(opt \"for updating\")"
# dfx deploy --network ic storage
# dfx canister --network ic call storage pause_replace "(null)"
# dfx canister --network ic call storage business_hashed_update "(true)"

# dfx deploy --network local storage --mode=reinstall --yes
dfx canister --network local call storage pause_replace "(opt \"for updating\")"
dfx deploy --network local storage
dfx canister --network local call storage pause_replace "(null)"
dfx canister --network local call storage business_hashed_update "(true)"

# 上传资源文件
RUST_BACKTRACE=1 cargo test upload -- --nocapture
