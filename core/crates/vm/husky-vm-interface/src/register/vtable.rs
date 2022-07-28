use super::*;
use std::ffi::c_char;

// order matters!
#[repr(C)]
pub struct __RegisterVTable {
    pub typename_str: *const c_char,
    pub primitive_value_to_bool: Option<fn(data: __RegisterData) -> bool>,
    pub primitive_value_to_box: Option<fn(data: *mut __RegisterData) -> *mut ()>,
    pub clone: Option<fn(data: *mut ()) -> *mut ()>,
    pub drop: Option<fn(data: *mut ())>,
}

unsafe impl Sync for __RegisterVTable {}
unsafe impl Send for __RegisterVTable {}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<Option<*const ()>>(), 16);
    assert_eq!(
        std::mem::size_of::<Option<std::ptr::NonNull<*const ()>>>(),
        8
    );
    assert_eq!(
        std::mem::size_of::<Option<fn(data: __RegisterData) -> bool>>(),
        8
    )
}
