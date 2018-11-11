use std::ffi::CString;
use std::os::raw::c_char;

pub fn say_hello() -> *mut c_char {
    CString::new("Hello Rust").unwrap().into_raw()
}
