use check_utils::*;

use crate::*;

use super::*;

#[derive(Debug, Clone)]
pub struct SharedValue<'eval>(Arc<dyn AnyValueDyn<'eval>>);

impl<'eval> PartialEq for SharedValue<'eval> {
    fn eq(&self, other: &SharedValue<'eval>) -> bool {
        self.0.equal_any(&*other.0)
    }
}

impl<'eval> Eq for SharedValue<'eval> {}

impl<'eval> SharedValue<'eval> {
    pub fn new<T: AnyValueDyn<'eval>>(value: T) -> SharedValue<'eval> {
        Self(Arc::new(value))
    }

    pub fn clone_from(value: &dyn AnyValueDyn<'eval>) -> SharedValue<'eval> {
        Self(value.clone_into_arc_dyn())
    }

    pub fn any_ptr(&self) -> *const (dyn AnyValueDyn<'eval>) {
        &*(self.0)
    }

    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        &*self.0
    }

    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        self.0.downcast_ref()
    }

    pub fn share(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        todo!()
    }
}
