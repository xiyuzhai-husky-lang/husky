pub unsafe fn arb_lifetime_mut<'a, T>(r: &mut T) -> &'a mut T {
    let ptr: *const T = r;
    let mut_ptr = ptr as *mut T;
    &mut *mut_ptr
}
