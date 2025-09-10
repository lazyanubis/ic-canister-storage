use ic_canister_kit::types::*;
use serde::{Deserialize, Serialize};

// 初始化参数
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArg {
    pub supers: Option<Vec<UserId>>,     // init super administrators or deployer
    pub schedule: Option<DurationNanos>, // init scheduled task or not
}
