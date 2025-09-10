use candid::CandidType;
use serde::{Deserialize, Serialize};

use super::HashDigest;

// =========== 上传过程中的对象 ===========

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct UploadingFile {
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub hash: HashDigest, // hash 值，在 hashed 为 false 的情况下不使用
    pub data: Vec<u8>,    // 上传中的数据

    pub size: u64,          // 文件大小
    pub chunk_size: u32,    // 块大小 块分割的大小
    pub chunks: u32,        // 需要上传的次数
    pub chunked: Vec<bool>, // 记录每一个块的上传状态
}

// 上传参数
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct UploadingArg {
    pub path: String,
    pub headers: Vec<(String, String)>, // 使用的 header
    pub hash: HashDigest,               // hash 值，在 hashed 为 false 的情况下不使用
    pub size: u64,                      // 文件大小
    pub chunk_size: u32,                // 块大小 块分割的大小
    pub index: u32,                     // 本次上传的数据
    pub chunk: Vec<u8>,                 // 上传中的数据
}
