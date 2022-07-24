use super::*;
use std::panic::{RefUnwindSafe, UnwindSafe};

#[derive(Debug, Clone, Copy)]
pub struct __ModelLinkage(pub &'static dyn ModelDyn);

impl std::ops::Deref for __ModelLinkage {
    type Target = dyn ModelDyn;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl PartialEq for __ModelLinkage {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const dyn ModelDyn as *const () as usize
            == other.0 as *const dyn ModelDyn as *const () as usize
    }
}

impl Eq for __ModelLinkage {}

pub trait ModelDyn: std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe {
    fn train_dyn(&self, opds: Vec<(Vec<__Register>, __Register)>) -> __VMResult<__Register>;
    fn eval_dyn<'eval>(
        &self,
        internal: &__Register,
        arguments: &[__Register],
    ) -> __VMResult<__Register>;
}

pub trait Model:
    std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + Sized + 'static
{
    type Internal: __Registrable;
    fn train(&self, opds: Vec<(Vec<__Register>, __Register)>) -> __VMResult<__Register>;
    fn eval<'eval>(
        &self,
        internal: &Self::Internal,
        arguments: &[__Register],
    ) -> __VMResult<__Register>;
}

impl<T: Model> ModelDyn for T {
    fn train_dyn(&self, opds: Vec<(Vec<__Register>, __Register)>) -> __VMResult<__Register> {
        Ok(self.train(opds)?)
    }

    fn eval_dyn<'eval>(
        &self,
        internal: &__Register,
        arguments: &[__Register],
    ) -> __VMResult<__Register> {
        let internal: &T::Internal = unsafe { internal.downcast_temp_ref() };
        self.eval(internal, arguments)
    }
}
