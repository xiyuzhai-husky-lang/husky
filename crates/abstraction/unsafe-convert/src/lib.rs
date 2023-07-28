pub unsafe trait UnsafeFrom<T> {
    unsafe fn unsafe_from(t: T) -> Self;
}

pub unsafe trait UnsafeInto<T>: Sized {
    unsafe fn unsafe_into(self) -> T;
}

unsafe impl<S, T> UnsafeInto<T> for S
where
    T: UnsafeFrom<S>,
{
    unsafe fn unsafe_into(self) -> T {
        T::unsafe_from(self)
    }
}
