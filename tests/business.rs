//! https://github.com/dfinity/pocketic
use candid::{Principal, encode_one};
use pocket_ic::PocketIc;

mod util;

mod service;

const INIT_CYCLES: u128 = 2 * 10_u128.pow(12); // 2T cycles

const WASM_MODULE_NEXT: &[u8] = include_bytes!("../sources/source_opt.wasm.gz");

#[ignore]
#[test]
#[rustfmt::skip]
fn test_business_apis() {
    let pic = PocketIc::new();

    let default_identity = Principal::from_text("2ibo7-dia").unwrap();
    let alice_identity = Principal::from_text("uuc56-gyb").unwrap();
    let bob_identity = Principal::from_text("hqgi5-iic").unwrap(); // cspell: disable-line
    let carol_identity = Principal::from_text("jmf34-nyd").unwrap();
    let anonymous_identity = Principal::from_text("2vxsx-fae").unwrap();


    let canister_id = pic.create_canister_with_settings(Some(default_identity), None);
    pic.add_cycles(canister_id, INIT_CYCLES);

    pic.install_canister(canister_id, WASM_MODULE_NEXT.to_vec(), encode_one(None::<()>).unwrap(), Some(default_identity));

    use service::*;

    let pocketed_canister_id = PocketedCanisterId::new(canister_id, &pic);
    #[allow(unused)] let default = pocketed_canister_id.sender(default_identity);
    #[allow(unused)] let alice = pocketed_canister_id.sender(alice_identity);
    #[allow(unused)] let bob = pocketed_canister_id.sender(bob_identity);
    #[allow(unused)] let carol = pocketed_canister_id.sender(carol_identity);
    #[allow(unused)] let anonymous = pocketed_canister_id.sender(anonymous_identity);

    // 🚩 1 example business
    assert_eq!(alice.business_example_query().unwrap(), "".to_string());
    assert_eq!(default.business_example_query().unwrap(), "".to_string());
    assert_eq!(alice.business_example_set("test string".to_string()).unwrap_err().reject_message, "Permission 'BusinessExampleSet' is required".to_string());
    assert_eq!(default.business_example_set("test string".to_string()).unwrap(), ());
    assert_eq!(alice.business_example_query().unwrap(), "test string".to_string());
    assert_eq!(default.business_example_query().unwrap(), "test string".to_string());

    // 🚩 1.2 example business
    assert_eq!(default.business_example_count_query().unwrap(), 0);
    assert_eq!(default.business_example_count_set(1).unwrap(), ());
    assert_eq!(default.business_example_count_query().unwrap(), 1);
    assert!(default.business_example_count_set_panic_in_state(2).unwrap_err().reject_message.contains("panic in state"));
    assert_eq!(default.business_example_count_query().unwrap(), 1);
    assert!(default.business_example_count_set_panic_after_state(3).unwrap_err().reject_message.contains("panic after state"));
    assert_eq!(default.business_example_count_query().unwrap(), 1);

    // 🚩 2 test stable data
    assert_eq!(default.pause_replace(Some("reason".to_string())).unwrap(), ());
    assert!(default.pause_query().unwrap());
    pic.upgrade_canister(canister_id, WASM_MODULE_NEXT.to_vec(), encode_one(None::<()>).unwrap(), Some(default_identity)).unwrap();
    assert_eq!(default.pause_replace(None).unwrap(), ());
    assert!(!default.pause_query().unwrap());
    assert_eq!(default.business_example_query().unwrap(), "test string".to_string());
}
