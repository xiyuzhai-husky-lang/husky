use std::ffi::{c_char, CStr};

#[no_mangle]
pub extern "C" fn how_many_characters(s: *const c_char) -> u32 {
    let c_str: &CStr = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str: &str = c_str.to_str().unwrap();
    r_str.chars().count() as u32
}
