pub unsafe fn ref_to_mut_ref<T>(r: &T) -> &mut T {
    let ptr: *const T = r;
    let mut_ptr = ptr as *mut T;
    &mut *mut_ptr
}

pub unsafe fn arb_ref<'a, T>(r: &T) -> &'a mut T {
    let ptr: *const T = r;
    let mut_ptr = ptr as *mut T;
    &mut *mut_ptr
}
