use ic_canister_kit::stable;
use ic_canister_kit::types::*;

use super::ExampleCell;
use super::ExampleVec;

const MEMORY_ID_EXAMPLE_CELL: MemoryId = MemoryId::new(0); // 测试 Cell
const MEMORY_ID_EXAMPLE_VEC: MemoryId = MemoryId::new(1); // 测试 Vec
const MEMORY_ID_EXAMPLE_MAP: MemoryId = MemoryId::new(2); // 测试 Map
const MEMORY_ID_EXAMPLE_LOG_INDEX: MemoryId = MemoryId::new(3); // 测试 Log
const MEMORY_ID_EXAMPLE_LOG_DATA: MemoryId = MemoryId::new(4); // 测试 Log
const MEMORY_ID_EXAMPLE_PRIORITY_QUEUE: MemoryId = MemoryId::new(5); // 测试 PriorityQueue

pub(super) fn init_example_cell_data() -> StableCell<ExampleCell> {
    stable::init_cell_data(MEMORY_ID_EXAMPLE_CELL, ExampleCell::default())
}

pub(super) fn init_example_vec_data() -> StableVec<ExampleVec> {
    stable::init_vec_data(MEMORY_ID_EXAMPLE_VEC)
}

pub(super) fn init_example_map_data() -> StableBTreeMap<u64, String> {
    stable::init_map_data(MEMORY_ID_EXAMPLE_MAP)
}

pub(super) fn init_example_log_data() -> StableLog<String> {
    stable::init_log_data(MEMORY_ID_EXAMPLE_LOG_INDEX, MEMORY_ID_EXAMPLE_LOG_DATA)
}

pub(super) fn init_example_priority_queue_data() -> StablePriorityQueue<ExampleVec> {
    stable::init_priority_queue_data(MEMORY_ID_EXAMPLE_PRIORITY_QUEUE)
}
