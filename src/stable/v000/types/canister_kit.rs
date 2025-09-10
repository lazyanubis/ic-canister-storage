use ic_canister_kit::types::*;
use serde::{Deserialize, Serialize};

// 框架需要的数据结构
#[derive(Serialize, Deserialize, Default)]
pub struct CanisterKit {
    pub pause: Pause,             // 记录维护状态 // ? 堆内存 序列化
    pub permissions: Permissions, // 记录自身权限 // ? 堆内存 序列化
    pub records: Records,         // 记录操作记录 // ? 堆内存 序列化
    pub schedule: Schedule,       // 记录定时任务 // ? 堆内存 序列化
}
