pub unsafe fn arb_mut<'a, T: ?Sized>(r: &mut T) -> &'a mut T {
    let ptr: *mut T = r;
    &mut *ptr
}

pub unsafe fn arb_ref<'a, T: ?Sized>(r: &T) -> &'a T {
    let ptr: *const T = r;
    &*ptr
}
