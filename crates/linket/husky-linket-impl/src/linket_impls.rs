use crate::{
    eval_context::{DevEvalContext, DevEvalContextGuard, IsDevRuntimeDyn},
    linket_impl::IsLinketImpl,
};
use husky_item_path_interface::ItemPathIdInterface;

pub struct LinketImpls<LinketImpl: IsLinketImpl> {
    try_set_dev_eval_context:
        unsafe fn(DevEvalContext<LinketImpl>) -> Result<DevEvalContextGuard, ()>,
    linket_impls: Vec<LinketImpl>,
}

impl<LinketImpl: IsLinketImpl> LinketImpls<LinketImpl> {
    pub fn new(
        try_set_dev_eval_context: unsafe fn(
            DevEvalContext<LinketImpl>,
        ) -> Result<DevEvalContextGuard, ()>,
        linket_impls: Vec<LinketImpl>,
    ) -> Self {
        Self {
            try_set_dev_eval_context,
            linket_impls,
        }
    }
}

impl<LinketImpl: IsLinketImpl> LinketImpls<LinketImpl> {
    pub fn linket_impls(&self) -> &[LinketImpl] {
        &self.linket_impls
    }

    /// the `&mut self` reflects some change on the otherside
    pub unsafe fn try_set_dev_eval_context(
        &self,
        runtime: &'static dyn IsDevRuntimeDyn<LinketImpl>,
    ) -> Result<DevEvalContextGuard, ()> {
        (self.try_set_dev_eval_context)(DevEvalContext::new(runtime))
    }
}
