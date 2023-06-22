#[test]
fn arc_stack_overflow() {
    const N: usize = 420480;
    type T = [[[[[[i32; 20]; 10]; 10]; 10]; 10]; 10];
    unsafe {
        let a: Box<T> = Box::from_raw(std::alloc::alloc(std::alloc::Layout::new::<T>()) as *mut T);
        let a: std::sync::Arc<T> = a.into();
    }
}

struct V<T> {
    t: T,
}

impl<T> V<std::sync::Arc<T>> {
    fn a() {}
}

impl<T> V<Vec<T>> {
    fn a() {}
}

impl<T> V<[T; 2]> {
    fn a() {}
}
