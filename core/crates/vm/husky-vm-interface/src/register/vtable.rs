use super::*;
use std::ffi::c_char;

// order matters!
#[repr(C)]
pub struct __RegisterTyVTable {
    pub primitive_value_to_bool: Option<unsafe extern "C" fn(data: __RegisterData) -> bool>,
    pub primitive_value_to_box: Option<unsafe extern "C" fn(data: __RegisterData) -> *mut ()>,
    pub clone: unsafe extern "C" fn(data: *mut ()) -> *mut (),
    pub drop: unsafe extern "C" fn(data: *mut ()),
    pub eq: unsafe extern "C" fn(this: &(), this: &()) -> bool,
    pub assign: unsafe extern "C" fn(args: *mut __Register),
    pub typename_str_hash_u64: u64,
    pub typename_str: &'static str,
}

unsafe impl Sync for __RegisterTyVTable {}

unsafe impl Send for __RegisterTyVTable {}

impl std::fmt::Debug for __RegisterTyVTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        // f.debug_struct("__RegisterTyVTable")
        //     .field("typename_str", unsafe {
        //         &std::ffi::CStr::from_ptr(self.typename_str)
        //     })
        //     .finish()
    }
}

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
