use ic_canister_kit::stable;
use ic_canister_kit::types::*;

use super::SliceOfHashDigest;

const MEMORY_ID_ASSETS: MemoryId = MemoryId::new(0); // 存放实际文件，hash 为键

pub(super) fn init_assets_data() -> StableBTreeMap<SliceOfHashDigest, Vec<u8>> {
    stable::init_map_data(MEMORY_ID_ASSETS)
}
