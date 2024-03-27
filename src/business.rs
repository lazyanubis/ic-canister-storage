#[allow(unused)]
use ic_canister_kit::common::once::call_once_guard;
#[allow(unused)]
use ic_canister_kit::identity::caller;

#[allow(unused)]
use crate::stable::*;
#[allow(unused)]
use crate::types::*;

// 查询
#[ic_cdk::query(guard = "has_business_example_query")]
fn business_example_query() -> String {
    with_state(|s| s.business_example_query())
}

// 修改
#[ic_cdk::update(guard = "has_business_example_set")]
fn business_example_set(test: String) {
    let _guard = call_once_guard(); // post 接口应该拦截

    let caller = caller();
    let arg_content = format!("set test: {}", test); // * 记录参数内容

    with_mut_state(
        |s, _done| {
            s.business_example_update(test);
        },
        caller,
        RecordTopics::Example.topic(),
        arg_content,
    )
}
