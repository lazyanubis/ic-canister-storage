pub use ic_canister_kit::types::*;
use serde::{Deserialize, Serialize};

#[allow(unused)]
pub use super::super::{Business, MutableBusiness, ParsePermission, ScheduleTask};

#[allow(unused)]
pub use super::super::business::*;
#[allow(unused)]
pub use super::business::*;
#[allow(unused)]
pub use super::permission::*;
#[allow(unused)]
pub use super::schedule::schedule_task;

mod _init;
pub use _init::*;
mod _upgrade;
pub use _upgrade::*;
mod _topic;
pub use _topic::*;
mod _canister_kit;
pub use _canister_kit::*;

// 业务类型
mod common;
pub use common::*;
mod assets;
pub use assets::*;
mod upload;
pub use upload::*;
mod stable;
use stable::*;

// 能序列化的和不能序列化的放在一起
// 其中不能序列化的采用如下注解
// #[serde(skip)] 默认初始化方式
// #[serde(skip, default="init_xxx_data")] 指定初始化方式
// ! 如果使用 ic-stable-structures 提供的稳定内存，不能变更 memory_id 的使用类型，否则会出现各个版本不兼容，数据会被清空
#[derive(Serialize, Deserialize)]
pub struct InnerState {
    pub canister_kit: CanisterKit, // 框架需要的数据 // ? 堆内存 序列化

    // 业务数据
    pub hashed: bool, // 是否相信上传的 hash 值，true -> 直接采用接口传递的 hash 值， false -> 数据上传完成后，需要罐子再 hash 一次 // ? 堆内存 序列化

    pub assets: HashMap<HashDigest, AssetData>, // key 是 hash // ? 堆内存 序列化
    pub files: HashMap<String, AssetFile>,      // key 是 path // ? 堆内存 序列化
    hashes: HashMap<HashDigest, HashedPath>, // key 是 hash, value 是 path, 没有 path 的数据是没有保存意义的 // ? 堆内存 序列化

    uploading: HashMap<String, UploadingFile>, // key 是 path // ? 堆内存 序列化
}

impl Default for InnerState {
    fn default() -> Self {
        ic_cdk::println!("InnerState::default()");
        Self {
            canister_kit: Default::default(),

            // 业务数据
            hashed: Default::default(),

            assets: Default::default(),
            files: Default::default(),
            hashes: Default::default(),

            uploading: Default::default(),
        }
    }
}

impl InnerState {
    pub fn do_init(&mut self, _arg: InitArg) {
        // maybe do something
    }

    pub fn do_upgrade(&mut self, _arg: UpgradeArg) {
        // maybe do something
    }

    fn hash(file: &UploadingFile) -> HashDigest {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(&file.data[0..(file.size as usize)]);
        let digest: [u8; 32] = hasher.finalize().into();
        HashDigest(digest)
    }
    fn put_file(&mut self, path: String, headers: Vec<(String, String)>, hash: HashDigest, size: u64) {
        // 3. 插入 files: path -> hash
        let now = ic_canister_kit::times::now();
        if let Some(exist) = self.files.get_mut(&path) {
            exist.modified = now;
            exist.headers = headers;
            exist.hash = hash;
        } else {
            self.files.insert(
                path.clone(),
                AssetFile {
                    path: path.clone(),
                    created: now,
                    modified: now,
                    headers,
                    hash,
                    size,
                },
            );
        }

        // 4. 插入 hashes: hash -> [path]
        self.hashes.entry(hash).or_default();
        if let Some(hash_path) = self.hashes.get_mut(&hash) {
            hash_path.0.insert(path);
        }
    }
    fn put_assets(&mut self, file: UploadingFile) {
        // 0. 先清空同路径的文件
        self.clean_file(&file.path);
        // 1. 计算 hash
        let hash = if self.hashed {
            file.hash // hashed true 直接使用
        } else {
            Self::hash(&file) // hashed false 要计算一次
        };
        // 2. 插入 assets: hash -> data
        self.assets
            .entry(hash)
            .or_insert_with(|| AssetData::from(&hash, file.data));

        self.put_file(file.path, file.headers, hash, file.size); // 存完毕 assets 数据了，然后要对文件建立代理索引
    }
    pub fn clean_file(&mut self, path: &String) {
        // 1. 删除文件
        let file = match self.files.remove(path) {
            Some(file) => file.clone(),
            None => return,
        };
        // 2. 清除 hashes
        if let Some(HashedPath(path_set)) = self.hashes.get_mut(&file.hash) {
            path_set.remove(&file.path);
            if path_set.is_empty() {
                // 需要清空
                self.hashes.remove(&file.hash);
                // 4. 清空 assets
                self.assets.remove(&file.hash);
            }
        }
    }
    pub fn files(&self) -> Vec<QueryFile> {
        self.files
            .iter()
            .map(|(path, file)| QueryFile {
                path: path.to_string(),
                size: file.size,
                headers: file.headers.clone(),
                created: file.created,
                modified: file.modified,
                hash: file.hash.hex(),
            })
            .collect()
    }
    pub fn download(&self, path: String) -> Vec<u8> {
        use ic_canister_kit::common::trap;
        let file = trap(self.files.get(&path).ok_or("File not found"));
        let asset = trap(self.assets.get(&file.hash).ok_or("File not found"));
        asset.slice(&file.hash, file.size, 0, file.size as usize).to_vec()
    }
    pub fn download_by(&self, path: String, offset: u64, size: u64) -> Vec<u8> {
        use ic_canister_kit::common::trap;
        let file = trap(self.files.get(&path).ok_or("File not found"));
        let asset = trap(self.assets.get(&file.hash).ok_or("File not found"));
        asset
            .slice(&file.hash, file.size, offset as usize, size as usize)
            .to_vec()
    }

    fn chunks(arg: &UploadingArg) -> u32 {
        let mut chunks = arg.size / arg.chunk_size as u64; // 完整的块数
        if chunks * (arg.chunk_size as u64) < arg.size {
            chunks += 1;
        }
        chunks as u32
    }
    fn offset(arg: &UploadingArg) -> (usize, usize) {
        let chunks = Self::chunks(arg);
        let offset = arg.chunk_size as u64 * arg.index as u64;
        let mut offset_end = offset + arg.chunk_size as u64;
        if arg.index == chunks - 1 {
            offset_end = arg.size;
        }
        (offset as usize, offset_end as usize)
    }
    fn check_path_and_headers(arg: &UploadingArg) {
        // 1. 检查 路径名
        assert!(!arg.path.is_empty(), "must has path");
        assert!(arg.path.starts_with('/'), "path must start with /");
        // 2. 检查 headers
        for (name, value) in &arg.headers {
            assert!(name.len() <= 64, "header name is too large");
            assert!(value.len() <= 1024 * 8, "header value is too large");
        }
    }
    fn check_size_and_data(arg: &UploadingArg) {
        // 3. 检查 size
        assert!(0 < arg.size, "size can not be 0");
        assert!(
            arg.size <= 1024 * 1024 * 1024 * 2, // 最大文件 2G
            "size must less than 4GB"
        );
        // 4. 检查 chunk_size
        assert!(0 < arg.chunk_size, "chunk size can not be 0");
        // 5. 检查 index
        let chunks = Self::chunks(arg);
        assert!(arg.index < chunks, "wrong index");
        // 6. 检查 data
        if arg.index < chunks - 1 || arg.size == arg.chunk_size as u64 * chunks as u64 {
            // 是前面完整的 或者 整好整除
            assert!(arg.chunk.len() as u32 == arg.chunk_size, "wrong chunk length");
        } else {
            // 是剩下的
            assert!(
                arg.chunk.len() as u64 == arg.size % (arg.chunk_size as u64),
                "wrong chunk length"
            );
        }
    }
    fn assure_uploading(&mut self, arg: &UploadingArg) {
        let chunks = Self::chunks(arg);
        if let Some(exist) = self.uploading.get(&arg.path) {
            // 已经有这个文件了, 需要比较一下, 参数是否一致
            assert!(exist.path == arg.path, "wrong path, system error.");
            if exist.hash != arg.hash // hash 不一致
                || exist.size != arg.size // 文件长度不一致
                || exist.data.len() != arg.size as usize // 暂存长度不对
                || exist.chunk_size != arg.chunk_size
                || exist.chunks != chunks
                || exist.chunked.len() != chunks as usize
            {
                // 非致命错误, 清空原来的文件就好
                self.files.remove(&arg.path);
            }
        } else {
            // 原来没有的情况下
            self.uploading.insert(
                arg.path.clone(),
                UploadingFile {
                    path: arg.path.clone(),
                    headers: arg.headers.clone(),
                    hash: arg.hash,
                    data: vec![0; arg.size as usize],
                    size: arg.size,
                    chunk_size: arg.chunk_size,
                    chunks,
                    chunked: vec![false; chunks as usize],
                },
            );
        }
    }
    pub fn put_uploading(&mut self, arg: UploadingArg) {
        // 1. 检查参数是否有效
        Self::check_path_and_headers(&arg);

        // 2. 如果 hashed true 并且已经存在改 hash 值文件了，直接保存即可
        if self.hashed
            && self.assets.contains_key(&arg.hash)
            && let Some(path) = self.hashes.get(&arg.hash)
            && let Some(path) = path.0.iter().next()
            && let Some(file) = self.files.get(path)
        {
            self.put_file(arg.path, arg.headers, arg.hash, file.size); // size 不可信，只能从已存在的文件内容中查找
            return;
        }

        // 3. 检查其他参数
        Self::check_size_and_data(&arg);

        // 4. 确保有缓存空间
        self.assure_uploading(&arg); // 确保该文件已经存在缓存数据了

        // 5. 找的对应的缓存文件
        let mut done = false;
        if let Some(file) = self.uploading.get_mut(&arg.path) {
            // 3. 复制有效的信息
            let (offset, offset_end) = Self::offset(&arg);
            file.headers = arg.headers;
            file.data.splice(offset..offset_end, arg.chunk); // 复制内容
            file.chunked[arg.index as usize] = true;

            // 4. 是否已经完整
            done = file.chunked.iter().all(|c| *c);
        }
        if done && let Some(file) = self.uploading.remove(&arg.path) {
            // 处理这个已经完成的数据
            self.put_assets(file);
        }
    }
    pub fn clean_uploading(&mut self, path: &String) {
        self.files.remove(path);
    }
}
