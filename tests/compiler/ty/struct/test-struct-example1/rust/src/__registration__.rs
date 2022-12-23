use crate::*;
pub(crate) use __husky::registration::*;

type A = crate::A;

// A
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __a_clone(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    Box::<A>::into_raw(Box::new((*(data as *mut A)).clone())) as *mut std::ffi::c_void
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __a_drop(data: *mut std::ffi::c_void) {
    drop(Box::from_raw(data as *mut A))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __a_eq(this: &std::ffi::c_void, other: &std::ffi::c_void) -> bool {
    *(this as *const std::ffi::c_void as *const A) == *(other as *const std::ffi::c_void as *const A)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __a_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<A>(&__A_VTABLE) = registers[1].downcast_move(&__A_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __A_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_ref_to_bool: None,
    primitive_value_to_box: None,
    clone: __a_clone,
    drop: __a_drop,
    eq: __a_eq,
    assign: __a_assign,
    typename_str_hash_u64: 15047818145317598341,
    typename_str: "A",
};
