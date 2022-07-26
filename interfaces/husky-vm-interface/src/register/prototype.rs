use super::*;
use std::ffi::{c_char, CStr, CString};

unsafe impl Sync for __RegisterPrototype {}
unsafe impl Send for __RegisterPrototype {}
#[repr(C)]
pub struct __RegisterPrototype {
    pub type_name: *const c_char,
}
extern "C" {
    pub static __VOID_REGISTER_PROTOTYPE: __RegisterPrototype;
    pub static __BOOL_REGISTER_PROTOTYPE: __RegisterPrototype;
    pub static __I32_REGISTER_PROTOTYPE: __RegisterPrototype;
    pub static __I64_REGISTER_PROTOTYPE: __RegisterPrototype;
    pub static __B32_REGISTER_PROTOTYPE: __RegisterPrototype;
    pub static __B64_REGISTER_PROTOTYPE: __RegisterPrototype;
    pub static __F32_REGISTER_PROTOTYPE: __RegisterPrototype;
    pub static __F64_REGISTER_PROTOTYPE: __RegisterPrototype;
}
