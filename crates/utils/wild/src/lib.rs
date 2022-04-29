pub unsafe fn ref_to_mut_ref<T>(r: &T) {
    let ptr:*T = r;
    let mut_ptr = ptr as *mut T;
    unsafe {&mut *mut_ptr}
}
