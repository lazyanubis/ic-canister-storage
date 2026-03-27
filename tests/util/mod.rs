#![cfg(test)]
use candid::Principal;

#[allow(unused)]
pub fn print_backtrace() {
    let backtraces = format!("{}", std::backtrace::Backtrace::force_capture());
    let backtraces = backtraces.split('\n').collect::<Vec<_>>();
    let position = backtraces.iter().position(|b| b.contains("5: ")).unwrap();
    eprintln!("{}", backtraces[position + 1]);
}

pub fn get_identity() -> (Principal, Principal, Principal, Principal, Principal) {
    let default_identity = Principal::from_text("2ibo7-dia").unwrap();
    let alice_identity = Principal::from_text("uuc56-gyb").unwrap();
    let bob_identity = Principal::from_text("hqgi5-iic").unwrap(); // cspell: disable-line
    let carol_identity = Principal::from_text("jmf34-nyd").unwrap();
    let anonymous_identity = Principal::from_text("2vxsx-fae").unwrap();
    (
        default_identity,
        alice_identity,
        bob_identity,
        carol_identity,
        anonymous_identity,
    )
}
