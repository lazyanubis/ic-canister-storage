#![cfg(test)]

#[allow(unused)]
pub fn print_backtrace() {
    let backtraces = format!("{}", std::backtrace::Backtrace::force_capture());
    let backtraces = backtraces.split('\n').collect::<Vec<_>>();
    let position = backtraces.iter().position(|b| b.contains("5: ")).unwrap();
    eprintln!("{}", backtraces[position + 1]);
}
