pub trait Upcast<T: ?Sized> {
    fn upcast(&self) -> &T;
}
pub trait UpcastMut<T: ?Sized> {
    fn upcast_mut(&mut self) -> &mut T;
}
