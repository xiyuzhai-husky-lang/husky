#[no_mangle]
pub extern "C" fn addition(a: u32, b: u32) -> u32 {
    unsafe { actual_addition(a, b) }
}

extern "C" {
    fn actual_addition(a: u32, b: u32) -> u32;
}
