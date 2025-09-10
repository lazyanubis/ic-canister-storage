#[allow(unused)]
use super::*;
#[allow(unused)]
pub use ic_canister_kit::identity::self_canister_id;
#[allow(unused)]
pub use ic_canister_kit::types::{CanisterId, PauseReason, UserId};
#[allow(unused)]
pub use std::collections::{HashMap, HashSet};
#[allow(unused)]
pub use std::fmt::Display;

#[allow(unused_variables)]
pub trait Business:
    Pausable<PauseReason>
    + ParsePermission
    + Permissable<Permission>
    + Recordable<Record, RecordTopic, RecordSearch>
    + Schedulable
    + ScheduleTask
    + StableHeap
{
    // 对外的查询接口
    fn business_hashed_find(&self) -> bool {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_files(&self) -> Vec<crate::stable::QueryFile> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_download(&self, path: String) -> Vec<u8> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_download_by(&self, path: String, offset: u64, size: u64) -> Vec<u8> {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // 对外的修改接口
    fn business_hashed_update(&mut self, hashed: bool) {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_upload(&mut self, args: Vec<crate::stable::UploadingArg>) {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_delete(&mut self, names: Vec<String>) {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_example_cell_update2(&mut self, test: String) {
        ic_cdk::trap("Not supported operation by this version.")
    }

    // 内部使用的接口
    fn business_assets_get_file(&self, path: &str) -> Option<&crate::stable::AssetFile> {
        ic_cdk::trap("Not supported operation by this version.")
    }
    fn business_assets_get(&self, hash: &crate::stable::HashDigest) -> Option<&crate::stable::AssetData> {
        ic_cdk::trap("Not supported operation by this version.")
    }
}

// 业务实现
impl Business for State {
    fn business_hashed_find(&self) -> bool {
        self.get().business_hashed_find()
    }
    fn business_files(&self) -> Vec<QueryFile> {
        self.get().business_files()
    }
    fn business_download(&self, path: String) -> Vec<u8> {
        self.get().business_download(path)
    }
    fn business_download_by(&self, path: String, offset: u64, size: u64) -> Vec<u8> {
        self.get().business_download_by(path, offset, size)
    }

    fn business_hashed_update(&mut self, hashed: bool) {
        self.get_mut().business_hashed_update(hashed)
    }
    fn business_upload(&mut self, args: Vec<UploadingArg>) {
        self.get_mut().business_upload(args)
    }
    fn business_delete(&mut self, names: Vec<String>) {
        self.get_mut().business_delete(names)
    }
    fn business_example_cell_update2(&mut self, test: String) {
        self.get_mut().business_example_cell_update2(test)
    }

    fn business_assets_get_file(&self, path: &str) -> Option<&AssetFile> {
        self.get().business_assets_get_file(path)
    }
    fn business_assets_get(&self, hash: &HashDigest) -> Option<&AssetData> {
        self.get().business_assets_get(hash)
    }
}
