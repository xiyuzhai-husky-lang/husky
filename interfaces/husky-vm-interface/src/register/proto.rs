use super::*;
use std::ffi::{c_char, CStr, CString};

#[repr(C)]
pub struct __RegisterPrototype {
    pub type_name: *const c_char,
}

unsafe impl Sync for __RegisterPrototype {}
unsafe impl Send for __RegisterPrototype {}

pub static __I32_REGISTER_PROTOTYPE: __RegisterPrototype = __RegisterPrototype {
    type_name: "i32".to_string(),
};

pub static __I64_REGISTER_PROTOTYPE: __RegisterPrototype = __RegisterPrototype {
    type_name: CString::new("i64").unwrap().as_ptr() as *const c_char,
};

pub static __BOOL_REGISTER_PROTOTYPE: __RegisterPrototype = __RegisterPrototype {
    type_name: CString::new("bool").unwrap().as_ptr() as *const c_char,
};

pub static __VOID_REGISTER_PROTOTYPE: __RegisterPrototype = __RegisterPrototype {
    type_name: CString::new("void").unwrap().as_ptr() as *const c_char,
};

pub static __B32_REGISTER_PROTOTYPE: __RegisterPrototype = __RegisterPrototype {
    type_name: CString::new("b32").unwrap().as_ptr() as *const c_char,
};

pub static __B64_REGISTER_PROTOTYPE: __RegisterPrototype = __RegisterPrototype {
    type_name: CString::new("b64").unwrap().as_ptr() as *const c_char,
};

pub static __F32_REGISTER_PROTOTYPE: __RegisterPrototype = __RegisterPrototype {
    type_name: CString::new("f32").unwrap().as_ptr() as *const c_char,
};

pub static __F64_REGISTER_PROTOTYPE: __RegisterPrototype = __RegisterPrototype {
    type_name: CString::new("f64").unwrap().as_ptr() as *const c_char,
};
