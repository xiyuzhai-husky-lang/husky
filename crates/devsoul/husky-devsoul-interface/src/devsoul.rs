use crate::{DevEvalContext, IsLinkageImpl};

pub trait IsDevsoulInterface {
    type LinkageImpl: IsLinkageImpl;

    fn eval_context() -> DevEvalContext<Self::LinkageImpl>;
}
