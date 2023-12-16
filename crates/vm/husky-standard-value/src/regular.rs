use crate::{
    r#static::{Static, StaticDyn},
    *,
};

pub trait __Regular: std::fmt::Debug {
    type Static: Static;
}

impl<T> __Regular for &mut T
where
    T: __Regular,
{
    type Static = *mut <T as __Regular>::Static;
}

impl PartialEq for Box<dyn StaticDyn> {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
        // self.0 == other.0 && self.1 == other.1
    }
}

impl Clone for Box<dyn StaticDyn> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Eq for Box<dyn StaticDyn> {}

impl PartialEq for &'static dyn StaticDyn {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl Eq for &'static dyn StaticDyn {}
