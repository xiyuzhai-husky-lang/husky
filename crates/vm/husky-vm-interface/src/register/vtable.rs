use super::*;
use std::ffi::c_void;

// order matters!
#[repr(C)]
pub struct __RegisterTyVTable {
    pub primitive_value_to_bool: Option<unsafe extern "C" fn(data: __RegisterData) -> bool>,
    pub primitive_ref_to_bool: Option<unsafe extern "C" fn(data_ptr: *mut c_void) -> bool>,
    pub primitive_value_to_box: Option<unsafe extern "C" fn(data: __RegisterData) -> *mut c_void>,
    pub clone: unsafe extern "C" fn(data: *mut c_void) -> *mut c_void,
    pub drop: unsafe extern "C" fn(data: *mut c_void),
    pub eq: unsafe extern "C" fn(this: &c_void, this: &c_void) -> bool,
    pub assign: unsafe extern "C" fn(args: *mut __RegularValue),
    pub typename_str_hash_u64: u64,
    pub typename_str: &'static str,
}

impl PartialEq for __RegisterTyVTable {
    fn eq(&self, other: &Self) -> bool {
        self.typename_str_hash_u64 == other.typename_str_hash_u64
    }
}

impl Eq for __RegisterTyVTable {}

unsafe impl Sync for __RegisterTyVTable {}

unsafe impl Send for __RegisterTyVTable {}

impl std::fmt::Debug for __RegisterTyVTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("__RegisterTyVTable")
            .field("typename_str", &self.typename_str)
            .finish()
    }
}

#[test]
fn test_size() {
    assert_eq!(std::mem::size_of::<Option<*const c_void>>(), 16);
    assert_eq!(
        std::mem::size_of::<Option<std::ptr::NonNull<*const c_void>>>(),
        8
    );
    assert_eq!(
        std::mem::size_of::<Option<fn(data: __RegisterData) -> bool>>(),
        8
    )
}
