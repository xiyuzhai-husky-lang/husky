pub trait Is<T>: From<T> + Into<T> {}

impl<T> Is<T> for T {}

pub trait IsSized<T>: From<T> + Into<T>
where
    T: Sized,
{
    fn cast_as_ref(&self) -> &T;
}

impl<T: Sized> IsSized<T> for T {
    fn cast_as_ref(&self) -> &T {
        self
    }
}
