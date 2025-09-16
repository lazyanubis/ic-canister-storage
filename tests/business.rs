//! https://github.com/dfinity/pocketic
use candid::{Principal, encode_one};
use pocket_ic::PocketIc;

mod service;

// 2T cycles
const INIT_CYCLES: u128 = 2_000_000_000_000;

const WASM_MODULE: &[u8] = include_bytes!("../sources/source_opt.wasm");

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

    pic.install_canister(canister_id, WASM_MODULE.to_vec(), encode_one(None::<()>).unwrap(), Some(default_identity));

    use service::*;

    let pocketed_canister_id = PocketedCanisterId::new(canister_id, &pic);
    #[allow(unused)] let default = pocketed_canister_id.sender(default_identity);
    #[allow(unused)] let alice = pocketed_canister_id.sender(alice_identity);
    #[allow(unused)] let bob = pocketed_canister_id.sender(bob_identity);
    #[allow(unused)] let carol = pocketed_canister_id.sender(carol_identity);
    #[allow(unused)] let anonymous = pocketed_canister_id.sender(anonymous_identity);

    // 🚩 1 business query
    assert_eq!(alice.business_files().unwrap(), vec![]);
    assert_eq!(alice.business_download("/123.txt".to_string()).unwrap_err().reject_message.contains("File not found"), true);
    assert_eq!(alice.business_download("/456.txt".to_string()).unwrap_err().reject_message.contains("File not found"), true);

    // 🚩 2 business upload
    assert_eq!(alice.business_upload(vec![UploadingArg { hash: vec![0; 32].into(), chunk: vec![1, 2, 3].into(), path: "/123.txt".to_string(), size: 3, headers: vec![], index: 0, chunk_size: 3 }]).unwrap_err().reject_message, "Permission 'BusinessUpload' is required".to_string());
    assert_eq!(default.business_upload(vec![UploadingArg { hash: vec![0; 32].into(), chunk: vec![1, 2, 3].into(), path: "/123.txt".to_string(), size: 3, headers: vec![], index: 0, chunk_size: 3 }]).unwrap(), ());
    assert_eq!(alice.business_files().unwrap().pop().unwrap().hash, "039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81".to_string());
    assert_eq!(alice.business_download("/123.txt".to_string()).unwrap(), vec![1, 2, 3]);
}
