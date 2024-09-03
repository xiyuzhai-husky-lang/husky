use crate::thawed::{Thawed, ThawedDyn};

pub trait __Regular: std::fmt::Debug {
    type Thawed: Thawed;
}

impl<T> __Regular for &mut T
where
    T: __Regular,
{
    type Thawed = *mut <T as __Regular>::Thawed;
}

impl PartialEq for Box<dyn ThawedDyn> {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
        // self.0 == other.0 && self.1 == other.1
    }
}

impl Clone for Box<dyn ThawedDyn> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Eq for Box<dyn ThawedDyn> {}

impl PartialEq for &'static dyn ThawedDyn {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl Eq for &'static dyn ThawedDyn {}
