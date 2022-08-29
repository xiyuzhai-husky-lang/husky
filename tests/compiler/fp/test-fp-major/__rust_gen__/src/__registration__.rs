use crate::*;
pub(crate) use __husky::registration::*;

type ThickFp__i32_i32 = ThickFp<fn(i32) -> i32>;

// ThickFp__i32_i32
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __thick_fp_i_32_i_32_clone(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    Box::<ThickFp__i32_i32>::into_raw(Box::new((*(data as *mut ThickFp__i32_i32)).clone())) as *mut std::ffi::c_void
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __thick_fp_i_32_i_32_drop(data: *mut std::ffi::c_void) {
    drop(Box::from_raw(data as *mut ThickFp__i32_i32))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __thick_fp_i_32_i_32_eq(this: &std::ffi::c_void, other: &std::ffi::c_void) -> bool {
    *(this as *const std::ffi::c_void as *const ThickFp__i32_i32) == *(other as *const std::ffi::c_void as *const ThickFp__i32_i32)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __thick_fp_i_32_i_32_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<ThickFp__i32_i32>(&__THICK_FP_I_32_I_32_VTABLE) = registers[1].downcast_move(&__THICK_FP_I_32_I_32_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __THICK_FP_I_32_I_32_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_ref_to_bool: None,
    primitive_value_to_box: None,
    clone: __thick_fp_i_32_i_32_clone,
    drop: __thick_fp_i_32_i_32_drop,
    eq: __thick_fp_i_32_i_32_eq,
    assign: __thick_fp_i_32_i_32_assign,
    typename_str_hash_u64: 4753418850164497511,
    typename_str: "ThickFp__i32_i32",
};
