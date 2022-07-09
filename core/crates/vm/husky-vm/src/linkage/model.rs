use std::panic::{RefUnwindSafe, UnwindSafe};

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ModelLinkage(pub &'static dyn ModelDyn);

impl std::ops::Deref for ModelLinkage {
    type Target = dyn ModelDyn;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl PartialEq for ModelLinkage {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const dyn ModelDyn as *const () as usize
            == other.0 as *const dyn ModelDyn as *const () as usize
    }
}

impl Eq for ModelLinkage {}

pub trait ModelDyn: std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe {
    fn train_dyn(&self, opds: Vec<(Vec<__EvalValue<'static>>, Label)>) -> __EvalResult;
    fn eval_dyn<'eval>(
        &self,
        internal: &__EvalValue<'static>,
        arguments: &[__EvalValue<'eval>],
    ) -> EvalValueResult<'eval>;
}

pub trait Model:
    std::fmt::Debug + Send + Sync + RefUnwindSafe + UnwindSafe + Sized + 'static
{
    type Internal: __AnyValue<'static>;
    fn train(&self, opds: Vec<(Vec<__EvalValue<'static>>, Label)>) -> __EvalResult<Self::Internal>;
    fn eval<'eval>(
        &self,
        internal: &Self::Internal,
        arguments: &[__EvalValue<'eval>],
    ) -> EvalValueResult<'eval>;
}

impl<T: Model> ModelDyn for T {
    fn train_dyn(&self, opds: Vec<(Vec<__EvalValue<'static>>, Label)>) -> __EvalResult {
        Ok(self.train(opds)?.__into_eval_value())
    }

    fn eval_dyn<'eval>(
        &self,
        internal: &__EvalValue<'static>,
        arguments: &[__EvalValue<'eval>],
    ) -> EvalValueResult<'eval> {
        let internal: &T::Internal = internal.any_ref().__downcast_ref();
        self.eval(internal, arguments)
    }
}
