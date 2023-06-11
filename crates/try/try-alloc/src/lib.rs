#[test]
fn arc_stack_overflow() {
    const N: usize = 420480;
    type T = [[[[[[i32; 20]; 10]; 10]; 10]; 10]; 10];
    unsafe {
        let a: Box<T> = Box::from_raw(std::alloc::alloc(std::alloc::Layout::new::<T>()) as *mut T);
        let a: std::sync::Arc<T> = a.into();
    }
}
