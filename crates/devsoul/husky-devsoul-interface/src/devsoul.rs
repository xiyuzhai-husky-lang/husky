use crate::{DevEvalContext, IsLinketImpl};

pub trait IsDevsoulInterface {
    type LinketImpl: IsLinketImpl;

    fn set_eval_context(ctx: DevEvalContext<Self::LinketImpl>);

    fn eval_context() -> DevEvalContext<Self::LinketImpl>;
}
