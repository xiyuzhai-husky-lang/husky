use crate::{DevEvalContext, IsLinketImpl};

pub trait IsDevsoulInterface {
    type LinketImpl: IsLinketImpl;

    fn eval_context() -> DevEvalContext<Self::LinketImpl>;
}
