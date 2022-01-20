use std::fmt::Debug;

use common::*;

pub trait HasRef {
    fn has_ref(&self) -> bool;
}

pub trait Any: std::any::Any + Debug {
    fn clone_any(&self) -> Box<dyn Any>;
}

impl<T: Clone + 'static + HasRef + Debug> Any for T {
    fn clone_any(&self) -> Box<dyn Any> {
        should!(!self.has_ref());
        Box::new(self.clone())
    }
}

impl HasRef for i32 {
    fn has_ref(&self) -> bool {
        false
    }
}

impl HasRef for f32 {
    fn has_ref(&self) -> bool {
        false
    }
}

impl HasRef for () {
    fn has_ref(&self) -> bool {
        false
    }
}

impl<T: HasRef> HasRef for Vec<T> {
    fn has_ref(&self) -> bool {
        self.iter().all(|x| x.has_ref())
    }
}
