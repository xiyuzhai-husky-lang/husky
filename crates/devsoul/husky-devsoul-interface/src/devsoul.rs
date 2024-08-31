use husky_linket_impl::{
    eval_context::{DevEvalContext, DevEvalContextGuard},
    linket_impl::IsLinketImpl,
};

pub trait IsDevsoulInterface {
    type LinketImpl: IsLinketImpl;

    fn try_set_dev_eval_context(
        ctx: DevEvalContext<Self::LinketImpl>,
    ) -> Result<DevEvalContextGuard, ()>;
    fn dev_eval_context() -> DevEvalContext<Self::LinketImpl>;
}
