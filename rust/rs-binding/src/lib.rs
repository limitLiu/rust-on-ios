use std::os::raw::c_char;
use std::panic::{self, UnwindSafe};
use std::process;

fn stop_unwind<F: FnOnce() -> T + UnwindSafe, T>(f: F) -> T {
    match panic::catch_unwind(f) {
        Ok(tmp) => tmp,
        Err(_) => process::abort(),
    }
}

#[no_mangle]
pub extern "C" fn say_hello() -> *mut c_char {
    stop_unwind(rs::say_hello)
}
