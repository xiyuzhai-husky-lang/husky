use crate::*;
pub(crate) use __husky::registration::*;

type Vec__Option__i32 = Vec<Option<i32>>;

// Vec__Option__i32
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_option_i_32_clone(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    Box::<Vec__Option__i32>::into_raw(Box::new((*(data as *mut Vec__Option__i32)).clone())) as *mut std::ffi::c_void
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_option_i_32_drop(data: *mut std::ffi::c_void) {
    drop(Box::from_raw(data as *mut Vec__Option__i32))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_option_i_32_eq(this: &std::ffi::c_void, other: &std::ffi::c_void) -> bool {
    *(this as *const std::ffi::c_void as *const Vec__Option__i32) == *(other as *const std::ffi::c_void as *const Vec__Option__i32)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __vec_option_i_32_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<Vec__Option__i32>(&__VEC_OPTION_I_32_VTABLE) = registers[1].downcast_move(&__VEC_OPTION_I_32_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __VEC_OPTION_I_32_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_ref_to_bool: None,
    primitive_value_to_box: None,
    clone: __vec_option_i_32_clone,
    drop: __vec_option_i_32_drop,
    eq: __vec_option_i_32_eq,
    assign: __vec_option_i_32_assign,
    typename_str_hash_u64: 5114766044509264115,
    typename_str: "Vec__Option__i32",
};
