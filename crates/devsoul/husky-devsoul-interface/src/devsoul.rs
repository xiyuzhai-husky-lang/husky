use crate::{DevEvalContext, IsLinketImpl};

pub trait IsDevsoulInterface {
    type LinketImpl: IsLinketImpl;

    fn set_dev_eval_context(ctx: DevEvalContext<Self::LinketImpl>);
    fn unset_dev_eval_context();

    fn dev_eval_context() -> DevEvalContext<Self::LinketImpl>;
}
