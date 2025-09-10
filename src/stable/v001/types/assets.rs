use std::collections::HashSet;

use candid::CandidType;
use ic_canister_kit::types::*;
use serde::{Deserialize, Serialize};

use crate::stable::v001::types::init_assets_data;

use super::{HashDigest, SliceOfHashDigest};

// ============================== 文件数据 ==============================

// 单个文件数据
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct AssetData {
    // 堆内存无数据，存放在稳定内存了
}

const MAX_BUCKET_SIZE: u64 = 1024 * 1024 * 2;

#[inline]
fn get_key(hash: &HashDigest, chunk: u32) -> SliceOfHashDigest {
    let mut key = [0; 36];
    key[..4].copy_from_slice(&chunk.to_be_bytes());
    key[4..].copy_from_slice(&hash.0);
    key
}

impl AssetData {
    pub fn from(hash: &HashDigest, data: Vec<u8>) -> Self {
        // 切片
        let size = data.len() as u64;
        let chunks = size / MAX_BUCKET_SIZE;
        let mut index = (0..chunks)
            .map(|i| {
                let key = get_key(hash, i as u32);
                (key, MAX_BUCKET_SIZE * i, MAX_BUCKET_SIZE)
            })
            .collect::<Vec<_>>();
        let remain = size - chunks * MAX_BUCKET_SIZE;
        if 0 < remain {
            let key = get_key(hash, chunks as u32);
            index.push((key, MAX_BUCKET_SIZE * chunks, remain))
        }

        // 插入数据
        for (key, offset, size) in index {
            let offset = offset as usize;
            let size = size as usize;
            let data = data[offset..offset + size].to_vec();
            ic_cdk::futures::spawn(async move {
                let mut assets = init_assets_data();
                assets.insert(key, data);
            });
        }

        // 返回空对象
        AssetData {}
    }
    pub fn slice(&self, hash: &HashDigest, data_size: u64, offset: usize, size: usize) -> std::borrow::Cow<'_, [u8]> {
        assert!(offset < data_size as usize);
        let offset_end = offset + size;
        assert!(offset_end <= data_size as usize);

        let mut result = vec![0; size];
        let mut cursor = 0;

        let assets = init_assets_data();

        let mut last_chunk = offset as u64 / MAX_BUCKET_SIZE;
        let mut offset = (offset as u64 - last_chunk * MAX_BUCKET_SIZE) as usize;
        let mut size = size;
        while 0 < size {
            let remain = MAX_BUCKET_SIZE as usize - offset; // 本次最多可以取这么多
            let fetch = std::cmp::min(size, remain); // 本次应该取的数据

            let key = get_key(hash, last_chunk as u32);

            let data = assets.get(&key);
            let data = ic_canister_kit::common::trap(data.ok_or("can not be"));

            result[cursor..cursor + fetch].copy_from_slice(&data[offset..offset + fetch]);

            cursor += fetch; // 修改结果写入位置
            last_chunk += 1; // 修改为下一个块
            offset = (offset + fetch) % MAX_BUCKET_SIZE as usize; // 修改并检查新的块偏移位置
            size -= fetch; // 修改剩余的数据
        }

        std::borrow::Cow::Owned(result)
    }
}

// 对外的路径数据 指向文件数据
#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct AssetFile {
    pub path: String,
    pub created: TimestampNanos,
    pub modified: TimestampNanos,
    pub headers: Vec<(String, String)>,
    pub hash: HashDigest,
    pub size: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default)]
pub struct HashedPath(pub(super) HashSet<String>);
