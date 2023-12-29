use crate::{val_control_flow::ValControlFlow, val_repr::ValReprInterface, value::IsValue};
use crate::{val_repr::ValArgumentReprInterface, DevEvalContext};

pub trait IsLinkageImpl: Send + Copy + 'static {
    type Pedestal: Copy + 'static;
    type Value: IsValue;
    type Error: std::fmt::Debug;

    /// assumed that pedestal has already been
    fn eval(
        self,
        val_repr_interface: ValReprInterface,
        ctx: DevEvalContext<Self>,
        arguments: &[ValArgumentReprInterface],
    ) -> LinkageImplValControlFlow<Self>;
}

pub type LinkageImplValControlFlow<
    LinkageImpl: IsLinkageImpl,
    C = <LinkageImpl as IsLinkageImpl>::Value,
> = ValControlFlow<C, LinkageImpl::Value, LinkageImpl::Error>;
