use crate::{
    eval_context::{DevEvalContext, IsDevRuntimeDyn},
    linket_impl::IsLinketImpl,
};
use husky_item_path_interface::ItemPathIdInterface;

pub struct LinketImpls<LinketImpl: IsLinketImpl> {
    set_dev_eval_context: unsafe fn(DevEvalContext<LinketImpl>),
    unset_dev_eval_context: unsafe fn(),
    init_item_path_id_interface_caches: fn(&[ItemPathIdInterface]),
    linket_impls: Vec<LinketImpl>,
}

impl<LinketImpl: IsLinketImpl> LinketImpls<LinketImpl> {
    pub fn new(
        set_dev_eval_context: unsafe fn(DevEvalContext<LinketImpl>),
        unset_dev_eval_context: unsafe fn(),
        init_item_path_id_interface_caches: fn(&[ItemPathIdInterface]),
        linket_impls: Vec<LinketImpl>,
    ) -> Self {
        Self {
            set_dev_eval_context,
            unset_dev_eval_context,
            init_item_path_id_interface_caches,
            linket_impls,
        }
    }
}

impl<LinketImpl: IsLinketImpl> LinketImpls<LinketImpl> {
    pub fn linket_impls(&self) -> &[LinketImpl] {
        &self.linket_impls
    }

    /// the `&mut self` reflects some change on the otherside
    pub unsafe fn set_dev_eval_context(
        &mut self,
        runtime: &'static dyn IsDevRuntimeDyn<LinketImpl>,
    ) {
        (self.set_dev_eval_context)(DevEvalContext::new(runtime))
    }

    pub unsafe fn unset_dev_eval_context(&mut self) {
        (self.unset_dev_eval_context)()
    }
}
