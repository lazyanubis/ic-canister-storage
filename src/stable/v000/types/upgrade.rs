pub use ic_canister_kit::types::*;
use serde::{Deserialize, Serialize};

// 升级参数
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType)]
pub struct UpgradeArg {
    pub supers: Option<Vec<UserId>>,     // add new super administrators of not
    pub schedule: Option<DurationNanos>, // init scheduled task or not
}
