use super::*;
use std::ffi::{c_char, CStr, CString};

unsafe impl Sync for __RegisterVTable {}
unsafe impl Send for __RegisterVTable {}
#[repr(C)]
pub struct __RegisterVTable {
    pub typename_str: *const c_char,
    pub primitive_value_to_bool: Option<fn(data: __RegisterData) -> bool>,
    pub primitive_value_to_box: Option<fn(data: *mut __RegisterData) -> *mut ()>,
    pub drop: Option<fn(data: *mut ())>
}
extern "C" {
    pub static __VOID_VTABLE: __RegisterVTable;
    pub static __BOOL_VTABLE: __RegisterVTable;
    pub static __I32_VTABLE: __RegisterVTable;
    pub static __I64_VTABLE: __RegisterVTable;
    pub static __B32_VTABLE: __RegisterVTable;
    pub static __B64_VTABLE: __RegisterVTable;
    pub static __F32_VTABLE: __RegisterVTable;
    pub static __F64_VTABLE: __RegisterVTable;
}
